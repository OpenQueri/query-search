use std::error::Error;

use rayon::prelude::*;

use rust_stemmers::{Algorithm, Stemmer};

fn main() -> Result<(), Box<dyn Error>> {

    let keys = vec![
        "foo", "food", "fob", "focus",     // English
        "дом", "машина", "река",           // Russian
        "дім", "машина", "ріка"             // Ukrainian
    ];

    

    Ok(())
}
