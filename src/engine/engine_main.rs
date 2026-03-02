use std::{error::Error, sync::RwLock, sync::Arc};

use dashmap::DashMap;
use indexmap::IndexMap;
use libc::SOCK_CLOEXEC;
use once_cell::sync::{Lazy};

use rayon::{prelude::*, spawn, scope};

use crate::Request;
use crate::engine::other::*;


static ALL_LINKS: Lazy<Arc<RwLock<IndexMap<u64, String>>>> = Lazy::new(||{Arc::new(RwLock::new(IndexMap::new()))});



//Dash map search "word" -> Vec<"LinkData inside link: usize index and frequency usize">
static LINK_DATA: Lazy<DashMap<String, Vec<u64>>> = Lazy::new(||{DashMap::new()});

#[derive(Debug)]
pub struct Response{
    link: String,
    frequency: usize,
}

pub struct EngineSearch;

impl EngineSearch{

    
    pub async fn engine_search(request: &Request<'_>) -> Result<(), Box<dyn Error>>{
        println!("{:?}", request);

        let all_links_map = Arc::clone(&ALL_LINKS);

        scope(|s|{
            s.spawn(|_|{

                let mut map_index_site:IndexMap<usize, usize>  = IndexMap::new();

                for word in request.words.iter(){
                    if let Some(val) = LINK_DATA.get(word){
                        val.iter().for_each(|iw| {
                            *map_index_site.entry((*iw).try_into().unwrap()).or_insert(0) += 1;
                        });
                    }
                }
                println!("{:?}", map_index_site);

                let mut response: Vec<Response> = Vec::new();
                

                for (site_idx, site_frequency) in map_index_site.iter(){
                    let hesh = site_idx.clone() as u64;
                    if let Some(get_link) = all_links_map.read().unwrap().get(&hesh){
                        println!("{}", get_link);
                        response.push(Response {
                            link: get_link.clone(),
                            frequency: site_frequency.clone(),
                        });
                    };
                    

                }

                println!("{:?}", response)
                
            });
        });

        Ok(())
    }

}
pub struct EngineEdit;

impl EngineEdit {
    pub async fn engine_write(link: &str, request:&Request<'_>) -> Result<(), Box<dyn Error>>{

        let all_links_map = Arc::clone(&ALL_LINKS);


        let worlds = &request.words;


        scope(|s|{
            s.spawn( |_| {
               
                let hesh = AllFrequencySite.calculate_hash(&link).unwrap();
                println!("\n Hesh {}", hesh);

                if let Some(val) = all_links_map.read().unwrap().get_index_of(&hesh){


                    for word in worlds.iter() {
                        LINK_DATA.entry(word.to_string()).or_insert(Vec::new()).push(hesh);
                    }

                }else {

                    let mut writetable_map = all_links_map.write().unwrap();

                    //measurement general gen_num_id 
                    let gen_num_id = AllFrequencySite.calculate_hash(&link).unwrap();
                    let  (idx, _) = writetable_map.insert_full(gen_num_id, link.to_string());
                    
    
                    for word in worlds.iter() {
                        LINK_DATA.entry(word.to_string()).or_insert(Vec::new()).push(gen_num_id);
                    
                    }
                    
                    
                };
                

            });
        });
        println!("{:?}, \n\n {:?}", ALL_LINKS.read(), LINK_DATA);

        Ok(())
    }


}


