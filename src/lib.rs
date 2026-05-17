use std::{error::Error, time::Duration};



mod engine;
mod seo;
mod extract_words;

use serde::Serialize;
use engine::engine_main::{EngineEdit, EngineSearch,SaveLoadData};
use std::time::Instant;
use perf_event::{Builder, Group, events::Hardware};
use std::arch::x86_64::_rdtsc;
use crawler_engine::DataSiteResponse;

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



pub async fn add_data(data: &Vec<DataSiteResponse>) -> Result<(), Box<dyn Error>>{

    
    


    

    Ok(())
}


pub async fn loading_data() -> Result<(), Box<dyn Error>>{


    SaveLoadData::load_everything().await?;
    

    Ok(())
}

