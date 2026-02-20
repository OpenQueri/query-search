use cld3::NNetLanguageIdentifier;
use std::error::Error;
use object_pool::Pool;
use std::cell::RefCell;
use phf::phf_map;

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

);

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

pub async fn cld3_main(text: &str) -> Result<&'static str, Box<dyn Error>> {
    CLD_POOL.with(|pool_cell| {
        let mut pool_opt = pool_cell.borrow_mut();
        
        match pool_opt.as_mut() {
            Some(pool) => {
                let mut detector = pool.pull(|| {
                    NNetLanguageIdentifier::new(0, 500)
                        .expect("Failed to create CLD3 detector")
                });
                
                let result = detector.find_language(text);
                let language = match LANGUAGES.get(result.language.as_str()).copied() {
                    Some(res) => res,
                    None => "None",
                };
                
                Ok(language)
            }
            None => Err("CLD3 not available".into())
        }
    })
}