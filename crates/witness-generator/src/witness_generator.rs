use anyhow::Result;
use async_trait::async_trait;

use crate::BlocksAndWitnesses;

/// Trait for generating blocks and witnesses.
#[async_trait]
pub trait WitnessGenerator {
    /// Generates `BlocksAndWitnesses`.
    async fn generate(&self) -> Result<Vec<BlocksAndWitnesses>>;
}
