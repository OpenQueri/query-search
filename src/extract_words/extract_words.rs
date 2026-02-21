use regex::Regex;
use once_cell::sync::Lazy;
use std::error::Error;
use rustc_hash::FxHashMap;

/// Regular expression for matching words in Unicode text
/// Uses \p{L}+ to match any sequence of Unicode letters (supports all languages)
/// Lazy initialization ensures the regex is compiled only once when first accessed
static WORD_REGEX: Lazy<Option<Regex>> = Lazy::new(|| {
    Regex::new(r"\p{L}+").ok()
});

/// Extracts words from multiple text fragments and counts their frequencies
/// 
/// # Arguments
/// * `text_fragments` - A slice of string slices, each containing text to process
/// 
/// # Returns
/// * `Ok(Vec<(String, usize)>)` - A vector of (word, count) pairs, where each word is 
///   converted to lowercase and count represents its frequency across all fragments
/// * `Err(Box<dyn Error>)` - Error if the word regex failed to compile
/// 
/// # Example
/// ```
/// let fragments = vec!["Hello world", "Hello Rust"];
/// let word_counts = extract_words(&fragments).await?;
/// assert_eq!(word_counts, vec![
///     ("hello".to_string(), 2),
///     ("world".to_string(), 1),
///     ("rust".to_string(), 1)
/// ]);
/// ```
/// 
/// # Features
/// * **Unicode Support**: Works with any language that uses Unicode letters
/// * **Case Insensitive**: All words are converted to lowercase for consistent counting
/// * **Multi-fragment**: Processes multiple text fragments in a single pass
/// * **Memory Efficient**: Uses FxHashMap for O(1) word lookups during counting
/// 
/// # Performance Considerations
/// * The regex is compiled once using Lazy static for optimal performance
/// * FxHashMap provides efficient word counting with minimal overhead
/// * The function processes fragments sequentially, suitable for large text collections
pub async fn extract_words(text_fragments: &[&str]) -> Result<Vec<(String, usize)>, Box<dyn Error>>
{
    // Access the lazily compiled regex, return error if compilation failed
    let regex = match WORD_REGEX.as_ref() {
        Some(re) => re,
        None => return Err("WORD_REGEX no compile".into())
    };
    
    // FxHashMap to store word frequencies
    let mut words: FxHashMap<String, usize> = FxHashMap::default();

    // Process each text fragment
    for &fragment in text_fragments {
        // Find all word matches in the current fragment
        regex
            .find_iter(fragment)
            // Extract the matched text and convert to lowercase for case-insensitive counting
            .map(|mat: regex::Match<'_>| mat.as_str().to_lowercase())
            // Update the frequency count for each word
            .for_each(|word| {
                *words.entry(word).or_insert(0) += 1;
            });
    }    
    
    // Convert FxHashMap to Vec of (word, count) pairs for the return value
    Ok(words.into_iter().collect())
}