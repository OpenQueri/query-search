use sled::{Db, Tree};
use once_cell::sync::Lazy;
use serde::{Serialize, Deserialize};
use std::error::Error;

// Импортируем твой правильный резак
use crate::extract_words::extract_words::extract_words;

// Глобальный объект индекса
pub static BM25_ENGINE: Lazy<Bm25Index> = Lazy::new(|| Bm25Index::open("bm25_data"));

pub struct Bm25Index {
    db: Db,
    df_tree: Tree,          // Слово -> Количество документов (u32)
    meta_tree: Tree,        // "total_docs", "total_words"
    doc_lengths: Tree,      // ID документа -> Его длина (u32)
}

impl Bm25Index {
    pub fn open(path: &str) -> Self {
        let db = sled::open(path).expect("Не удалось открыть Sled для BM25");
        let df_tree = db.open_tree("df").expect("Ошибка дерева DF");
        let meta_tree = db.open_tree("meta").expect("Ошибка дерева Meta");
        let doc_lengths = db.open_tree("lengths").expect("Ошибка дерева Lengths");

        Self { db, df_tree, meta_tree, doc_lengths }
    }

    /// Добавление документа теперь async, так как extract_words — async
    pub async fn add_document(&self, doc_id: usize, text: &str) -> Result<(), Box<dyn Error>> {
        // Используем твой Unicode-резак
        let word_counts = extract_words(&[text]).await?;

        if word_counts.is_empty() { return Ok(()); }

        let mut total_doc_words: u32 = 0;

        for (word, count) in word_counts {
            total_doc_words += count as u32;
            
            // DF увеличиваем только на 1 (факт наличия слова в документе), 
            // независимо от того, сколько раз оно там встретилось
            self.increment_df(&word);
        }

        // 1. Сохраняем общую длину документа
        self.doc_lengths.insert(doc_id.to_be_bytes(), &total_doc_words.to_be_bytes())?;

        // 2. Общая статистика (docs + 1, words + len)
        self.increment_meta("total_docs", 1);
        self.increment_meta("total_words", total_doc_words as u64);

        // Сбрасываем на диск
        let _ = self.db.flush_async().await;
        Ok(())
    }

    /// Оценка документа. doc_text тоже прогоняем через Unicode-резак для точности
    pub async fn score(&self, query: &str, doc_id: usize, doc_text: &str) -> Result<f32, Box<dyn Error>> {
        let k1 = 1.2;
        let b = 0.75;
        
        // Токенизируем запрос и документ твоим методом
        let query_words = extract_words(&[query]).await?;
        let doc_words = extract_words(&[doc_text]).await?;
        
        // Превращаем слова документа в HashMap для O(1) доступа к частоте (TF)
        let doc_tf_map: std::collections::HashMap<String, usize> = doc_words.into_iter().collect();

        let avg_dl = self.get_avg_dl();
        let total_docs = self.get_meta_u64("total_docs") as f32;

        let doc_len = self.doc_lengths.get(doc_id.to_be_bytes())?
            .map(|b| u32::from_be_bytes(b.as_ref().try_into().unwrap()))
            .unwrap_or(0) as f32;

        let mut total_score = 0.0;

        for (term, _q_count) in query_words {
            // Ищем слово в глобальной базе DF
            let df = match self.df_tree.get(&term)? {
                Some(val) => u32::from_be_bytes(val.as_ref().try_into().unwrap()) as f32,
                None => continue,
            };

            // 1. IDF: Насколько редкое слово во всем индексе
            let idf = ((total_docs - df + 0.5) / (df + 0.5) + 1.0).ln();
            
            // 2. TF: Как часто слово в этом конкретном документе
            let tf = *doc_tf_map.get(&term).unwrap_or(&0) as f32;

            if tf > 0.0 {
                // Формула BM25
                let numerator = tf * (k1 + 1.0);
                let denominator = tf + k1 * (1.0 - b + b * (doc_len / avg_dl));
                total_score += idf * (numerator / denominator);
            }
        }

        Ok(total_score)
    }

    // --- Вспомогательные методы ---

    fn increment_meta(&self, key: &str, inc: u64) {
        let _ = self.meta_tree.fetch_and_update(key, |old| {
            let val = old.map(|b| u64::from_be_bytes(b.try_into().unwrap())).unwrap_or(0);
            Some((val + inc).to_be_bytes().to_vec())
        });
    }

    fn increment_df(&self, word: &str) {
        let _ = self.df_tree.fetch_and_update(word, |old| {
            let val = old.map(|b| u32::from_be_bytes(b.try_into().unwrap())).unwrap_or(0);
            Some((val + 1).to_be_bytes().to_vec())
        });
    }

    pub fn get_avg_dl(&self) -> f32 {
        let docs = self.get_meta_u64("total_docs") as f32;
        let words = self.get_meta_u64("total_words") as f32;
        if docs == 0.0 { return 0.0; }
        words / docs
    }

    fn get_meta_u64(&self, key: &str) -> u64 {
        self.meta_tree.get(key).unwrap()
            .map(|b| u64::from_be_bytes(b.as_ref().try_into().unwrap()))
            .unwrap_or(0)
    }
}