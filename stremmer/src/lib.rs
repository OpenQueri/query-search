#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

use std::ffi::{CStr, CString};
use std::ptr;
use std::os::raw::c_char;

pub struct Stemmer {
    inner: *mut sb_stemmer,
}

impl Stemmer {
    pub fn new(lang: &str) -> Option<Self> {
        unsafe {
            let c_lang = CString::new(lang).ok()?;
            let inner = sb_stemmer_new(c_lang.as_ptr(), ptr::null());
            
            if inner.is_null() {
                None
            } else {
                Some(Stemmer { inner })
            }
        }
    }
    
    pub fn stem(&self, word: &str) -> Option<String> {
        unsafe {
            let c_word = CString::new(word).ok()?;
            let stemmed = sb_stemmer_stem(
                self.inner,
                c_word.as_ptr() as *const u8,
                word.len() as std::os::raw::c_int,
            );
            
            if stemmed.is_null() {
                None
            } else {
                Some(CStr::from_ptr(stemmed as *const c_char).to_string_lossy().into_owned())
            }
        }
    }
}

impl Drop for Stemmer {
    fn drop(&mut self) {
        unsafe {
            sb_stemmer_delete(self.inner);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_ukrainian() {
        let stemmer = Stemmer::new("ukrainian").expect("Failed to create stemmer");
        let word = "тестовий";
        let stemmed = stemmer.stem(word).expect("Failed to stem");
        println!("{} -> {}", word, stemmed);
    }
    
    #[test]
    fn test_english() {
        let stemmer = Stemmer::new("english").expect("Failed to create stemmer");
        let word = "testing";
        let stemmed = stemmer.stem(word).expect("Failed to stem");
        println!("{} -> {}", word, stemmed);
    }
}