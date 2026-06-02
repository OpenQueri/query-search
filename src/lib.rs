use std::{error::Error, time::Duration};
mod extract;
mod onnx;
mod engine;
use serde::{Deserialize,Serialize};
use std::time::Instant;
use crawler_engine::DataSiteResponse;
use onnx::onnx::OnnxEmbeddingEngine;
use crate::extract::extract_sentences::extract_chunks;
use tokio::task;
use crate::engine::engine_main::{EngineEdit,EngineSearch,MetaData,SaveLoadData,SearchIndex};
use tokio::sync::Semaphore;
use std::sync::Arc;
use futures::stream::{self, StreamExt}; 

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MetaSite {
    pub url: String,
    pub title: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct QuerySite {
    pub meta_data: Vec<MetaSite>,
    pub duration: Duration,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct QueryImage {
    pub meta_data: Vec<MetaData>,
    pub duration: Duration,
}


pub async fn search_site_data(text: &str) -> Result<QuerySite, Box<dyn Error + Send + Sync>> {
    let start_time = Instant::now();

    let text_owned = text.to_string();

    let raw_embedding = task::spawn_blocking(move || {
        let onnx = OnnxEmbeddingEngine::global();
        onnx.get_raw_embedding_query(&text_owned)
    })
    .await?
    ?;     

    let limit = 30;
    let result_search = EngineSearch::engine_search(&raw_embedding, limit).await?;

    let duration = start_time.elapsed();


    let meta_site_vec: Vec<MetaSite> = result_search
        .into_iter()
        .map(|data| MetaSite {
            url: data.url,
            title: data.title,
        })
        .collect();

    let query = QuerySite {
        meta_data: meta_site_vec,
        duration,
    };

    println!("Час: {:?}", duration);

    Ok(query)
}

pub async fn search_image_data(text: &str) -> Result<QueryImage, Box<dyn Error + Send + Sync>> {
    let start_time = Instant::now();

    let text_owned = text.to_string();

    let raw_embedding = task::spawn_blocking(move || {
        let onnx = OnnxEmbeddingEngine::global();
        onnx.get_raw_embedding_query(&text_owned)
    })
    .await?
    ?;

    let limit = 30;
    let result_search = EngineSearch::engine_search(&raw_embedding, limit).await?;

    let duration = start_time.elapsed();

    let query = QueryImage {
        meta_data: result_search,
        duration,
    };

    println!("image {:?}", query);
    println!("Час: {:?}", duration);

    Ok(query)
}


pub async fn add_data(data: Vec<DataSiteResponse>) -> Result<(), Box<dyn Error + Send + Sync>> {
    let start_time_total = Instant::now();
    

    let max_parallel_sites = 3; 

    stream::iter(data)
        .for_each_concurrent(max_parallel_sites, |site_data| async move {
            let mut cout_sentens = 0;
            let start_time_site = Instant::now();

            let mut text_refs: Vec<&str> = site_data.text.iter().map(|s| s.as_str()).collect();
            text_refs.push(site_data.title.as_str());
            
            let sentences_result = extract_chunks(&text_refs).await.map_err(|e| e.to_string());

            match sentences_result {
                Ok(sentences) => {
                    for fragment in sentences {
                        cout_sentens += 1;
                        
                        let raw_embedding = {
                            let onnx = OnnxEmbeddingEngine::global();
                            onnx.get_raw_embedding_passage(fragment.as_str())
                        };

                        match raw_embedding {
                            Ok(tensor) => {

                                let meta_data = MetaData {
                                    url: site_data.link.clone(),
                                    title: site_data.title.clone(),
                                    image: site_data.image.clone(),
                                };

                                match EngineEdit::engine_insert(&tensor, meta_data).await {
                                    Ok(_) => {},
                                    Err(e) => println!("add_data (insert error): {}", e),
                                }
                            },
                            Err(e) => println!("Помилка ONNX: {}", e),
                        }
                    }
                },
                Err(e) => println!("Помилка чанкера: {}", e), 
            };

            let duration_site = start_time_site.elapsed();

            println!("--------------------------------------------------");
            println!("🔗 Сайт: {}", site_data.link);
            println!("⚡ Час оброблення сайту в потоці: {:?}", duration_site);
            println!("📊 Речень оброблено: {}", cout_sentens);
            println!("--------------------------------------------------");
        })
        .await;

    println!("✅ Всі сайти оброблуні час: {:?}", start_time_total.elapsed());
    Ok(())
}

pub async fn loading_data() -> Result<(), Box<dyn Error + Send + Sync>>{


    
    SaveLoadData::load_everything().await?;
    

    Ok(())
}

