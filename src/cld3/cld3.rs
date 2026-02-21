use cld3::NNetLanguageIdentifier;
use std::error::Error;
use object_pool::Pool;
use std::cell::RefCell;
use phf::phf_map;

/// Static mapping of language codes to their full names
/// Supports both ISO 639-1 (2-letter) and ISO 639-2 (3-letter) codes
static LANGUAGES: phf::Map<&'static str, &'static str> = phf_map!(
    "ar" => "arabic",
    "hy" => "armenian",
    "eu" => "basque",
    "ca" => "catalan",
    "da" => "danish",
    "nl" => "dutch",
    "en" => "english",
    "eo" => "esperanto",
    "et" => "estonian",
    "fi" => "finnish",
    "fr" => "french",
    "de" => "german",
    "el" => "greek",
    "hi" => "hindi",
    "hu" => "hungarian",
    "id" => "indonesian",
    "ga" => "irish",
    "it" => "italian",
    "lt" => "lithuanian",
    "ne" => "nepali",
    "no" => "norwegian",
    "pl" => "polish",
    "pt" => "portuguese",
    "ro" => "romanian",
    "ru" => "russian",
    "sr" => "serbian",
    "es" => "spanish",
    "sv" => "swedish",
    "ta" => "tamil",
    "tr" => "turkish",
    "yi" => "yiddish",
    "uk" => "ukrainian",
    "eng" => "english",
    "ara" => "arabic",
    "fra" => "french",
    // Additional mappings can be added here
);

/// Thread-local storage for CLD3 detector pool
/// Uses RefCell for interior mutability and Option for lazy initialization
/// Pool size is set to 32 detectors to handle concurrent requests efficiently
thread_local! {
    static CLD_POOL: RefCell<Option<Pool<NNetLanguageIdentifier>>> = RefCell::new(
        NNetLanguageIdentifier::new(0, 500).ok().map(|_| {
            Pool::new(32, || {
                NNetLanguageIdentifier::new(0, 500)
                    .expect("Failed to create CLD3 detector")
            })
        })
    );
}

/// Detects the language of the input text using CLD3
/// 
/// # Arguments
/// * `text` - The input text to detect language for
/// 
/// # Returns
/// * `Ok(&'static str)` - The detected language name (e.g., "english", "spanish")
/// * `Err(Box<dyn Error>)` - Error if CLD3 is not available or detection fails
/// 
/// # Example
/// ```
/// let lang = cld3_main("Hello world").await?;
/// assert_eq!(lang, "english");
/// ```
/// 
/// # Performance
/// Uses an object pool to reuse CLD3 detectors, reducing allocation overhead
/// The pool is thread-local, so each thread gets its own set of detectors
pub async fn cld3_main(text: &str) -> Result<&'static str, Box<dyn Error>> {
    CLD_POOL.with(|pool_cell| {
        let mut pool_opt = pool_cell.borrow_mut();
        
        match pool_opt.as_mut() {
            Some(pool) => {
                // Pull a detector from the pool or create a new one if needed
                let mut detector = pool.pull(|| {
                    NNetLanguageIdentifier::new(0, 500)
                        .expect("Failed to create CLD3 detector")
                });
                
                // Perform language detection
                let result = detector.find_language(text);
                
                // Map the detected language code to its full name
                let language = match LANGUAGES.get(result.language.as_str()).copied() {
                    Some(res) => res,
                    None => "None", // Return "None" for unsupported languages
                };
                
                Ok(language)
            }
            None => Err("CLD3 not available".into())
        }
    })
}