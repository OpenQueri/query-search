use std::{error::Error, time::Duration};



mod extract_words;
mod cld3;
mod stremmer;
mod engine;

use serde::Serialize;
use cld3::cld3::cld3_main;
use extract_words::extract_words::extract_words;
use stremmer::stremer_main::stremer_main;
use engine::engine_main::{EngineEdit, EngineSearch};
use std::time::Instant;

use crate::engine::engine_main::Response;


#[derive(Debug, Clone, Serialize)] 
pub struct ResponseSearchData{
    pub result: Vec<Response>,
    pub duration: Duration,
}

pub async fn search_data(text: &str) -> Result<ResponseSearchData, Box<dyn Error>>{

    let start = Instant::now();

    let vec_text = vec![text];
    
    let extra: Vec<(String, usize)> = extract_words(&[&vec_text[0]]).await?;

    let result = EngineSearch::engine_search(extra).await?;

    let duration = start.elapsed();

    let response = ResponseSearchData{
        result: result,
        duration: duration,
    };

    Ok(response)
}


pub async fn add_data(link: &[&str], text: &[&str]) -> Result<(), Box<dyn Error>>{

    for (link, text) in link.iter().zip(text.iter()) {
        let (request, link) = match  request(text, link).await {
            Ok(val) => val,
            Err(e) => return Err(e), 
        };
        EngineEdit::engine_write(&link, &request).await?;
    }

    Ok(())
}


#[derive(Debug, Clone, Serialize)] 
pub struct Request<'a>{
    pub launge: &'a str,
    pub words: Vec<String>,
}
pub async fn request<'b>(text: &str, link:&'b str ) -> Result<(Request<'b>, &'b str), Box<dyn Error>>{


    let result_cld3_main = cld3_main(&text).await?;

    let result_extract_words = extract_words(&[&text]).await?;

    let stremer_main = stremer_main(&result_cld3_main, &result_extract_words).await?;

    let request = Request{
        launge: &result_cld3_main,
        words: stremer_main,
    };


    Ok((request,&link))
}