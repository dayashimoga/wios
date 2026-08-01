//! End-to-end tests verifying cross-crate flows.

#[cfg(test)]
mod e2e {
    use wios_core::config::WiosConfig;
    use wios_core::types::NodeId;

    /// E2E: Full node lifecycle — init, configure, encrypt, store, retrieve, verify.
    #[tokio::test]
    async fn test_node_lifecycle() {
        // 1. Create config
        let config = WiosConfig::default();
        assert!(!config.node.name.is_empty());

        // 2. Generate identity
        let node_id = NodeId::new();
        assert!(!node_id.as_str().is_empty());

        // 3. Encrypt data
        let key = wios_crypto::encryption::EncryptionService::generate_key();
        let plaintext = b"WIOS node secret data";
        let encrypted =
            wios_crypto::encryption::EncryptionService::encrypt(&key, plaintext).unwrap();
        assert_ne!(encrypted, plaintext.to_vec());

        // 4. Store in SQLite
        let store = wios_storage::SqliteStore::open_in_memory().unwrap();
        store
            .put("default", "node_secret", &encrypted)
            .await
            .unwrap();

        // 5. Retrieve and decrypt
        let retrieved = store.get("default", "node_secret").await.unwrap().unwrap();
        let decrypted =
            wios_crypto::encryption::EncryptionService::decrypt(&key, &retrieved).unwrap();
        assert_eq!(decrypted, plaintext);
    }

    /// E2E: Crypto pipeline — keygen, sign, verify, encrypt, exchange.
    #[test]
    fn test_crypto_pipeline() {
        // Generate signing keypair (private, public)
        let (priv_key, pub_key) = wios_crypto::signing::SigningService::generate_keypair();

        // Sign data
        let data = b"Critical mesh message";
        let signature = wios_crypto::signing::SigningService::sign(&priv_key, data).unwrap();

        // Verify signature
        assert!(wios_crypto::signing::SigningService::verify(&pub_key, data, &signature).unwrap());

        // Tampered data fails
        assert!(
            !wios_crypto::signing::SigningService::verify(&pub_key, b"Tampered", &signature)
                .unwrap()
        );

        // Key exchange
        let kp_a = wios_crypto::kex::KeyExchange::generate_keypair();
        let kp_b = wios_crypto::kex::KeyExchange::generate_keypair();
        let shared_a =
            wios_crypto::kex::KeyExchange::derive_shared_secret(&kp_a.secret, &kp_b.public)
                .unwrap();
        let shared_b =
            wios_crypto::kex::KeyExchange::derive_shared_secret(&kp_b.secret, &kp_a.public)
                .unwrap();
        assert_eq!(shared_a, shared_b);
    }

    /// E2E: Storage pipeline — chunk, version, backup.
    #[test]
    fn test_storage_pipeline() {
        // Chunk data
        let data = vec![42u8; 10_000];
        let mut engine = wios_storage::chunking::ChunkEngine::new(1024);
        let chunks = engine.chunk_data(&data);
        assert!(chunks.len() > 1);

        // Reassemble with hash verification
        let file_hash = {
            use ring::digest;
            let d = digest::digest(&digest::SHA256, &data);
            d.as_ref()
                .iter()
                .map(|b| format!("{:02x}", b))
                .collect::<String>()
        };
        let reassembled = engine.reassemble(&chunks, &file_hash).unwrap();
        assert_eq!(reassembled, data);

        // Version tracking
        let mut vmgr = wios_storage::VersionManager::new(10);
        let v1 = vmgr.commit(
            "file.bin",
            "hash1".into(),
            10000,
            "user".into(),
            "Initial".into(),
        );
        let v2 = vmgr.commit(
            "file.bin",
            "hash2".into(),
            10100,
            "user".into(),
            "Update".into(),
        );
        assert_eq!(v1, 1);
        assert_eq!(v2, 2);

        // Backup
        let mut bmgr = wios_storage::BackupManager::new(5);
        let snap_id = bmgr.create_snapshot("e2e-test".into(), vec!["file.bin".into()], 10100);
        assert!(!snap_id.is_empty());
        assert_eq!(bmgr.total_size(), 10100);
    }

    /// E2E: Network pipeline — SOS, compression, delta sync.
    #[test]
    fn test_network_pipeline() {
        // Compression roundtrip
        let data = vec![0xABu8; 500];
        let compressed = wios_network::compression::compress(
            &data,
            wios_network::compression::CompressionAlgo::Zstd,
        )
        .unwrap();
        let decompressed = wios_network::compression::decompress(
            &compressed,
            wios_network::compression::CompressionAlgo::Zstd,
        )
        .unwrap();
        assert_eq!(decompressed, data);

        // Delta sync — use larger data where delta is smaller than full copy
        let mut source = vec![0u8; 1000];
        for (i, b) in source.iter_mut().enumerate() {
            *b = (i % 256) as u8;
        }
        let mut target = source.clone();
        target[500] = 0xFF; // Small change
        let delta = wios_network::delta_sync::compute_delta(&source, &target, 8);
        let result = wios_network::delta_sync::apply_delta(&source, &delta).unwrap();
        assert_eq!(result, target);
    }

    /// E2E: AI pipeline — anomaly detection + NLP + workflow.
    #[test]
    fn test_ai_pipeline() {
        // Anomaly detection
        let mut detector = wios_ai::AnomalyDetector::new(2.5, 20);
        for _ in 0..15 {
            assert!(detector.check("cpu_temp", 45.0).is_none());
        }
        let anomaly = detector.check("cpu_temp", 200.0);
        assert!(anomaly.is_some());

        // NLP
        let nlp = wios_ai::NlpProcessor::new();
        let intent = nlp.parse("Scan the network");
        assert_eq!(intent.intent, "scan_network");
        assert!(intent.confidence > 0.5);

        // Workflow
        let mut wf = wios_ai::WorkflowEngine::new();
        wf.add_step("a".into(), "fetch".into(), vec![]);
        wf.add_step("b".into(), "process".into(), vec!["a".into()]);
        assert_eq!(wf.ready_steps().len(), 1);
        wf.complete_step("a");
        assert_eq!(wf.ready_steps().len(), 1);
        wf.complete_step("b");
        assert!(wf.is_complete());
    }
}
