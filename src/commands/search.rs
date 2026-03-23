//! Search tasks by semantic similarity.

use std::path::Path;

use crate::embedding::{cosine_similarity, EmbeddingIndex, DEFAULT_MODEL_ID};

pub fn execute(
    tasks_path: &Path,
    query: &str,
    top_k: usize,
    model_id: Option<&str>,
) -> crate::Result<()> {
    let model_id = model_id.unwrap_or(DEFAULT_MODEL_ID);

    let embeddings_path = tasks_path
        .parent()
        .unwrap_or(tasks_path)
        .join(".taskgraph/embeddings.safetensors");

    if !embeddings_path.exists() {
        eprintln!("No embedding index found. Run 'taskgraph embed' first.");
        return Ok(());
    }

    let index = EmbeddingIndex::load(&embeddings_path)?;

    println!("Loading model {}...", model_id);
    let model = model2vec_rs::model::StaticModel::from_pretrained(model_id, None, None, None)
        .map_err(|e| crate::Error::Graph(format!("Failed to load model: {}", e)))?;

    let query_embedding = model.encode_single(query);

    let mut results: Vec<(f32, u64, u32, u32, Option<String>)> = index
        .indices
        .iter()
        .zip(index.embeddings.outer_iter())
        .map(|(window, embedding)| {
            let score = cosine_similarity(&query_embedding, embedding.as_slice().unwrap());
            let path = index.path_map.get(&window.file_path_hash).cloned();
            (
                score,
                window.file_path_hash,
                window.start_token,
                window.end_token,
                path,
            )
        })
        .collect();

    results.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));

    let mut seen_tasks = std::collections::HashSet::new();
    let mut count = 0;

    println!("\nSearch results for: \"{}\"\n", query);
    println!("{:<10} {:<40} TOKENS", "SCORE", "FILE");
    println!("{}", "-".repeat(70));

    for (score, file_path_hash, start_token, end_token, path) in results {
        if count >= top_k {
            break;
        }

        let task_id = path
            .as_ref()
            .map(|p| {
                p.rsplit('/')
                    .next()
                    .unwrap_or(p)
                    .trim_end_matches(".md")
                    .to_string()
            })
            .unwrap_or_else(|| format!("hash:{}", file_path_hash));

        if seen_tasks.contains(&task_id) {
            continue;
        }
        seen_tasks.insert(task_id.clone());

        println!(
            "{:<10.4} {:<40} {}-{}",
            score,
            path.as_deref().unwrap_or(&task_id),
            start_token,
            end_token
        );
        count += 1;
    }

    if count == 0 {
        println!("No results found.");
    }

    Ok(())
}
