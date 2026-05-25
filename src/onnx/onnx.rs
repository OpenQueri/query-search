use ort::{inputs, session::Session, value::Value};
use tokenizers::Tokenizer;
use std::sync::OnceLock;
use crossbeam_channel::{Sender, Receiver, bounded};

pub struct OnnxEmbeddingEngine {
    tokenizer: Tokenizer,
    pool_rx: Receiver<Session>,
    pool_tx: Sender<Session>,
}

impl OnnxEmbeddingEngine {
    pub fn global() -> &'static Self {
        static INSTANCE: OnceLock<OnnxEmbeddingEngine> = OnceLock::new();
        INSTANCE.get_or_init(|| {
            let tokenizer = Tokenizer::from_file("tokenizer.json")
                .expect("Помилка: tokenizer.json не знайдено");
            
            let num_workers = std::thread::available_parallelism()
            .map(|n| n.get().min(3)) 
                .unwrap_or(3);

            let (tx, rx) = bounded::<Session>(num_workers);

            for _ in 0..num_workers {
                let model = Session::builder()
                    .expect("Помилка створення білдера сесії")
                    .with_intra_threads(1)
                    .expect("Помилка налаштування потоків")
                    .commit_from_file("model.onnx")
                    .expect("Помилка: model.onnx не знайдено");

                tx.send(model).expect("Не вдалося закинути воркера в пул");
            }

            Self {
                tokenizer,
                pool_rx: rx,
                pool_tx: tx,
            }
        })
    }

    pub fn get_raw_embedding_passage(&self, text: &str) -> Result<Vec<f32>, Box<dyn std::error::Error + Send + Sync>> {
        let text = format!("passage: {}", text);

        let encoding = self.tokenizer.encode(text, true).map_err(|e| e.to_string())?;
        let ids: Vec<i64> = encoding.get_ids().iter().map(|&x| x as i64).collect();
        let mask: Vec<i64> = encoding.get_attention_mask().iter().map(|&x| x as i64).collect();

        let seq_len = ids.len();

        let input_ids_value = Value::from_array(([1, seq_len], ids))?;
        let attention_mask_value = Value::from_array(([1, seq_len], mask))?;

        let mut model = self.pool_rx.recv().map_err(|_| "Помилка отримання моделі з пулу")?;

        let (data_vec, dim) = {
            let outputs = model.run(inputs![
                "input_ids" => input_ids_value,
                "attention_mask" => attention_mask_value,
            ])?;

            let output_value = &outputs["last_hidden_state"];
            let (shape, data) = output_value.try_extract_tensor::<f32>()?;
            let dim = *shape.last().unwrap() as usize;
            
            (data.to_vec(), dim)
        };

        self.pool_tx.send(model).map_err(|_| "Не вдалося повернути модель в пул")?;

        // === CLS Pooling (головна зміна) ===
        let embedding = data_vec[0..dim].to_vec();

        // L2 normalization
        let norm = embedding.iter().map(|x| x * x).sum::<f32>().sqrt();
        
        let mut embedding = if norm > 0.0 {
            embedding.into_iter().map(|x| x / norm).collect()
        } else {
            embedding
        };

        Ok(embedding)
    }

   pub fn get_raw_embedding_query(&self, text: &str) -> Result<Vec<f32>, Box<dyn std::error::Error + Send + Sync>> {
        let text = format!("query: {}", text);

        let encoding = self.tokenizer.encode(text, true).map_err(|e| e.to_string())?;
        let ids: Vec<i64> = encoding.get_ids().iter().map(|&x| x as i64).collect();
        let mask: Vec<i64> = encoding.get_attention_mask().iter().map(|&x| x as i64).collect();

        let seq_len = ids.len();

        let input_ids_value = Value::from_array(([1, seq_len], ids))?;
        let attention_mask_value = Value::from_array(([1, seq_len], mask))?;

        let mut model = self.pool_rx.recv().map_err(|_| "Помилка отримання моделі з пулу")?;

        let (data_vec, dim) = {
            let outputs = model.run(inputs![
                "input_ids" => input_ids_value,
                "attention_mask" => attention_mask_value,
            ])?;

            let output_value = &outputs["last_hidden_state"];
            let (shape, data) = output_value.try_extract_tensor::<f32>()?;
            let dim = *shape.last().unwrap() as usize;
            
            (data.to_vec(), dim)
        };

        self.pool_tx.send(model).map_err(|_| "Не вдалося повернути модель в пул")?;

        // === CLS Pooling (головна зміна) ===
        let embedding = data_vec[0..dim].to_vec();

        // L2 normalization
        let norm = embedding.iter().map(|x| x * x).sum::<f32>().sqrt();
        let mut embedding = if norm > 0.0 {
            embedding.into_iter().map(|x| x / norm).collect()
        } else {
            embedding
        };

        Ok(embedding)
    }
}