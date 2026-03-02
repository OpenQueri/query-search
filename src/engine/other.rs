use std::{error::Error};

use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

pub trait Other {
    fn cout_frequency(&self, words: &[String]) -> Result<u64, Box<dyn Error>>;
    fn calculate_hash(&self, link: &str) -> Result<u64, Box<dyn Error>>;
}

pub struct AllFrequencySite;

impl Other for AllFrequencySite{
    fn cout_frequency(&self, words: &[String]) -> Result<u64, Box<dyn Error>>{
        
        let len = words.len();
        Ok(len as u64)
    }

    fn calculate_hash(&self, link: &str) -> Result<u64, Box<dyn Error>> {
        let mut hasher = DefaultHasher::new();
        link.hash(&mut hasher);
        Ok(hasher.finish())
    }
}