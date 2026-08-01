//! Performance benchmarks for critical WIOS operations.

#[cfg(test)]
mod benchmarks {
    use std::time::Instant;

    /// Benchmark: AES-256-GCM encryption throughput.
    #[test]
    fn bench_encryption_throughput() {
        let key = wios_crypto::encryption::EncryptionService::generate_key();
        let data = vec![0u8; 1_000_000]; // 1 MB
        let iterations = 100;

        let start = Instant::now();
        for _ in 0..iterations {
            let _ = wios_crypto::encryption::EncryptionService::encrypt(&key, &data).unwrap();
        }
        let elapsed = start.elapsed();
        let throughput_mbps = (iterations as f64 * data.len() as f64) / elapsed.as_secs_f64() / 1_000_000.0;
        println!("Encryption throughput: {:.1} MB/s ({} iterations)", throughput_mbps, iterations);
        assert!(throughput_mbps > 10.0, "Encryption too slow: {:.1} MB/s", throughput_mbps);
    }

    /// Benchmark: Ed25519 signing throughput.
    #[test]
    fn bench_signing_throughput() {
        let (priv_key, _pub_key) = wios_crypto::signing::SigningService::generate_keypair();
        let data = b"Benchmark signing data for WIOS mesh messages";
        let iterations = 10_000;

        let start = Instant::now();
        for _ in 0..iterations {
            let _ = wios_crypto::signing::SigningService::sign(&priv_key, data).unwrap();
        }
        let elapsed = start.elapsed();
        let ops_per_sec = iterations as f64 / elapsed.as_secs_f64();
        println!("Signing throughput: {:.0} ops/s ({} iterations)", ops_per_sec, iterations);
        assert!(ops_per_sec > 1000.0, "Signing too slow: {:.0} ops/s", ops_per_sec);
    }

    /// Benchmark: Chunking throughput.
    #[test]
    fn bench_chunking_throughput() {
        let data = vec![42u8; 10_000_000]; // 10 MB
        let iterations = 10;

        let start = Instant::now();
        for _ in 0..iterations {
            let mut engine = wios_storage::chunking::ChunkEngine::new(65536);
            let chunks = engine.chunk_data(&data);
            let file_hash = {
                use ring::digest;
                let d = digest::digest(&digest::SHA256, &data);
                d.as_ref().iter().map(|b| format!("{:02x}", b)).collect::<String>()
            };
            let _ = engine.reassemble(&chunks, &file_hash).unwrap();
        }
        let elapsed = start.elapsed();
        let throughput_mbps = (iterations as f64 * data.len() as f64 * 2.0) / elapsed.as_secs_f64() / 1_000_000.0;
        println!("Chunking roundtrip: {:.1} MB/s ({} iterations)", throughput_mbps, iterations);
        assert!(throughput_mbps > 50.0, "Chunking too slow: {:.1} MB/s", throughput_mbps);
    }

    /// Benchmark: SQLite operations throughput.
    #[tokio::test]
    async fn bench_sqlite_throughput() {
        let store = wios_storage::SqliteStore::open_in_memory().unwrap();
        let iterations = 1_000;

        // Write benchmark
        let start = Instant::now();
        for i in 0..iterations {
            store.put("bench", &format!("key_{}", i), format!("value_{}", i).as_bytes()).await.unwrap();
        }
        let write_elapsed = start.elapsed();
        let writes_per_sec = iterations as f64 / write_elapsed.as_secs_f64();

        // Read benchmark
        let start = Instant::now();
        for i in 0..iterations {
            let _ = store.get("bench", &format!("key_{}", i)).await.unwrap();
        }
        let read_elapsed = start.elapsed();
        let reads_per_sec = iterations as f64 / read_elapsed.as_secs_f64();

        println!("SQLite writes: {:.0} ops/s, reads: {:.0} ops/s", writes_per_sec, reads_per_sec);
        assert!(writes_per_sec > 100.0, "SQLite writes too slow");
        assert!(reads_per_sec > 1000.0, "SQLite reads too slow");
    }

    /// Benchmark: Delta sync speed.
    #[test]
    fn bench_delta_sync() {
        let source: Vec<u8> = (0..100_000).map(|i| (i % 256) as u8).collect();
        let mut target = source.clone();
        // Modify 1% of data
        for i in (0..target.len()).step_by(100) {
            target[i] = 0xFF;
        }

        let iterations = 50;
        let start = Instant::now();
        for _ in 0..iterations {
            let delta = wios_network::delta_sync::compute_delta(&source, &target, 64);
            let _ = wios_network::delta_sync::apply_delta(&source, &delta).unwrap();
        }
        let elapsed = start.elapsed();
        let ops_per_sec = iterations as f64 / elapsed.as_secs_f64();
        println!("Delta sync: {:.0} ops/s on 100KB data", ops_per_sec);
        assert!(ops_per_sec > 5.0, "Delta sync too slow");
    }

    /// Benchmark: Compression throughput.
    #[test]
    fn bench_compression() {
        let data = vec![0xABu8; 1_000_000]; // 1 MB compressible data
        let iterations = 100;

        let start = Instant::now();
        for _ in 0..iterations {
            let compressed = wios_network::compression::compress(&data, wios_network::compression::CompressionAlgo::Zstd).unwrap();
            let _ = wios_network::compression::decompress(&compressed, wios_network::compression::CompressionAlgo::Zstd).unwrap();
        }
        let elapsed = start.elapsed();
        let throughput_mbps = (iterations as f64 * data.len() as f64 * 2.0) / elapsed.as_secs_f64() / 1_000_000.0;
        println!("Compression roundtrip: {:.1} MB/s", throughput_mbps);
        assert!(throughput_mbps > 50.0, "Compression too slow");
    }
}
