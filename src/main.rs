use std::{error::Error, thread};

use rayon::prelude::*;

mod extract_words;
mod cld3;
mod stremmer;
mod engine;

use cld3::cld3::cld3_main;
use extract_words::extract_words::extract_words;
use stremmer::stremer_main::stremer_main;
use engine::engine_main::{EngineEdit, EngineSearch};
use std::time::Instant;

#[derive(Debug)]
pub struct Request<'a>{
    launge: &'a str,
    words: Vec<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {



    let link= vec!["https://www.google.com/","https://www.googl4.com/","https://www.goo4gl4.com/"];

    let text = vec!["Знаєш, іноді після чергового нескінченного робочого марафону, коли вже сил майже не лишилось, я сідаю в свою стару машинку, яка скрипить і пахне бензином, і просто їду додому через вечірнє місто. Там мене чекає м’яке ліжко, гарячий чай з м’ятою, і мій пухнастий кіт, який весь день сумно нявкав біля дверей. І в цю мить здається, що все на світі знову стало на свої місця.",
    "Після цілого дня за комп’ютером, коли очі вже болять, а спина ниє, я заводжу свій пошарпаний, але надійний автомобіль і повільно їду через затори додому. Вдома вже готова тепла ковдра, щойно заварена кава з корицею і мій улюблений сірий котяра, який крутиться під ногами і вимагає уваги. Саме ці прості речі роблять вечір по-справжньому хорошим.",
    "Буває, що після виснажливого дня в офісі я ледве дочікуюсь моменту, коли сяду за кермо своєї старенької машини й поїду додому. Місто вже світиться вогнями, а я думаю тільки про те, як зараз ляжу в затишне ліжко, вип’ю ароматної кави і погладжу свого пухнастика, який весь день чекав мене біля вікна. Ось заради таких вечорів і живеш."];
    
    for (link, text) in link.iter().zip(text.iter()) {

        let (request, link) = match  request(text, link).await {
            Ok(val) => val,
            Err(e) => return Err(e), 
        };
        EngineEdit::engine_write(&link, &request).await?;
    }

    let extra: Vec<(String, usize)> = extract_words(&[&text[0]]).await?;

    let start = Instant::now();
    EngineSearch::engine_search(extra).await?;
    let duration = start.elapsed();

    println!("Выполнилось за: {:?}", duration);
    Ok(())
}


async fn request<'a, 'b>(text: &'a str, link:&'b str ) -> Result<(Request<'b>, &'b str), Box<dyn Error>>{


    let result_cld3_main = cld3_main(&text).await?;

    let result_extract_words = extract_words(&[&text]).await?;

    let stremer_main = stremer_main(&result_cld3_main, &result_extract_words).await?;

    let request = Request{
        launge: &result_cld3_main,
        words: stremer_main,
    };


    Ok((request,&link))
}