use std::sync::Arc;

use anyhow::Result;

use crate::{
    domain::{
        repositories::adventurers::AdventurersReopsitory,
        value_objects::adventurer_model::RegisterAdventurerModel,
    },
    infrastructure::argon2_hashing,
};

pub struct AdventurersUseCase<T>
where
    // Send: Allows moving data between threads.
    // Sync: Allows data to be accessed safely by multiple threads simultaneously.
    T: AdventurersReopsitory + Send + Sync,
{
    adventurers_repository: Arc<T>,
}

impl<T> AdventurersUseCase<T>
where
    T: AdventurersReopsitory + Send + Sync,
{
    pub fn new(adventurers_repository: Arc<T>) -> Self {
        Self {
            adventurers_repository,
        }
    }

    pub async fn register(
        &self,
        mut register_adventurer_model: RegisterAdventurerModel,
    ) -> Result<i32> {
        let hashed_password = argon2_hashing::hash(register_adventurer_model.password.clone())?;
        register_adventurer_model.password = hashed_password;
        let register_entity = register_adventurer_model.to_entity();
        let adventurer_id = self
            .adventurers_repository
            .register(register_entity)
            .await?;

        Ok(adventurer_id)
    }
}
