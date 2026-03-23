//! Embedding-based semantic search for task descriptions.
//!
//! This module provides rolling window embeddings for long text documents,
//! stored in safetensor format for efficient similarity search.

use std::path::Path;

/// Embedding dimension for the default model (potion-base-8M).
pub const DEFAULT_EMBEDDING_DIM: usize = 256;

/// Default window size in tokens.
pub const DEFAULT_WINDOW_SIZE: usize = 512;

/// Default overlap between windows (50%).
pub const DEFAULT_OVERLAP: f32 = 0.5;

/// Default model ID.
pub const DEFAULT_MODEL_ID: &str = "minishlab/potion-base-8M";

/// Index entry for a single embedding window.
///
/// 24 bytes packed structure for efficient storage.
#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg(feature = "semantic")]
pub struct WindowIndex {
    /// xxHash3 hash of the file path.
    pub file_path_hash: u64,
    /// Start token position in the document.
    pub start_token: u32,
    /// End token position in the document.
    pub end_token: u32,
    /// Start character offset in the document.
    pub start_char: u32,
    /// End character offset in the document.
    pub end_char: u32,
}

#[cfg(feature = "semantic")]
impl WindowIndex {
    /// Create a new window index.
    pub fn new(
        file_path_hash: u64,
        start_token: u32,
        end_token: u32,
        start_char: u32,
        end_char: u32,
    ) -> Self {
        Self {
            file_path_hash,
            start_token,
            end_token,
            start_char,
            end_char,
        }
    }

    /// Hash a file path using xxHash3.
    pub fn hash_path(path: &str) -> u64 {
        twox_hash::xxhash3_64::Hasher::oneshot(path.as_bytes())
    }

    /// Pack the struct into a byte array.
    pub fn to_bytes(&self) -> [u8; 24] {
        let mut bytes = [0u8; 24];
        bytes[0..8].copy_from_slice(&self.file_path_hash.to_le_bytes());
        bytes[8..12].copy_from_slice(&self.start_token.to_le_bytes());
        bytes[12..16].copy_from_slice(&self.end_token.to_le_bytes());
        bytes[16..20].copy_from_slice(&self.start_char.to_le_bytes());
        bytes[20..24].copy_from_slice(&self.end_char.to_le_bytes());
        bytes
    }

    /// Unpack from a byte array.
    pub fn from_bytes(bytes: [u8; 24]) -> Self {
        Self {
            file_path_hash: u64::from_le_bytes(bytes[0..8].try_into().unwrap()),
            start_token: u32::from_le_bytes(bytes[8..12].try_into().unwrap()),
            end_token: u32::from_le_bytes(bytes[12..16].try_into().unwrap()),
            start_char: u32::from_le_bytes(bytes[16..20].try_into().unwrap()),
            end_char: u32::from_le_bytes(bytes[20..24].try_into().unwrap()),
        }
    }
}

/// Metadata stored in the safetensor file.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[cfg(feature = "semantic")]
pub struct EmbeddingMetadata {
    /// Model ID used for embeddings.
    pub model_id: String,
    /// Embedding dimension.
    pub embedding_dim: usize,
    /// Window size in tokens.
    pub window_size: usize,
    /// Overlap between windows.
    pub overlap: f32,
    /// Creation timestamp (ISO 8601).
    pub created_at: String,
    /// Number of files embedded.
    pub file_count: usize,
}

#[cfg(feature = "semantic")]
impl EmbeddingMetadata {
    /// Create new metadata with current timestamp.
    pub fn new(
        model_id: &str,
        embedding_dim: usize,
        window_size: usize,
        overlap: f32,
        file_count: usize,
    ) -> Self {
        Self {
            model_id: model_id.to_string(),
            embedding_dim,
            window_size,
            overlap,
            created_at: chrono::Utc::now().to_rfc3339(),
            file_count,
        }
    }
}

/// Create rolling token windows from token IDs.
///
/// Returns a list of (start_token, end_token, start_char, end_char) tuples
/// along with the token ID chunks for each window.
#[cfg(feature = "semantic")]
pub fn create_rolling_windows(
    token_ids: &[u32],
    token_offsets: &[usize],
    window_size: usize,
    overlap: f32,
) -> Vec<(Vec<u32>, usize, usize, usize, usize)> {
    if token_ids.is_empty() {
        return Vec::new();
    }

    let total_tokens = token_ids.len();

    if total_tokens <= window_size {
        let start_char = token_offsets.first().copied().unwrap_or(0);
        let end_char = token_offsets.last().copied().unwrap_or(0);
        return vec![(token_ids.to_vec(), 0, total_tokens, start_char, end_char)];
    }

    let overlap_tokens = (window_size as f32 * overlap) as usize;
    let step_size = window_size - overlap_tokens;

    let mut windows = Vec::new();
    let mut start_idx = 0;

    while start_idx < total_tokens {
        let end_idx = (start_idx + window_size).min(total_tokens);
        let window_tokens: Vec<u32> = token_ids[start_idx..end_idx].to_vec();

        let start_char = token_offsets.get(start_idx).copied().unwrap_or(0);
        let end_char = token_offsets
            .get(end_idx.saturating_sub(1))
            .copied()
            .unwrap_or(0);

        windows.push((window_tokens, start_idx, end_idx, start_char, end_char));

        if end_idx >= total_tokens {
            break;
        }

        start_idx += step_size;
    }

    windows
}

