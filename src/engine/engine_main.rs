use std::{error::Error, sync::RwLock, sync::Arc};
use dashmap::DashMap;
use indexmap::IndexMap;
use once_cell::sync::{Lazy};
use crate::Request;
use crate::engine::other::*;
use serde::Serialize;

// Global storage: hash → full URL string
// IndexMap preserves insertion order
// RwLock + Arc for safe concurrent access (mostly reads, rare writes)
static ALL_LINKS: Lazy<Arc<RwLock<IndexMap<u64, String>>>> = Lazy::new(||{Arc::new(RwLock::new(IndexMap::new()))});

// Inverted index: word → list of document hashes (u64)
// DashMap chosen for concurrent reads/writes
static LINK_DATA: Lazy<DashMap<String, Vec<u64>>> = Lazy::new(||{DashMap::new()});

#[derive(Debug, Clone, Serialize)] 
pub struct Response{
    link: String,
    frequency: usize,
}

pub struct EngineSearch;

impl EngineSearch{
    // Currently takes Vec<(word, weight)>, but weight is not used yet
    // Simple term frequency sum (no tf-idf, no normalization)
    pub async fn engine_search(request_text: Vec<String>) -> Result<Vec<Response>, Box<dyn Error>>{
        let all_links_map = Arc::clone(&ALL_LINKS);


       
                // Accumulate document → match count
                let mut map_index_site:IndexMap<usize, usize> = IndexMap::new();

                // For each word in the query
                for (word) in request_text.iter(){
                    // Get posting list for this word
                    if let Some(val) = LINK_DATA.get(word){
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
                    if let Some(get_link) = all_links_map.read().unwrap().get(&hesh){
                        response.push(Response {
                            link: get_link.clone(),
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
    pub async fn engine_write(link: &str, request: &Request<'_>) -> Result<(), Box<dyn Error>>{
        
        let all_links_map = Arc::clone(&ALL_LINKS);
        let worlds = &request.words;

        let site = AllFrequencySite;

        // Hash of the URL (used as document ID)
        let hesh = AllFrequencySite.calculate_hash(link)?;

        // Fast path: check if already exists (read lock)
        let read_guard = all_links_map.read().unwrap();
        
        if let Some(_) = read_guard.get_index_of(&hesh) {
            drop(read_guard); // звільняємо read lock
            
            for (i, word) in worlds.iter().enumerate() {
                LINK_DATA.entry(word.to_string()).or_insert(Vec::new()).push(hesh);
            }
        } else {
            drop(read_guard); // звільняємо read lock перед write
            
            let mut writetable_map = all_links_map.write().unwrap();

            // Re-compute hash (just in case)
            let gen_num_id = site.calculate_hash(link)?;

            // Insert URL and get its position (though position not used here)
            let (idx, _) = writetable_map.insert_full(gen_num_id, link.to_string());

            // Add all words to inverted index
            for (i, word) in worlds.iter().enumerate() {
                LINK_DATA.entry(word.to_string()).or_insert(Vec::new()).push(gen_num_id);
            }
        };

        Ok(())
    }
}