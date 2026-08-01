//! File chunking and deduplication engine.

use ring::digest;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use wios_core::error::{WiosError, WiosResult};

/// Default chunk size: 256 KB.
pub const DEFAULT_CHUNK_SIZE: usize = 256 * 1024;

/// A file chunk with content-addressable hash.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chunk {
    pub index: u32,
    pub hash: String,
    pub size: usize,
    pub data: Vec<u8>,
}

/// Metadata for a chunked file.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChunkedFile {
    pub file_id: String,
    pub original_name: String,
    pub total_size: u64,
    pub chunk_count: u32,
    pub chunk_hashes: Vec<String>,
    pub file_hash: String,
    pub mime_type: String,
    pub encrypted: bool,
}

/// Deduplication statistics.
#[derive(Debug, Clone, Default)]
pub struct DedupStats {
    pub total_chunks: u64,
    pub unique_chunks: u64,
    pub bytes_saved: u64,
}

/// File chunking and deduplication engine.
pub struct ChunkEngine {
    chunk_size: usize,
    /// Hash → reference count for deduplication.
    chunk_refs: HashMap<String, u32>,
}

impl ChunkEngine {
    pub fn new(chunk_size: usize) -> Self {
        Self {
            chunk_size: if chunk_size == 0 { DEFAULT_CHUNK_SIZE } else { chunk_size },
            chunk_refs: HashMap::new(),
        }
    }

    /// Split data into content-addressed chunks.
    pub fn chunk_data(&mut self, data: &[u8]) -> Vec<Chunk> {
        let mut chunks = Vec::new();
        let mut offset = 0;
        let mut index = 0u32;

        while offset < data.len() {
            let end = (offset + self.chunk_size).min(data.len());
            let chunk_data = &data[offset..end];
            let hash = sha256_hex(chunk_data);

            // Track references for dedup
            *self.chunk_refs.entry(hash.clone()).or_insert(0) += 1;

            chunks.push(Chunk {
                index,
                hash,
                size: chunk_data.len(),
                data: chunk_data.to_vec(),
            });

            offset = end;
            index += 1;
        }

        chunks
    }

    /// Create a ChunkedFile manifest from data.
    pub fn create_manifest(
        &mut self,
        file_id: &str,
        name: &str,
        mime_type: &str,
        data: &[u8],
    ) -> ChunkedFile {
        let file_hash = sha256_hex(data);
        let chunks = self.chunk_data(data);
        let chunk_hashes: Vec<String> = chunks.iter().map(|c| c.hash.clone()).collect();

        ChunkedFile {
            file_id: file_id.to_string(),
            original_name: name.to_string(),
            total_size: data.len() as u64,
            chunk_count: chunks.len() as u32,
            chunk_hashes,
            file_hash,
            mime_type: mime_type.to_string(),
            encrypted: false,
        }
    }

    /// Reassemble chunks into original data (verifying hashes).
    pub fn reassemble(&self, chunks: &[Chunk], expected_hash: &str) -> WiosResult<Vec<u8>> {
        let mut sorted: Vec<&Chunk> = chunks.iter().collect();
        sorted.sort_by_key(|c| c.index);

        let mut data = Vec::new();
        for chunk in &sorted {
            let hash = sha256_hex(&chunk.data);
            if hash != chunk.hash {
                return Err(WiosError::Storage(format!(
                    "Chunk {} hash mismatch: expected {}, got {}",
                    chunk.index, chunk.hash, hash
                )));
            }
            data.extend_from_slice(&chunk.data);
        }

        let file_hash = sha256_hex(&data);
        if file_hash != expected_hash {
            return Err(WiosError::Storage("File hash mismatch after reassembly".into()));
        }

        Ok(data)
    }

    /// Get deduplication statistics.
    pub fn dedup_stats(&self) -> DedupStats {
        let total: u64 = self.chunk_refs.values().map(|&v| v as u64).sum();
        let unique = self.chunk_refs.len() as u64;
        let duplicate_refs = total.saturating_sub(unique);
        DedupStats {
            total_chunks: total,
            unique_chunks: unique,
            bytes_saved: duplicate_refs * self.chunk_size as u64,
        }
    }
}

impl Default for ChunkEngine {
    fn default() -> Self {
        Self::new(DEFAULT_CHUNK_SIZE)
    }
}

fn sha256_hex(data: &[u8]) -> String {
    let d = digest::digest(&digest::SHA256, data);
    d.as_ref().iter().map(|b| format!("{:02x}", b)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chunk_and_reassemble() {
        let mut engine = ChunkEngine::new(16); // Small chunks for testing
        let data = b"Hello, WIOS! This is a file that should be chunked into pieces.";

        let manifest = engine.create_manifest("f1", "test.txt", "text/plain", data);
        assert!(manifest.chunk_count > 1);
        assert_eq!(manifest.total_size, data.len() as u64);

        let chunks = engine.chunk_data(data);
        let reassembled = engine.reassemble(&chunks, &manifest.file_hash).unwrap();
        assert_eq!(reassembled, data);
    }

    #[test]
    fn test_deduplication() {
        let mut engine = ChunkEngine::new(16);
        let data = b"aaaaaaaaaaaaaaaa"; // exactly 16 bytes
        engine.chunk_data(data);
        engine.chunk_data(data); // duplicate

        let stats = engine.dedup_stats();
        assert_eq!(stats.unique_chunks, 1);
        assert_eq!(stats.total_chunks, 2);
    }

    #[test]
    fn test_hash_mismatch() {
        let engine = ChunkEngine::new(16);
        let chunks = vec![Chunk {
            index: 0,
            hash: "wrong_hash".into(),
            size: 5,
            data: b"hello".to_vec(),
        }];
        assert!(engine.reassemble(&chunks, "any").is_err());
    }
}
