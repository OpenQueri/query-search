use whatlang::detect;
use std::error::Error;
use phf::phf_map;

/// Теперь используем строки "&str" как ключи, это надежнее для PHF
static LANGUAGES: phf::Map<&'static str, &'static str> = phf_map!(
    "ara" => "arabic",
    "hye" => "armenian",
    "eus" => "basque",
    "cat" => "catalan",
    "dan" => "danish",
    "nld" => "dutch",
    "eng" => "english",
    "epo" => "esperanto",
    "est" => "estonian",
    "fin" => "finish",
    "fra" => "french",
    "deu" => "german",
    "ell" => "greek",
    "hin" => "hindi",
    "hun" => "hungarian",
    "ind" => "indonesian",
    "iri" => "irish",
    "ita" => "italian",
    "lit" => "lithuanian",
    "nep" => "nepali",
    "nor" => "norwegian",
    "pol" => "polish",
    "por" => "portuguese",
    "ron" => "romanian",
    "rus" => "russian",
    "srp" => "serbian",
    "spa" => "spanish",
    "swe" => "swedish",
    "tam" => "tamil",
    "tur" => "turkish",
    "yid" => "yiddish",
    "ukr" => "ukrainian",
);

pub async fn cld3_main(text: &str) -> Result<&'static str, Box<dyn Error>> {
    if let Some(info) = detect(text) {
        // Получаем ISO код языка (например, "rus")
        let code = info.lang().code(); 
        
        // Ищем в карте по строковому ключу
        let language = LANGUAGES.get(code).copied().unwrap_or("None");
        Ok(language)
    } else {
        Ok("None")
    }
}