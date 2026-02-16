use regex::Regex;
use once_cell::sync::Lazy;
use std::error::Error;
use std::{collections::HashMap};



static WORD_REGEX: Lazy<Option<Regex>> = Lazy::new(|| {
    Regex::new(r"\p{L}+").ok()
});

pub async fn extract_words(text_fragments: &[&str]) -> Result<Vec<(String, usize)>, Box<dyn Error>>
{
    let regex = match WORD_REGEX.as_ref() {
        Some(re) => re,
        None => return Err("WORD_REGEX no compile".into())
    };
    let mut words: HashMap<String, usize> = HashMap::new();

    for &fragment in text_fragments {
        regex
            .find_iter(fragment)
            .map(|mat: regex::Match<'_>| mat.as_str().to_lowercase())
            .for_each(|word| {
                *words.entry(word).or_insert(0) += 1;
            });
    }    
    Ok(words.into_iter().collect())
}