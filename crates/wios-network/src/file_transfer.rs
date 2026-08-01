//! File transfer protocol — chunked, resumable, encrypted.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use wios_core::error::{WiosError, WiosResult};
use wios_core::types::NodeId;

/// Transfer state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TransferState {
    Pending,
    InProgress,
    Paused,
    Completed,
    Failed,
    Cancelled,
}

/// A file transfer request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileTransfer {
    pub id: String,
    pub filename: String,
    pub total_bytes: u64,
    pub transferred_bytes: u64,
    pub chunk_size: usize,
    pub total_chunks: u32,
    pub completed_chunks: Vec<u32>,
    pub sender: NodeId,
    pub recipient: NodeId,
    pub state: TransferState,
    pub encrypted: bool,
}

impl FileTransfer {
    pub fn new(
        filename: String,
        total_bytes: u64,
        chunk_size: usize,
        sender: NodeId,
        recipient: NodeId,
    ) -> Self {
        let total_chunks = ((total_bytes as f64) / (chunk_size as f64)).ceil() as u32;
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            filename,
            total_bytes,
            transferred_bytes: 0,
            chunk_size,
            total_chunks,
            completed_chunks: Vec::new(),
            sender,
            recipient,
            state: TransferState::Pending,
            encrypted: true,
        }
    }

    /// Mark a chunk as received.
    pub fn ack_chunk(&mut self, chunk_index: u32) -> WiosResult<()> {
        if chunk_index >= self.total_chunks {
            return Err(WiosError::Storage(format!("Invalid chunk index: {}", chunk_index)));
        }
        if !self.completed_chunks.contains(&chunk_index) {
            self.completed_chunks.push(chunk_index);
            self.transferred_bytes = (self.completed_chunks.len() as u64) * (self.chunk_size as u64);
            self.transferred_bytes = self.transferred_bytes.min(self.total_bytes);
        }
        if self.completed_chunks.len() as u32 == self.total_chunks {
            self.state = TransferState::Completed;
        } else {
            self.state = TransferState::InProgress;
        }
        Ok(())
    }

    /// Get missing chunk indices for resumable transfer.
    pub fn missing_chunks(&self) -> Vec<u32> {
        (0..self.total_chunks)
            .filter(|i| !self.completed_chunks.contains(i))
            .collect()
    }

    /// Progress as 0.0–1.0.
    pub fn progress(&self) -> f64 {
        if self.total_chunks == 0 { return 0.0; }
        self.completed_chunks.len() as f64 / self.total_chunks as f64
    }
}

/// Transfer manager tracking all active transfers.
pub struct TransferManager {
    transfers: HashMap<String, FileTransfer>,
}

impl TransferManager {
    pub fn new() -> Self {
        Self { transfers: HashMap::new() }
    }

    pub fn initiate(&mut self, transfer: FileTransfer) -> String {
        let id = transfer.id.clone();
        self.transfers.insert(id.clone(), transfer);
        id
    }

    pub fn ack_chunk(&mut self, transfer_id: &str, chunk_index: u32) -> WiosResult<()> {
        let transfer = self.transfers.get_mut(transfer_id)
            .ok_or(WiosError::NotFound { entity: "transfer".into(), id: transfer_id.into() })?;
        transfer.ack_chunk(chunk_index)
    }

    pub fn get(&self, transfer_id: &str) -> Option<&FileTransfer> {
        self.transfers.get(transfer_id)
    }

    pub fn cancel(&mut self, transfer_id: &str) -> WiosResult<()> {
        let transfer = self.transfers.get_mut(transfer_id)
            .ok_or(WiosError::NotFound { entity: "transfer".into(), id: transfer_id.into() })?;
        transfer.state = TransferState::Cancelled;
        Ok(())
    }

    pub fn active_transfers(&self) -> Vec<&FileTransfer> {
        self.transfers.values()
            .filter(|t| matches!(t.state, TransferState::InProgress | TransferState::Pending))
            .collect()
    }
}

impl Default for TransferManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_transfer_lifecycle() {
        let sender = NodeId::new();
        let recipient = NodeId::new();
        let mut transfer = FileTransfer::new("test.bin".into(), 10_000, 4096, sender, recipient);

        assert_eq!(transfer.total_chunks, 3); // ceil(10000/4096) = 3
        assert_eq!(transfer.state, TransferState::Pending);
        assert_eq!(transfer.missing_chunks(), vec![0, 1, 2]);

        transfer.ack_chunk(0).unwrap();
        assert_eq!(transfer.state, TransferState::InProgress);
        assert!((transfer.progress() - 1.0 / 3.0).abs() < 0.01);

        transfer.ack_chunk(1).unwrap();
        transfer.ack_chunk(2).unwrap();
        assert_eq!(transfer.state, TransferState::Completed);
        assert!((transfer.progress() - 1.0).abs() < 0.001);
        assert!(transfer.missing_chunks().is_empty());
    }

    #[test]
    fn test_resumable_transfer() {
        let mut transfer = FileTransfer::new("big.zip".into(), 20_000, 4096, NodeId::new(), NodeId::new());
        // Simulate partial transfer (chunks 0, 2 received, 1, 3, 4 missing)
        transfer.ack_chunk(0).unwrap();
        transfer.ack_chunk(2).unwrap();
        assert_eq!(transfer.missing_chunks(), vec![1, 3, 4]);
    }

    #[test]
    fn test_transfer_manager() {
        let mut mgr = TransferManager::new();
        let transfer = FileTransfer::new("doc.pdf".into(), 8000, 4096, NodeId::new(), NodeId::new());
        let id = mgr.initiate(transfer);

        mgr.ack_chunk(&id, 0).unwrap();
        assert_eq!(mgr.active_transfers().len(), 1);

        mgr.cancel(&id).unwrap();
        assert_eq!(mgr.active_transfers().len(), 0);
    }
}
