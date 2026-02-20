use std::{error::Error, result};

use rayon::prelude::*;

mod extract_words;
mod cld3;
mod stremmer;

use cld3::cld3::cld3_main;
use extract_words::extract_words::extract_words;
use stremmer::stremer_main::stremer_main;



#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    let words = "Еду домой?";

    let result_cld3_main = cld3_main(&words).await?;

    let ve_res = extract_words(&[&words]).await?;

    
    println!("Lang: {} Vec:{:?}",result_cld3_main , ve_res);

    let result = stremer_main(&result_cld3_main, &ve_res).await?;
    Ok(())
}
