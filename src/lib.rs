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
use perf_event::{Builder, Group, events::Hardware};
use std::arch::x86_64::_rdtsc;


use crate::engine::engine_main::Response;

#[inline(always)]
fn read_tsc() -> u64 {
    unsafe { _rdtsc() }
}

#[derive(Debug, Clone, Serialize)] 
pub struct ResponseSearchData<'b>{
    pub language: &'b str,
    pub result: Vec<Response>,
    pub duration: Duration,
}

pub async fn search_data(text: &str) -> Result<ResponseSearchData, Box<dyn Error>> {
    // Початок вимірювання
    //let start_tsc = read_tsc();
    let start_time = Instant::now();

    // Основний код пошуку
    let vec_text = vec![text];
    let result_extract_words: Vec<(String, usize)> = extract_words(&[&vec_text[0]]).await?;
    let result_cld3_main = cld3_main(text).await?;
    let stremer_main = stremer_main(&result_cld3_main, &result_extract_words).await?;
    let result = EngineSearch::engine_search(stremer_main).await?;
    
    let duration = start_time.elapsed();
    //let end_tsc = read_tsc();
    
    // Підрахунок тактів
    //let cycles = end_tsc - start_tsc;
    
    // Статистика
    // println!("\n📊 ПРОДУКТИВНІСТЬ ПОШУКУ:");
    // println!("   Запит: \"{}\"", text);
    // println!("   Довжина тексту: {} символів", text.len());
    println!("   Час: {:?}", duration);
    // println!("   Тактів CPU: {} тактів", cycles);
    // println!("   Тактів на символ: {:.0} тактів", cycles as f64 / text.len() as f64);
    
    // Оцінка IPC (приблизно 4-6 інструкцій за такт)
    // let estimated_ipc = 5.0;
    //// let estimated_instructions = cycles as f64 * estimated_ipc;
    //println!("   Приблизно інструкцій: {:.0}", estimated_instructions);
    
    // Розрахунок теоретичного RPS
    //let rps = 1_000_000_000.0 / duration.as_nanos() as f64;
    //println!("   Теоретичний RPS: {:.0} зап/сек", rps);
    
    let response = ResponseSearchData {
        language: &result_cld3_main,
        result: result,
        duration: duration,
    };
    
    Ok(response)
}

#[derive(Debug, Clone, Serialize)] 
pub struct DataADD<'a>{
    pub title: &'a str,
    pub link: &'a str,
    pub text: &'a str,
}

pub async fn add_data(data: &DataADD<'_>) -> Result<(), Box<dyn Error>>{

    

    let (request, link, title) = match  request(data.text, data.link, data.title).await {
        Ok(val) => val,
        Err(e) => return Err(e), 
    };


    EngineEdit::engine_write(&title, &link, &request).await?;
    

    Ok(())
}


#[derive(Debug, Clone, Serialize)] 
pub struct Request<'a>{
    pub launge: &'a str,
    pub words: Vec<String>,
}
pub async fn request<'b>(text: &str, link:&'b str, title: &'b str ) -> Result<(Request<'b>, &'b str,&'b str), Box<dyn Error>>{


    let result_cld3_main = cld3_main(&text).await?;

    let result_extract_words = extract_words(&[&text]).await?;

    let stremer_main = stremer_main(&result_cld3_main, &result_extract_words).await?;

    let request = Request{
        launge: &result_cld3_main,
        words: stremer_main,
    };


    Ok((request,&link, &title))
}