/// Embedding index containing all window embeddings and their metadata.
#[cfg(feature = "semantic")]
pub struct EmbeddingIndex {
    /// Embedding vectors [N, D].
    pub embeddings: ndarray::Array2<f32>,
    /// Window indices [N].
    pub indices: Vec<WindowIndex>,
    /// Metadata.
    pub metadata: EmbeddingMetadata,
    /// Hash to file path mapping.
    pub path_map: std::collections::HashMap<u64, String>,
}

#[cfg(feature = "semantic")]
impl EmbeddingIndex {
    /// Create a new empty index.
    pub fn new(model_id: &str, embedding_dim: usize) -> Self {
        Self {
            embeddings: ndarray::Array2::zeros((0, embedding_dim)),
            indices: Vec::new(),
            metadata: EmbeddingMetadata::new(
                model_id,
                embedding_dim,
                DEFAULT_WINDOW_SIZE,
                DEFAULT_OVERLAP,
                0,
            ),
            path_map: std::collections::HashMap::new(),
        }
    }

    /// Load index from a safetensor file.
    pub fn load(path: &Path) -> crate::Result<Self> {
        let buffer = std::fs::read(path)?;
        let tensors = safetensors::SafeTensors::deserialize(&buffer)
            .map_err(|e| crate::Error::Graph(format!("Failed to load embeddings: {}", e)))?;

        let metadata_tensor = tensors.tensor("__metadata__").ok();
        let metadata: EmbeddingMetadata = if let Some(tensor) = metadata_tensor {
            let meta_str = std::str::from_utf8(tensor.data())
                .map_err(|e| crate::Error::Graph(format!("Invalid metadata: {}", e)))?;
            serde_json::from_str(meta_str)
                .map_err(|e| crate::Error::Graph(format!("Failed to parse metadata: {}", e)))?
        } else {
            EmbeddingMetadata::new(
                DEFAULT_MODEL_ID,
                DEFAULT_EMBEDDING_DIM,
                DEFAULT_WINDOW_SIZE,
                DEFAULT_OVERLAP,
                0,
            )
        };

        let embeddings_tensor = tensors
            .tensor("embeddings")
            .map_err(|e| crate::Error::Graph(format!("Embeddings tensor not found: {}", e)))?;

        let shape = embeddings_tensor.shape();
        if shape.len() != 2 {
            return Err(crate::Error::Graph(format!(
                "Invalid embeddings shape: {:?}",
                shape
            )));
        }
        let n_windows = shape[0];
        let embedding_dim = shape[1];

        let embeddings_data = embeddings_tensor.data();
        let n_elements = n_windows * embedding_dim;
        let mut embeddings_vec = Vec::with_capacity(n_elements);
        for chunk in embeddings_data.chunks_exact(4) {
            let bytes: [u8; 4] = chunk.try_into().unwrap();
            embeddings_vec.push(f32::from_le_bytes(bytes));
        }
        let embeddings: ndarray::Array2<f32> =
            ndarray::Array::from_shape_vec((n_windows, embedding_dim), embeddings_vec)
                .map_err(|e| crate::Error::Graph(format!("Failed to reshape embeddings: {}", e)))?;

        let index_tensor = tensors
            .tensor("index")
            .map_err(|e| crate::Error::Graph(format!("Index tensor not found: {}", e)))?;

        let index_data = index_tensor.data();
        let n_indices = index_data.len() / 24;

        let mut indices = Vec::with_capacity(n_indices);
        for i in 0..n_indices {
            let offset = i * 24;
            let mut bytes = [0u8; 24];
            bytes.copy_from_slice(&index_data[offset..offset + 24]);
            indices.push(WindowIndex::from_bytes(bytes));
        }

        let path_map_tensor = tensors.tensor("path_map").ok();
        let path_map: std::collections::HashMap<u64, String> = if let Some(tensor) = path_map_tensor
        {
            let map_str = std::str::from_utf8(tensor.data())
                .map_err(|e| crate::Error::Graph(format!("Invalid path_map: {}", e)))?;
            serde_json::from_str(map_str).unwrap_or_default()
        } else {
            std::collections::HashMap::new()
        };

        Ok(Self {
            embeddings,
            indices,
            metadata,
            path_map,
        })
    }

