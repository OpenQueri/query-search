use std::error::Error;

use rayon::prelude::*;
use std::sync::Arc;

mod cld3;


use cld3::cld3::cld3_main;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {



    let res = cld3_main("Can I say Hello please?").await?;
    
    println!("{}", res);

    Ok(())
}
