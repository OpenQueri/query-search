use std::error::Error;
use std::path::Path;
use std::sync::{OnceLock, RwLock};
use std::fs;

use bincode;
use dashmap::DashMap;
use hnsw_rs::hnswio;
use hnsw_rs::prelude::*;
use sled::Db;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

use crate::engine::other::Other;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MetaData {
    pub url: String,
    pub title: String,
    pub image: Vec<String>,
}

pub static ORIGINAL_SITE: OnceLock<DashMap<u64, MetaData>> = OnceLock::new();
pub static SEARCH_INDEX: OnceLock<RwLock<Hnsw<'static, f32, DistCosine>>> = OnceLock::new();

fn get_original_sites() -> &'static DashMap<u64, MetaData> {
    ORIGINAL_SITE.get_or_init(DashMap::new)
}

pub struct SearchIndex;

impl SearchIndex {
    pub const HNSW_DIR: &str = "search_engine_graph/hnsw";
    pub const HNSW_BASE_NAME: &str = "hnsw_index";

    pub fn get_index() -> &'static RwLock<Hnsw<'static, f32, DistCosine>> {
        SEARCH_INDEX.get_or_init(|| {
            println!("ℹ️ Ініціалізація базового HNSW індексу в пам'яті...");
            let max_nb_connection = 32;
            let num_elements = 50_000;
            let max_layer = 16;
            let ef_construction = 300;

            let hnsw = Hnsw::<f32, DistCosine>::new(
                max_nb_connection,
                num_elements,
                max_layer,
                ef_construction,
                DistCosine {},
            );
            RwLock::new(hnsw)
        })
    }
}

pub struct EngineSearch;

impl EngineSearch {
    pub async fn engine_search(query: &[f32], k: usize) -> Result<Vec<MetaData>, Box<dyn Error + Send + Sync>> {
        let index_guard = SearchIndex::get_index()
            .read()
            .map_err(|_| "Не вдалося заблокувати HNSW")?;

        let neighbors = index_guard.search(query, k , 64);
        let sites = get_original_sites();

        const MAX_L2_DISTANCE: f32 = 0.25;

        struct SiteScore {
            meta: MetaData,
            count: usize,
            min_distance: f32,
        }

        let mut scored_sites: HashMap<u64, SiteScore> = HashMap::new();

        for neighbor in neighbors {
            let distance = neighbor.distance;

            println!("distance: {}", distance);

            if distance > MAX_L2_DISTANCE {
                continue;
            }

            let site_id = neighbor.d_id as u64;

            if let Some(url_entry) = sites.get(&site_id) {

                let entry = scored_sites.entry(site_id).or_insert_with(|| SiteScore {
                    meta: url_entry.value().clone(),
                    count: 0,
                    min_distance: distance,
                });

                entry.count += 1;
                if distance < entry.min_distance {
                    entry.min_distance = distance;
                }
            }
        }

        let mut final_results: Vec<SiteScore> = scored_sites.into_values().collect();

        final_results.sort_by(|a, b| {
            let count_cmp = b.count.cmp(&a.count);
            if count_cmp != std::cmp::Ordering::Equal {
                count_cmp
            } else {
                a.min_distance.partial_cmp(&b.min_distance).unwrap_or(std::cmp::Ordering::Equal)
            }
        });

        let trimmed_results: Vec<MetaData> = final_results
            .into_iter()
            .take(k)
            .map(|item| item.meta)
            .collect();

        Ok(trimmed_results)
    }
}

pub struct EngineEdit;

impl EngineEdit {
    pub async fn engine_insert(tensor: &[f32], data: MetaData) -> Result<(), Box<dyn Error + Send + Sync>> {
        let id = Other::hash_u64(&data.url).await;

        get_original_sites().insert(id, data);

        {
            let mut index_guard = SearchIndex::get_index()
                .write()
                .map_err(|_| "Не вдалося заблокувати HNSW для запису")?;
            index_guard.insert((&tensor, id as usize));
        }

        SaveLoadData::save_everything().await?;

        Ok(())
    }
}

