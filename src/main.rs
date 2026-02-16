use std::error::Error;

use rayon::prelude::*;
use std::sync::Arc;

mod extract_words;
mod cld3;


use cld3::cld3::cld3_main;
use extract_words::extract_words::extract_words;


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    let words = "Can I say Hello please?";

    let res = cld3_main(&words).await?;
    
    let ve_res = extract_words(&[&words]).await?;

    println!("Lang{} Vec:{:?}",res , ve_res);

    Ok(())
}
