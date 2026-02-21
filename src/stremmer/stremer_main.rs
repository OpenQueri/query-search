
use std::error::Error;
use stremer::Stemmer;


pub async fn stremer_main(launge: &str, words: &[(String, usize)]) -> Result<Vec<Option<String>>, Box<dyn Error>>{

    let stremmer = match Stemmer::new(&launge) {
        Some(res) => res,
        None => return Err(format!("Failed to create stemmer for language: {}", launge).into()),
    };

    let mut result_vec: Vec<Option<String>> = Vec::new();
    for word in words.iter(){

        if let Some(stemmed) = stremmer.stem(&word.0) {
            result_vec.push(Some(stemmed));
            
        } else {
            result_vec.push(None);
        }

    }

    Ok(result_vec)
    
}
