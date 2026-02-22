use std::{error::Error, sync::RwLock};

use dashmap::DashMap;
use once_cell::sync::Lazy;


use crate::Request;

static ALL_LINKS: Lazy<RwLock<Vec<String>>> = Lazy::new(||{ 
    RwLock::new(
       vec![
            "gogle".to_string()
        ]
)});

struct SearchLink{
    link: String,
    frequency: usize,
    
}

static LINK_DATA: Lazy<DashMap<String, Vec<SearchLink>>> = Lazy::new(||{DashMap::new()});




pub async fn engine_main(request: &Request<'_>) -> Result<(), Box<dyn Error>>{


    println!("{:?}", &request);




    

    Ok(())
}