    /// Save index to a safetensor file.
    pub fn save(&self, path: &Path) -> crate::Result<()> {
        let metadata_json = serde_json::to_string(&self.metadata)
            .map_err(|e| crate::Error::Graph(format!("Failed to serialize metadata: {}", e)))?;

        let path_map_json = serde_json::to_string(&self.path_map)
            .map_err(|e| crate::Error::Graph(format!("Failed to serialize path_map: {}", e)))?;

        let n_windows = self.embeddings.nrows();
        let embedding_dim = self.embeddings.ncols();

        let embeddings_data: Vec<u8> = self
            .embeddings
            .iter()
            .flat_map(|&f| f.to_le_bytes())
            .collect();

        let index_data: Vec<u8> = self.indices.iter().flat_map(|idx| idx.to_bytes()).collect();

        let path_map_bytes = path_map_json.into_bytes();

        let tensors: Vec<(String, safetensors::tensor::TensorView)> = vec![
            (
                "embeddings".to_string(),
                safetensors::tensor::TensorView::new(
                    safetensors::tensor::Dtype::F32,
                    vec![n_windows, embedding_dim],
                    &embeddings_data,
                )
                .map_err(|e| {
                    crate::Error::Graph(format!("Failed to create embeddings view: {}", e))
                })?,
            ),
            (
                "index".to_string(),
                safetensors::tensor::TensorView::new(
                    safetensors::tensor::Dtype::U8,
                    vec![n_windows, 24],
                    &index_data,
                )
                .map_err(|e| crate::Error::Graph(format!("Failed to create index view: {}", e)))?,
            ),
            (
                "path_map".to_string(),
                safetensors::tensor::TensorView::new(
                    safetensors::tensor::Dtype::U8,
                    vec![path_map_bytes.len()],
                    &path_map_bytes,
                )
                .map_err(|e| {
                    crate::Error::Graph(format!("Failed to create path_map view: {}", e))
                })?,
            ),
        ];

        let mut meta_map = std::collections::HashMap::new();
        meta_map.insert("__metadata__".to_string(), metadata_json);

        safetensors::tensor::serialize_to_file(tensors, &Some(meta_map), path)
            .map_err(|e| crate::Error::Graph(format!("Failed to save embeddings: {}", e)))?;

        Ok(())
    }
}

/// Search result for a query.
#[derive(Debug, Clone)]
#[cfg(feature = "semantic")]
pub struct SearchResult {
    /// File path hash.
    pub file_path_hash: u64,
    /// File path (if available in path_map).
    pub file_path: Option<String>,
    /// Similarity score.
    pub score: f32,
    /// Window index.
    pub window: WindowIndex,
}

/// Compute cosine similarity between two vectors.
#[cfg(feature = "semantic")]
pub fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
    if a.len() != b.len() || a.is_empty() {
        return 0.0;
    }

    let dot: f32 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
    let norm_a: f32 = a.iter().map(|x| x * x).sum::<f32>().sqrt();
    let norm_b: f32 = b.iter().map(|x| x * x).sum::<f32>().sqrt();

    if norm_a == 0.0 || norm_b == 0.0 {
        return 0.0;
    }

    dot / (norm_a * norm_b)
}

#[cfg(all(test, feature = "semantic"))]
mod tests {
    use super::*;

    #[test]
    fn test_window_index_roundtrip() {
        let idx = WindowIndex::new(12345, 0, 512, 0, 1024);
        let bytes = idx.to_bytes();
        let restored = WindowIndex::from_bytes(bytes);
        assert_eq!(idx, restored);
    }

    #[test]
    fn test_hash_path() {
        let path = "tasks/my-task.md";
        let hash1 = WindowIndex::hash_path(path);
        let hash2 = WindowIndex::hash_path(path);
        assert_eq!(hash1, hash2);

        let hash3 = WindowIndex::hash_path("tasks/other-task.md");
        assert_ne!(hash1, hash3);
    }

    #[test]
    fn test_rolling_windows_empty() {
        let windows = create_rolling_windows(&[], &[], 512, 0.5);
        assert!(windows.is_empty());
    }

    #[test]
    fn test_rolling_windows_single() {
        let token_ids: Vec<u32> = (0..100).collect();
        let offsets: Vec<usize> = (0..100).collect();
        let windows = create_rolling_windows(&token_ids, &offsets, 512, 0.5);
        assert_eq!(windows.len(), 1);
        assert_eq!(windows[0].1, 0);
        assert_eq!(windows[0].2, 100);
    }

    #[test]
    fn test_rolling_windows_overlap() {
        let token_ids: Vec<u32> = (0..1000).collect();
        let offsets: Vec<usize> = (0..1000).collect();
        let windows = create_rolling_windows(&token_ids, &offsets, 512, 0.5);

        assert!(windows.len() > 1);

        for i in 0..windows.len() - 1 {
            let current_end = windows[i].2;
            let next_start = windows[i + 1].1;
            assert!(next_start < current_end, "Windows should overlap");
        }
    }

    #[test]
    fn test_cosine_similarity() {
        let a = vec![1.0, 0.0, 0.0];
        let b = vec![1.0, 0.0, 0.0];
        assert!((cosine_similarity(&a, &b) - 1.0).abs() < 1e-6);

        let c = vec![0.0, 1.0, 0.0];
        assert!(cosine_similarity(&a, &c).abs() < 1e-6);

        let d = vec![0.707, 0.707, 0.0];
        assert!((cosine_similarity(&a, &d) - 0.707).abs() < 0.01);
    }
}
