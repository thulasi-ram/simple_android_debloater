// Simple Android Debloater
use anyhow;
use serde::{ser::Serializer, Serialize};

#[derive(Debug, thiserror::Error)]
pub enum SADError {
    #[error(transparent)]
    E(#[from] anyhow::Error),
}

impl Serialize for SADError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}
