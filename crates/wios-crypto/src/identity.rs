//! Certificate and identity management for node authentication.

use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use wios_core::error::{WiosError, WiosResult};
use wios_core::types::NodeId;

/// A node identity certificate.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeCertificate {
    /// Certificate serial number.
    pub serial: String,
    /// Node this certificate belongs to.
    pub node_id: NodeId,
    /// Ed25519 public key bytes (32 bytes, hex-encoded).
    pub public_key: String,
    /// X25519 public key for key exchange (32 bytes, hex-encoded).
    pub kex_public_key: String,
    /// Human-readable common name.
    pub common_name: String,
    /// Issued at timestamp.
    pub issued_at: DateTime<Utc>,
    /// Expires at timestamp.
    pub expires_at: DateTime<Utc>,
    /// Issuer node ID (self-signed if same as node_id).
    pub issuer: NodeId,
    /// Signature over the certificate fields (hex-encoded).
    pub signature: String,
    /// Whether this certificate has been revoked.
    pub revoked: bool,
}

impl NodeCertificate {
    /// Check if the certificate is currently valid (not expired, not revoked).
    pub fn is_valid(&self) -> bool {
        let now = Utc::now();
        !self.revoked && now >= self.issued_at && now <= self.expires_at
    }

    /// Check if this is a self-signed certificate.
    pub fn is_self_signed(&self) -> bool {
        self.node_id == self.issuer
    }

    /// Remaining validity in seconds.
    pub fn remaining_validity_secs(&self) -> i64 {
        (self.expires_at - Utc::now()).num_seconds().max(0)
    }
}

/// Manages node identity certificates.
pub struct CertificateManager {
    certificates: Arc<RwLock<HashMap<String, NodeCertificate>>>,
    /// Revocation list (serial numbers).
    revocation_list: Arc<RwLock<Vec<String>>>,
}

impl CertificateManager {
    pub fn new() -> Self {
        Self {
            certificates: Arc::new(RwLock::new(HashMap::new())),
            revocation_list: Arc::new(RwLock::new(Vec::new())),
        }
    }

    /// Issue a self-signed certificate for a node.
    pub async fn issue_self_signed(
        &self,
        node_id: &NodeId,
        common_name: &str,
        public_key_hex: &str,
        kex_public_key_hex: &str,
        validity_days: u32,
        sign_fn: impl FnOnce(&[u8]) -> WiosResult<Vec<u8>>,
    ) -> WiosResult<NodeCertificate> {
        let serial = uuid::Uuid::new_v4().to_string();
        let now = Utc::now();
        let expires = now + Duration::days(validity_days as i64);

        // Create certificate data to sign
        let cert_data = format!(
            "{}:{}:{}:{}:{}:{}",
            serial,
            node_id,
            public_key_hex,
            common_name,
            now.to_rfc3339(),
            expires.to_rfc3339()
        );
        let sig_bytes = sign_fn(cert_data.as_bytes())?;
        let signature = sig_bytes.iter().map(|b| format!("{:02x}", b)).collect::<String>();

        let cert = NodeCertificate {
            serial: serial.clone(),
            node_id: node_id.clone(),
            public_key: public_key_hex.to_string(),
            kex_public_key: kex_public_key_hex.to_string(),
            common_name: common_name.to_string(),
            issued_at: now,
            expires_at: expires,
            issuer: node_id.clone(),
            signature,
            revoked: false,
        };

        self.certificates.write().await.insert(serial, cert.clone());
        Ok(cert)
    }

    /// Store an externally-received certificate.
    pub async fn store(&self, cert: NodeCertificate) -> WiosResult<()> {
        // Check revocation list
        let revoked = self.revocation_list.read().await;
        if revoked.contains(&cert.serial) {
            return Err(WiosError::Crypto("Certificate is revoked".into()));
        }
        drop(revoked);

        self.certificates.write().await.insert(cert.serial.clone(), cert);
        Ok(())
    }

    /// Get a certificate by serial.
    pub async fn get(&self, serial: &str) -> WiosResult<NodeCertificate> {
        self.certificates.read().await
            .get(serial)
            .cloned()
            .ok_or_else(|| WiosError::Crypto(format!("Certificate not found: {}", serial)))
    }

    /// Get a valid certificate for a node.
    pub async fn get_for_node(&self, node_id: &NodeId) -> Option<NodeCertificate> {
        self.certificates.read().await
            .values()
            .find(|c| c.node_id == *node_id && c.is_valid())
            .cloned()
    }

    /// Revoke a certificate.
    pub async fn revoke(&self, serial: &str) -> WiosResult<()> {
        let mut certs = self.certificates.write().await;
        if let Some(cert) = certs.get_mut(serial) {
            cert.revoked = true;
            drop(certs);
            self.revocation_list.write().await.push(serial.to_string());
            Ok(())
        } else {
            Err(WiosError::Crypto(format!("Certificate not found: {}", serial)))
        }
    }

    /// List all valid certificates.
    pub async fn list_valid(&self) -> Vec<NodeCertificate> {
        self.certificates.read().await
            .values()
            .filter(|c| c.is_valid())
            .cloned()
            .collect()
    }

    /// Clean up expired certificates.
    pub async fn cleanup_expired(&self) -> usize {
        let mut certs = self.certificates.write().await;
        let expired: Vec<String> = certs
            .iter()
            .filter(|(_, c)| !c.is_valid())
            .map(|(k, _)| k.clone())
            .collect();
        let count = expired.len();
        for serial in expired {
            certs.remove(&serial);
        }
        count
    }
}

impl Default for CertificateManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_self_signed_certificate() {
        let mgr = CertificateManager::new();
        let node = NodeId::new();

        let cert = mgr.issue_self_signed(
            &node,
            "test-node",
            "aabbccdd",
            "eeff0011",
            365,
            |data| Ok(data[..64.min(data.len())].to_vec()),
        ).await.unwrap();

        assert!(cert.is_valid());
        assert!(cert.is_self_signed());
        assert!(cert.remaining_validity_secs() > 0);
    }

    #[tokio::test]
    async fn test_revoke_certificate() {
        let mgr = CertificateManager::new();
        let node = NodeId::new();

        let cert = mgr.issue_self_signed(
            &node, "test", "aa", "bb", 365,
            |data| Ok(data[..32.min(data.len())].to_vec()),
        ).await.unwrap();

        mgr.revoke(&cert.serial).await.unwrap();
        let retrieved = mgr.get(&cert.serial).await.unwrap();
        assert!(!retrieved.is_valid());
    }

    #[tokio::test]
    async fn test_get_for_node() {
        let mgr = CertificateManager::new();
        let node = NodeId::new();

        mgr.issue_self_signed(
            &node, "test", "aa", "bb", 365,
            |data| Ok(data[..32.min(data.len())].to_vec()),
        ).await.unwrap();

        assert!(mgr.get_for_node(&node).await.is_some());
        assert!(mgr.get_for_node(&NodeId::new()).await.is_none());
    }
}
