//! Build or rebuild the embedding index.

use std::path::Path;

use crate::discovery::TaskCollection;
use crate::embedding::{
    create_rolling_windows, EmbeddingIndex, EmbeddingMetadata, WindowIndex, DEFAULT_EMBEDDING_DIM,
    DEFAULT_MODEL_ID, DEFAULT_OVERLAP, DEFAULT_WINDOW_SIZE,
};

pub fn execute(tasks_path: &Path, status: bool, model_id: Option<&str>) -> crate::Result<()> {
    let model_id = model_id.unwrap_or(DEFAULT_MODEL_ID);
    let embeddings_path = tasks_path
        .parent()
        .unwrap_or(tasks_path)
        .join(".taskgraph/embeddings.safetensors");

    if status {
        return show_status(&embeddings_path);
    }

    println!("Loading model {}...", model_id);
    let model = model2vec_rs::model::StaticModel::from_pretrained(model_id, None, None, None)
        .map_err(|e| crate::Error::Graph(format!("Failed to load model: {}", e)))?;

    println!("Discovering tasks...");
    let collection = TaskCollection::from_directory(tasks_path);

    if collection.is_empty() {
        println!("No tasks found in {}", tasks_path.display());
        return Ok(());
    }

    println!("Found {} tasks. Building embeddings...", collection.len());

    let mut all_embeddings: Vec<Vec<f32>> = Vec::new();
    let mut all_indices: Vec<WindowIndex> = Vec::new();
    let mut path_map: std::collections::HashMap<u64, String> = std::collections::HashMap::new();
    let mut file_count = 0;

    for task in collection.tasks() {
        let Some(source) = &task.source else {
            continue;
        };

        let body = &task.body;
        if body.trim().is_empty() {
            continue;
        }

        file_count += 1;
        let file_path_hash = WindowIndex::hash_path(source);
        path_map.insert(file_path_hash, source.clone());

        let tokenizer = model.tokenizer();
        let encoding = tokenizer
            .encode(body.as_str(), false)
            .map_err(|e| crate::Error::Graph(format!("Tokenization failed: {}", e)))?;

        let token_ids = encoding.get_ids().to_vec();
        let offsets: Vec<usize> = encoding.get_offsets().iter().map(|(s, _)| *s).collect();

        if token_ids.is_empty() {
            continue;
        }

        let windows =
            create_rolling_windows(&token_ids, &offsets, DEFAULT_WINDOW_SIZE, DEFAULT_OVERLAP);

        for (window_tokens, start_token, end_token, start_char, end_char) in windows {
            let window_text: String = tokenizer.decode(&window_tokens, false).unwrap_or_default();

            let embedding = model.encode_single(&window_text);

            all_embeddings.push(embedding);
            all_indices.push(WindowIndex::new(
                file_path_hash,
                start_token as u32,
                end_token as u32,
                start_char as u32,
                end_char as u32,
            ));
        }
    }

    if all_embeddings.is_empty() {
        println!("No content to embed.");
        return Ok(());
    }

    let embedding_dim = all_embeddings
        .first()
        .map(|e| e.len())
        .unwrap_or(DEFAULT_EMBEDDING_DIM);
    let n_windows = all_embeddings.len();

    let embeddings_array = ndarray::Array2::from_shape_vec(
        (n_windows, embedding_dim),
        all_embeddings.into_iter().flatten().collect(),
    )
    .map_err(|e| crate::Error::Graph(format!("Failed to create embeddings array: {}", e)))?;

    let metadata = EmbeddingMetadata::new(
        model_id,
        embedding_dim,
        DEFAULT_WINDOW_SIZE,
        DEFAULT_OVERLAP,
        file_count,
    );

    let index = EmbeddingIndex {
        embeddings: embeddings_array,
        indices: all_indices,
        metadata,
        path_map,
    };

    if let Some(parent) = embeddings_path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    index.save(&embeddings_path)?;

    println!("\nEmbedding index built:");
    println!("  Files: {}", file_count);
    println!("  Windows: {}", n_windows);
    println!("  Dimension: {}", embedding_dim);
    println!("  Model: {}", model_id);
    println!("  Saved to: {}", embeddings_path.display());

    Ok(())
}

fn show_status(embeddings_path: &Path) -> crate::Result<()> {
    if !embeddings_path.exists() {
        println!("No embedding index found.");
        println!("Run 'taskgraph embed' to build one.");
        return Ok(());
    }

    let index = EmbeddingIndex::load(embeddings_path)?;

    println!("Embedding index status:");
    println!("  Path: {}", embeddings_path.display());
    println!("  Model: {}", index.metadata.model_id);
    println!("  Dimension: {}", index.metadata.embedding_dim);
    println!("  Window size: {}", index.metadata.window_size);
    println!("  Overlap: {:.0}%", index.metadata.overlap * 100.0);
    println!("  Files: {}", index.metadata.file_count);
    println!("  Windows: {}", index.indices.len());
    println!("  Created: {}", index.metadata.created_at);

    let file_size = std::fs::metadata(embeddings_path)
        .map(|m| m.len())
        .unwrap_or(0);
    println!("  File size: {:.2} KB", file_size as f64 / 1024.0);

    Ok(())
}
