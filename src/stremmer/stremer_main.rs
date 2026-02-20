

use std::error::Error;

use stremer::Stemmer;


pub async fn stremer_main(launge: &str, words: &[(String, usize)]) -> Result<(), Box<dyn Error>>{

    let stremmer = match Stemmer::new(&launge) {
        Some(res) => res,
        None => return Err(format!("Failed to create stemmer for language: {}", launge).into()),
    };

    //let result = Vec::new();

    for word in words.iter(){

        if let Some(stemmed) = stremmer.stem(&word.0) {
            println!("{} -> {}", word.0, stemmed);
            
        } else {
            println!("{} -> ошибка стемминга", word.0);
        }

    }

    Ok(())
    
}
