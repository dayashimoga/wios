//! Compression utilities for file transfer and messaging.

use wios_core::error::{WiosError, WiosResult};

/// Compression algorithm selection.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompressionAlgo {
    /// No compression
    None,
    /// Fast compression, moderate ratio
    Lz4,
    /// High compression ratio
    Zstd,
}

/// Compress data using the selected algorithm.
/// Uses a simple run-length encoding as a pure-Rust fallback.
pub fn compress(data: &[u8], algo: CompressionAlgo) -> WiosResult<Vec<u8>> {
    match algo {
        CompressionAlgo::None => Ok(data.to_vec()),
        CompressionAlgo::Lz4 | CompressionAlgo::Zstd => {
            // Pure Rust RLE-based compression (production would use lz4/zstd crates)
            Ok(rle_compress(data))
        }
    }
}

/// Decompress data.
pub fn decompress(data: &[u8], algo: CompressionAlgo) -> WiosResult<Vec<u8>> {
    match algo {
        CompressionAlgo::None => Ok(data.to_vec()),
        CompressionAlgo::Lz4 | CompressionAlgo::Zstd => {
            rle_decompress(data).map_err(|e| WiosError::Storage(e))
        }
    }
}

/// Estimate compression ratio without full compression.
pub fn estimate_ratio(data: &[u8]) -> f64 {
    if data.is_empty() { return 1.0; }
    let unique_bytes: std::collections::HashSet<u8> = data.iter().cloned().collect();
    let entropy = unique_bytes.len() as f64 / 256.0;
    // Higher entropy = less compressible
    0.3 + 0.7 * entropy
}

/// Simple RLE compression: [marker, byte, count] for runs >= 4.
fn rle_compress(data: &[u8]) -> Vec<u8> {
    const MARKER: u8 = 0xFF;
    let mut out = Vec::with_capacity(data.len());
    let mut i = 0;
    while i < data.len() {
        let byte = data[i];
        let mut run = 1usize;
        while i + run < data.len() && data[i + run] == byte && run < 255 {
            run += 1;
        }
        if run >= 4 {
            out.push(MARKER);
            out.push(byte);
            out.push(run as u8);
            i += run;
        } else {
            if byte == MARKER {
                out.push(MARKER);
                out.push(byte);
                out.push(1);
            } else {
                out.push(byte);
            }
            i += 1;
        }
    }
    out
}

/// RLE decompression.
fn rle_decompress(data: &[u8]) -> Result<Vec<u8>, String> {
    const MARKER: u8 = 0xFF;
    let mut out = Vec::new();
    let mut i = 0;
    while i < data.len() {
        if data[i] == MARKER {
            if i + 2 >= data.len() {
                return Err("Invalid RLE: truncated marker sequence".into());
            }
            let byte = data[i + 1];
            let count = data[i + 2] as usize;
            out.extend(std::iter::repeat(byte).take(count));
            i += 3;
        } else {
            out.push(data[i]);
            i += 1;
        }
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compress_decompress_none() {
        let data = b"hello world";
        let compressed = compress(data, CompressionAlgo::None).unwrap();
        assert_eq!(compressed, data);
    }

    #[test]
    fn test_compress_decompress_rle() {
        let data = vec![0u8; 100]; // Highly compressible
        let compressed = compress(&data, CompressionAlgo::Zstd).unwrap();
        assert!(compressed.len() < data.len());

        let decompressed = decompress(&compressed, CompressionAlgo::Zstd).unwrap();
        assert_eq!(decompressed, data);
    }

    #[test]
    fn test_roundtrip_mixed_data() {
        let mut data = Vec::new();
        data.extend_from_slice(b"ABC");
        data.extend(vec![0x42; 20]);
        data.extend_from_slice(b"XYZ");
        data.extend(vec![0x00; 50]);

        let compressed = compress(&data, CompressionAlgo::Lz4).unwrap();
        let decompressed = decompress(&compressed, CompressionAlgo::Lz4).unwrap();
        assert_eq!(decompressed, data);
    }

    #[test]
    fn test_estimate_ratio() {
        let uniform = vec![0u8; 100];
        let random: Vec<u8> = (0..=255).collect();

        assert!(estimate_ratio(&uniform) < estimate_ratio(&random));
    }
}
