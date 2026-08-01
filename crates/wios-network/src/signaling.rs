//! WebRTC signaling abstraction for voice/video channels.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use wios_core::error::{WiosError, WiosResult};
use wios_core::types::NodeId;

/// Signaling message types.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SignalMessage {
    Offer { sdp: String },
    Answer { sdp: String },
    IceCandidate { candidate: String, sdp_mid: String, sdp_mline_index: u32 },
    Hangup,
}

/// Call state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CallState {
    Idle,
    Ringing,
    Connecting,
    Connected,
    Ended,
}

/// A signaling session between two peers.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignalingSession {
    pub session_id: String,
    pub caller: NodeId,
    pub callee: NodeId,
    pub state: CallState,
    pub media_type: MediaType,
    pub signals: Vec<(String, SignalMessage)>, // (sender_id, message)
}

/// Media type for the call.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum MediaType {
    Audio,
    Video,
    Screen,
}

/// Signaling server managing call sessions.
pub struct SignalingServer {
    sessions: Arc<RwLock<HashMap<String, SignalingSession>>>,
}

impl SignalingServer {
    pub fn new() -> Self {
        Self { sessions: Arc::new(RwLock::new(HashMap::new())) }
    }

    /// Initiate a call.
    pub async fn initiate_call(
        &self,
        caller: NodeId,
        callee: NodeId,
        media_type: MediaType,
    ) -> WiosResult<String> {
        let session_id = uuid::Uuid::new_v4().to_string();
        let session = SignalingSession {
            session_id: session_id.clone(),
            caller,
            callee,
            state: CallState::Ringing,
            media_type,
            signals: Vec::new(),
        };
        self.sessions.write().await.insert(session_id.clone(), session);
        Ok(session_id)
    }

    /// Send a signaling message.
    pub async fn send_signal(
        &self,
        session_id: &str,
        sender: &str,
        message: SignalMessage,
    ) -> WiosResult<()> {
        let mut sessions = self.sessions.write().await;
        let session = sessions.get_mut(session_id)
            .ok_or(WiosError::NotFound { entity: "session".into(), id: session_id.into() })?;

        match &message {
            SignalMessage::Offer { .. } => session.state = CallState::Connecting,
            SignalMessage::Answer { .. } => session.state = CallState::Connected,
            SignalMessage::Hangup => session.state = CallState::Ended,
            _ => {}
        }

        session.signals.push((sender.to_string(), message));
        Ok(())
    }

    /// Get pending signals for a peer.
    pub async fn get_signals(&self, session_id: &str) -> WiosResult<Vec<(String, SignalMessage)>> {
        let sessions = self.sessions.read().await;
        let session = sessions.get(session_id)
            .ok_or(WiosError::NotFound { entity: "session".into(), id: session_id.into() })?;
        Ok(session.signals.clone())
    }

    /// End a call.
    pub async fn end_call(&self, session_id: &str) -> WiosResult<()> {
        let mut sessions = self.sessions.write().await;
        if let Some(session) = sessions.get_mut(session_id) {
            session.state = CallState::Ended;
        }
        Ok(())
    }

    /// Get active call count.
    pub async fn active_calls(&self) -> usize {
        self.sessions.read().await.values()
            .filter(|s| matches!(s.state, CallState::Ringing | CallState::Connecting | CallState::Connected))
            .count()
    }
}

impl Default for SignalingServer {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_call_lifecycle() {
        let server = SignalingServer::new();
        let caller = NodeId::new();
        let callee = NodeId::new();

        let sid = server.initiate_call(caller.clone(), callee, MediaType::Audio).await.unwrap();
        assert_eq!(server.active_calls().await, 1);

        server.send_signal(&sid, caller.as_str(), SignalMessage::Offer { sdp: "v=0...".into() }).await.unwrap();
        server.send_signal(&sid, "callee", SignalMessage::Answer { sdp: "v=0...".into() }).await.unwrap();

        let signals = server.get_signals(&sid).await.unwrap();
        assert_eq!(signals.len(), 2);

        server.end_call(&sid).await.unwrap();
        assert_eq!(server.active_calls().await, 0);
    }
}
