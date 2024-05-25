use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs;

#[derive(Serialize, Deserialize)]
pub struct PeerInfo {
    pub address: String,
    pub success: bool,
}

pub struct PeerManager {
    file_path: String,
    peers: Vec<PeerInfo>,
}

impl PeerManager {
    pub fn new(file_path: &str) -> Self {
        Self {
            file_path: file_path.to_string(),
            peers: vec![],
        }
    }

    pub fn load(&mut self) -> Result<(), Box<dyn Error>> {
        let json_content = fs::read_to_string(&self.file_path)?;
        self.peers = serde_json::from_str(&json_content)?;
        Ok(())
    }

    pub fn save(&self) -> Result<(), Box<dyn Error>> {
        let json_content = serde_json::to_string(&self.peers)?;
        fs::write(&self.file_path, json_content)?;
        Ok(())
    }

    pub fn add_peer(&mut self, address: String, success: bool) {
        self.peers.push(PeerInfo { address, success });
    }

    pub fn refresh(&mut self) -> Result<(), Box<dyn Error>> {
        self.peers.clear();
        self.save()?;
        Ok(())
    }
}
