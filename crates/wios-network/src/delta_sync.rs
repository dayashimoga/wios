//! Delta sync — binary diff/patch for efficient data synchronization.

use serde::{Deserialize, Serialize};
use wios_core::error::{WiosError, WiosResult};

/// A delta operation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DeltaOp {
    /// Copy `len` bytes from the source starting at `offset`.
    Copy { offset: usize, len: usize },
    /// Insert new data.
    Insert(Vec<u8>),
}

/// A computed delta between two versions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Delta {
    pub source_len: usize,
    pub target_len: usize,
    pub ops: Vec<DeltaOp>,
}

impl Delta {
    /// Size of the delta in bytes (approximate).
    pub fn size(&self) -> usize {
        self.ops.iter().map(|op| match op {
            DeltaOp::Copy { .. } => 16, // metadata only
            DeltaOp::Insert(data) => data.len() + 8,
        }).sum()
    }

    /// Compression ratio vs sending full target.
    pub fn ratio(&self) -> f64 {
        if self.target_len == 0 { return 1.0; }
        self.size() as f64 / self.target_len as f64
    }
}

/// Compute a delta between source and target using a simple block-matching algorithm.
pub fn compute_delta(source: &[u8], target: &[u8], block_size: usize) -> Delta {
    let block_size = block_size.max(4);
    let mut ops = Vec::new();
    let mut ti = 0;

    // Build index of source blocks
    let mut source_blocks = std::collections::HashMap::new();
    for i in (0..source.len()).step_by(block_size) {
        let end = (i + block_size).min(source.len());
        let block = &source[i..end];
        source_blocks.entry(block.to_vec()).or_insert(i);
    }

    while ti < target.len() {
        let remaining = target.len() - ti;
        let check_len = block_size.min(remaining);
        let target_block = &target[ti..ti + check_len];

        if let Some(&src_offset) = source_blocks.get(target_block) {
            // Found matching block — try to extend the match
            let mut match_len = check_len;
            while ti + match_len < target.len()
                && src_offset + match_len < source.len()
                && target[ti + match_len] == source[src_offset + match_len]
            {
                match_len += 1;
            }
            ops.push(DeltaOp::Copy { offset: src_offset, len: match_len });
            ti += match_len;
        } else {
            // No match — find how far until next match
            let mut insert_end = ti + 1;
            while insert_end < target.len() {
                let rem = target.len() - insert_end;
                let cl = block_size.min(rem);
                if cl >= block_size && source_blocks.contains_key(&target[insert_end..insert_end + cl]) {
                    break;
                }
                insert_end += 1;
            }
            ops.push(DeltaOp::Insert(target[ti..insert_end].to_vec()));
            ti = insert_end;
        }
    }

    Delta {
        source_len: source.len(),
        target_len: target.len(),
        ops,
    }
}

/// Apply a delta to a source to produce the target.
pub fn apply_delta(source: &[u8], delta: &Delta) -> WiosResult<Vec<u8>> {
    let mut result = Vec::with_capacity(delta.target_len);
    for op in &delta.ops {
        match op {
            DeltaOp::Copy { offset, len } => {
                if *offset + *len > source.len() {
                    return Err(WiosError::Storage("Delta copy out of bounds".into()));
                }
                result.extend_from_slice(&source[*offset..*offset + *len]);
            }
            DeltaOp::Insert(data) => {
                result.extend_from_slice(data);
            }
        }
    }
    if result.len() != delta.target_len {
        return Err(WiosError::Storage(format!(
            "Delta result length mismatch: got {} expected {}", result.len(), delta.target_len
        )));
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_identical_files() {
        let data = b"Hello, World! This is a test file.";
        let delta = compute_delta(data, data, 8);
        assert!(delta.ratio() < 0.5); // Much smaller than full copy
        let result = apply_delta(data, &delta).unwrap();
        assert_eq!(result, data);
    }

    #[test]
    fn test_small_change() {
        let source = b"The quick brown fox jumps over the lazy dog.";
        let target = b"The quick brown cat jumps over the lazy dog.";
        let delta = compute_delta(source, target, 8);
        let result = apply_delta(source, &delta).unwrap();
        assert_eq!(result, target);
    }

    #[test]
    fn test_append() {
        let source = b"Hello";
        let target = b"Hello, World!";
        let delta = compute_delta(source, target, 4);
        let result = apply_delta(source, &delta).unwrap();
        assert_eq!(result, target);
    }

    #[test]
    fn test_completely_different() {
        let source = b"AAAA";
        let target = b"BBBB";
        let delta = compute_delta(source, target, 4);
        let result = apply_delta(source, &delta).unwrap();
        assert_eq!(result, target);
    }

    #[test]
    fn test_empty_target() {
        let source = b"Hello";
        let target = b"";
        let delta = compute_delta(source, target, 4);
        let result = apply_delta(source, &delta).unwrap();
        assert_eq!(result, target.to_vec());
    }
}