pub static SLED_DB: OnceLock<Db> = OnceLock::new();

pub struct SaveLoadData;

impl SaveLoadData {
    const SLED_DB_DIR: &str = "search_engine_graph/sled"; 
    const TREE_SITES: &str = "original_sites";

    fn open_db() -> Result<Db, Box<dyn Error + Send + Sync>> {
        let db = SLED_DB.get_or_init(|| {
            if let Some(parent) = Path::new(Self::SLED_DB_DIR).parent() {
                let _ = fs::create_dir_all(parent);
            }
            sled::open(Self::SLED_DB_DIR).expect("Не вдалося відкрити Sled DB")
        });
        Ok(db.clone())
    }

    pub async fn save_everything() -> Result<(), Box<dyn Error + Send + Sync>> {
        let db = Self::open_db()?;
        let tree_sites = db.open_tree(Self::TREE_SITES)?;

        println!("💾 Збереження мапи сайтів у Sled...");
        let sites = get_original_sites();
        for entry in sites.iter() {
            let key = entry.key().to_be_bytes();
            let val = bincode::serialize(entry.value())?;
            tree_sites.insert(key, val)?;
        }
        db.flush_async().await?;

        println!("💾 Збереження HNSW графу у бінарні файли через hnswio...");
        {
            let index_guard = SearchIndex::get_index()
                .read()
                .map_err(|_| "Помилка блокування графу під час сейву")?;
            
            fs::create_dir_all(SearchIndex::HNSW_DIR)?;
            
            let path = Path::new(SearchIndex::HNSW_DIR);
            index_guard.file_dump(path, SearchIndex::HNSW_BASE_NAME)?;
        }

        println!("✨ Усі дані успішно синхронізовано з диском!");
        Ok(())
    }

pub async fn load_everything() -> Result<(), Box<dyn Error + Send + Sync>> {
        if !Path::new(Self::SLED_DB_DIR).exists() {
            println!("ℹ️ База даних Sled ще не створена. Пропускаю завантаження.");
            return Ok(());
        }

        let db = Self::open_db()?;
        let tree_sites = db.open_tree(Self::TREE_SITES)?;
        let sites = get_original_sites();

        let mut sites_count = 0;
        for item in tree_sites.iter() {
            let (k, v) = item?;
            let id = u64::from_be_bytes(k.as_ref().try_into()?);
            let meta_data: MetaData = bincode::deserialize(&v)?;
            sites.insert(id, meta_data);
            sites_count += 1;
        }
        println!("🚀 Sled: завантажено {} адрес сайтів (ORIGINAL_SITE)", sites_count);

        println!("📦 Спроба відновлення графу HNSW з папки {}...", SearchIndex::HNSW_DIR);
        let io_handler = hnswio::HnswIo::new(Path::new(SearchIndex::HNSW_DIR), SearchIndex::HNSW_BASE_NAME);
        let io_handler_static: &'static mut hnswio::HnswIo = Box::leak(Box::new(io_handler));
        
        match io_handler_static.load_hnsw::<f32, DistCosine>() {
            Ok(loaded_hnsw) => {
                if let Err(lock_err) = SEARCH_INDEX.set(RwLock::new(loaded_hnsw)) {
                    let graph_to_move = lock_err.into_inner();
                    
                    if let Ok(mut current_index) = SearchIndex::get_index().write() {
                        *current_index = graph_to_move?;
                    }
                }
                println!("✅ ГРАФ HNSW УСПІШНО ВІДНОВЛЕНО З ДИСКА В ПАМ'ЯТЬ!");
            }
            Err(e) => {
                println!("ℹ️ Існуючий граф HNSW не знайдено ({:?}). Використовуємо чистий індекс.", e);
                let _ = SearchIndex::get_index(); 
            }
        }

        Ok(())
    }
}