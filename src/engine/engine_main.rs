use std::{error::Error};
use dashmap::DashMap;
use indexmap::IndexMap;
use once_cell::sync::{Lazy};
use crate::Request;
use crate::engine::other::*;
use serde::{Serialize, Deserialize};


// Global storage: hash → full struct URL string + title string
// IndexMap preserves insertion order
// RwLock + Arc for safe concurrent access (mostly reads, rare writes)
#[derive(Serialize, Deserialize, Clone)]
struct ContentURL{
    url: String,
    title: String,
}
static ALL_LINKS: Lazy<DashMap<u64, ContentURL>> = Lazy::new(||{DashMap::new()});


// Inverted index: word → list of document hashes (u64)
// DashMap chosen for concurrent reads/writes
static LINK_DATA: Lazy<DashMap<u64, Vec<u64>>> = Lazy::new(||{DashMap::new()});

#[derive(Debug, Clone, Serialize)] 
pub struct Response{
    title: String,
    link: String,
    frequency: usize,
}

pub struct EngineSearch;

impl EngineSearch{
    // Currently takes Vec<(word, weight)>, but weight is not used yet
    // Simple term frequency sum (no tf-idf, no normalization)
    pub async fn engine_search(request_text: Vec<String>) -> Result<Vec<Response>, Box<dyn Error>>{
       
                // Accumulate document → match count
                let mut map_index_site:IndexMap<usize, usize> = IndexMap::new();

                //
                let site = AllFrequencySite;

                // For each word in the query
                for word in request_text.iter(){
                    // Get posting list for this word
                    if let Some(val) = LINK_DATA.get(&site.calculate_hash(word.as_str())?){
                        // Increment counter for every document that contains the word
                        val.iter().for_each(|iw| {
                            *map_index_site.entry((*iw).try_into().unwrap()).or_insert(0) += 1;
                        });
                    }
                }


                let mut response: Vec<Response> = Vec::new();

                // Build result list
                for (site_idx, site_frequency) in map_index_site.iter(){
                    let hesh = site_idx.clone() as u64;

                    // Lookup URL by hash
                    if let Some(get_link) = ALL_LINKS.get(&hesh){
                        response.push(Response {
                            link: get_link.url.clone(),
                            title: get_link.title.clone(),
                            frequency: site_frequency.clone(),
                        });
                    };
                }


        Ok(response)
    }
}

pub struct EngineEdit;

impl EngineEdit {
    // Add or update document in the index
    // If URL already exists → just append words to posting lists
    // If not → register new URL and add words
    pub async fn engine_write(title: &str, link: &str, request: &Request<'_>) -> Result<(), Box<dyn Error>>{
        
        let worlds = &request.words;

        let site = AllFrequencySite;


        // Hash of the URL (used as document ID)
        let hesh = AllFrequencySite.calculate_hash(&link)?;

        // Fast path: check if already exists (read lock)
        //get_index_of(&hesh)
        if let Some(_) = ALL_LINKS.get(&hesh) {
            
            for (i, word) in worlds.iter().enumerate() {
                LINK_DATA.entry(site.calculate_hash(word.as_str())?).or_insert(Vec::new()).push(hesh);
            }
            
        } else {
            

            // Re-compute hash (just in case)
            let gen_num_id = site.calculate_hash(&link)?;


            // Insert URL and get its position (though position not used here)
            ALL_LINKS.insert(gen_num_id, ContentURL{
                url: link.to_string(),
                title: title.to_string(),
            });

            // Add all words to inverted index
            for (_, word) in worlds.iter().enumerate() {
                LINK_DATA.entry(site.calculate_hash(word.as_str())?).or_insert(Vec::new()).push(gen_num_id);
                SaveLoadData::save_links().await?;
            }
        };

        Ok(())
    }
}



use std::fs;
use bincode;
use sled::{Db, Tree};

pub struct SaveLoadData;

impl SaveLoadData {
    // Тепер це не просто файли, а назва папки з базою
    const DB_NAME: &str = "search_engine_db";
    
    // Окремі дерева (як таблиці) всередині Sled
    const TREE_LINKS: &str = "links";
    const TREE_DATA: &str = "link_data";

    /// Відкриває базу даних Sled
    fn open_db() -> Result<Db, Box<dyn Error>> {
        let db = sled::open(Self::DB_NAME)?;
        Ok(db)
    }

    /// Зберігає поточний стан ALL_LINKS та LINK_DATA в Sled
    /// У Sled це працює як дозапис (upsert)
    pub async fn save_links() -> Result<(), Box<dyn Error>> {
        let db = Self::open_db()?;
        
        let tree_links = db.open_tree(Self::TREE_LINKS)?;
        let tree_data = db.open_tree(Self::TREE_DATA)?;

        // Зберігаємо посилання з ALL_LINKS
        for entry in ALL_LINKS.iter() {
            let key = entry.key().to_be_bytes();
            let val = bincode::serialize(entry.value())?;
            tree_links.insert(key, val)?;
        }

        // Зберігаємо дані індексу з LINK_DATA
        for entry in LINK_DATA.iter() {
            let key = entry.key().to_be_bytes();
            let val = bincode::serialize(entry.value())?;
            tree_data.insert(key, val)?;
        }

        // Чекаємо, поки дані фізично запишуться на диск
        db.flush_async().await?;
        
        Ok(())
    }

    /// Завантажує все з Sled у DashMap (ALL_LINKS та LINK_DATA)
    pub async fn load_everything() -> Result<(), Box<dyn Error>> {
        // Якщо папки бази немає, то й вантажити нічого
        if !std::path::Path::new(Self::DB_NAME).exists() {
            println!("База даних {} ще не створена. Пропускаю.", Self::DB_NAME);
            return Ok(());
        }

        let db = Self::open_db()?;
        let tree_links = db.open_tree(Self::TREE_LINKS)?;
        let tree_data = db.open_tree(Self::TREE_DATA)?;

        // Вантажимо посилання
        let mut links_count = 0;
        for item in tree_links.iter() {
            let (k, v) = item?;
            let hash = u64::from_be_bytes(k.as_ref().try_into()?);
            let content: ContentURL = bincode::deserialize(&v)?;
            ALL_LINKS.insert(hash, content);
            links_count += 1;
        }

        // Вантажимо індекс слів
        let mut data_count = 0;
        for item in tree_data.iter() {
            let (k, v) = item?;
            let word_hash = u64::from_be_bytes(k.as_ref().try_into()?);
            let site_hashes: Vec<u64> = bincode::deserialize(&v)?;
            LINK_DATA.insert(word_hash, site_hashes);
            data_count += 1;
        }

        println!("Sled: завантажено {} посилань та {} індексів", links_count, data_count);
        
        Ok(())
    }
}