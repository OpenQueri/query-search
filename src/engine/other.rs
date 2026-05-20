use std::{error::Error};

use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;
use fxhash::FxHasher;


pub struct Other;

impl  Other{


    pub async fn hash_u64(link: &str) -> u64 {
        let mut hasher = FxHasher::default();
        link.hash(&mut hasher);
        hasher.finish()
    }
}