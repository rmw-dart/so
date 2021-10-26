use anyhow::Result;
use blake3::{hash, Hasher};
use ed25519_dalek_blake3::{Keypair, SecretKey};
use std::fs;
use std::path::PathBuf;

pub struct Site {
  pub root: PathBuf,
  pub signer: Keypair,
}

impl Site {
  pub fn new(root: String, secret: &[u8]) -> Result<Self> {
    let root: PathBuf = root.into();
    fs::create_dir_all(&root)?;
    let secret = SecretKey::from_bytes(secret)?;
    Self {
      root: root,
      signer: Keypair {
        public: (&secret).into(),
        secret: secret,
      },
    }
  }
  pub fn add() {}
}
