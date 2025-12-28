use std::sync::Arc;

use anyhow::Result;

use crate::{
    domain::{
        repositories::guild_commanders::GuildCommandersRepository,
        value_objects::guild_commander_model::RegisterGuildCommanderModel,
    },
    infrastructure::argon2_hashing,
};

pub struct GuildComanndersUseCase<T>
where
    // Send: Allows moving data between threads.
    // Sync: Allows data to be accessed safely by multiple threads simultaneously.
    T: GuildCommandersRepository + Send + Sync,
{
    guild_commanders_repository: Arc<T>,
}

impl<T> GuildComanndersUseCase<T>
where
    T: GuildCommandersRepository + Send + Sync,
{
    pub fn new(guild_commanders_repository: Arc<T>) -> Self {
        Self {
            guild_commanders_repository,
        }
    }

    pub async fn register(
        &self,
        mut register_guild_commander_model: RegisterGuildCommanderModel,
    ) -> Result<i32> {
        let hashed_password =
            argon2_hashing::hash(register_guild_commander_model.password.clone())?;
        register_guild_commander_model.password = hashed_password;
        let register_entity = register_guild_commander_model.to_entity();
        let guild_commander_id = self
            .guild_commanders_repository
            .register(register_entity)
            .await?;

        Ok(guild_commander_id)
    }
}
