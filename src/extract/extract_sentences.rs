use unicode_segmentation::UnicodeSegmentation;
use std::{error::Error};

const TARGET_WORDS_PER_CHUNK: usize = 140;   
const OVERLAP_WORDS: usize = 35;             

pub async fn extract_chunks(text_fragments: &[&str]) -> Result<Vec<String>, Box<dyn Error + Send + Sync>> {
    let mut chunks = Vec::new();
    let full_text = text_fragments.join(" ");
    
    let sentences: Vec<&str> = full_text.unicode_sentences().collect();
    let mut current_chunk = String::new();
    let mut current_words = Vec::new();

    for sentence in sentences {
        let sentence = sentence.trim();
        if sentence.is_empty() {
            continue;
        }

        let sentence_words: Vec<&str> = sentence.split_whitespace().collect();
        let sentence_word_count = sentence_words.len();

        if sentence_word_count > TARGET_WORDS_PER_CHUNK * 2 {
            if !current_chunk.is_empty() {
                chunks.push(current_chunk.trim().to_string());
                current_chunk.clear();
                current_words.clear();
            }
            chunks.push(sentence.to_string());
            continue;
        }

        if current_words.len() + sentence_word_count > TARGET_WORDS_PER_CHUNK {
            if !current_chunk.is_empty() {
                chunks.push(current_chunk.trim().to_string());
            }

            let start = current_words.len().saturating_sub(OVERLAP_WORDS);
            current_words = current_words[start..].to_vec();
            current_chunk = current_words.join(" ");
        }

        if !current_chunk.is_empty() {
            current_chunk.push(' ');
        }
        current_chunk.push_str(sentence);
        current_words.extend_from_slice(&sentence_words);
    }

    if !current_chunk.is_empty() {
        chunks.push(current_chunk.trim().to_string());
    }

    let filtered: Vec<String> = chunks
        .into_iter()
        .filter(|c| c.split_whitespace().count() >= 25)
        .collect();

    Ok(filtered)
}
