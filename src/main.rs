use std::{error::Error};

use rayon::prelude::*;

mod extract_words;
mod cld3;
mod stremmer;
mod engine;

use cld3::cld3::cld3_main;
use extract_words::extract_words::extract_words;
use stremmer::stremer_main::stremer_main;
use engine::engine_main::engine_main;

#[derive(Debug)]
pub struct Request<'a>{
    launge: &'a str,
    words: Vec<Option<String>>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    let words = "Чи не здається тобі, що коли я сьогодні пізно ввечері після довгого та виснажливого робочого дня нарешті сяду в той старий, трохи іржавий, але такий рідний автомобіль та поїду через все місто додому, де на мене чекає тепла постіль, свіжозварена кава та улюблений кіт, який вже напевно засумував, то це буде найкращим завершенням цього нескінченного дня?";

    let result_cld3_main = cld3_main(&words).await?;

    let result_extract_words = extract_words(&[&words]).await?;
    

    let stremer_main = stremer_main(&result_cld3_main, &result_extract_words).await?;

    let res = Request{
        launge: result_cld3_main,
        words: stremer_main,
    };

    engine_main(&res).await?;


    Ok(())
}
