use std::sync::Arc;

use anyhow::{Ok, Result};

use crate::domain::{
    repositories::{quest_ops::QuestOpsRepository, quest_viewing::QuestViewingRepository},
    value_objects::quest_model::{AddQuestModel, EditQuestModel},
};

pub struct QuestOpsUseCase<T, A>
where
    // Send: Allows moving data between threads.
    // Sync: Allows data to be accessed safely by multiple threads simultaneously.
    T: QuestOpsRepository + Send + Sync,
    A: QuestViewingRepository + Send + Sync,
{
    quest_ops_repository: Arc<T>,
    quest_viewing_repository: Arc<A>,
}

impl<T, A> QuestOpsUseCase<T, A>
where
    T: QuestOpsRepository + Send + Sync,
    A: QuestViewingRepository + Send + Sync,
{
    pub fn new(quest_ops_repository: Arc<T>, quest_viewing_repository: Arc<A>) -> Self {
        Self {
            quest_ops_repository,
            quest_viewing_repository,
        }
    }
    pub async fn add(
        &self,
        guild_commander_id: i32,
        add_quest_model: AddQuestModel,
    ) -> Result<i32> {
        let insert_quest_entity = add_quest_model.to_entity(guild_commander_id);
        let result = self.quest_ops_repository.add(insert_quest_entity).await?;
        Ok(result)
    }
    pub async fn edit(
        &self,
        quest_id: i32,
        guild_commander_id: i32,
        edit_quest_model: EditQuestModel,
    ) -> Result<i32> {
        let adventurerrs_count = self
            .quest_viewing_repository
            .adventurers_counting_by_quest_id(quest_id)
            .await?;

        if adventurerrs_count > 0 {
            return Err(anyhow::anyhow!(
                "Quest has been taken by adventurers for now!"
            ));
        }

        let edit_quest_entity = edit_quest_model.to_entity(guild_commander_id);
        let result = self
            .quest_ops_repository
            .edit(quest_id, edit_quest_entity)
            .await?;

        Ok(result)
    }
    pub async fn remove(&self, quest_id: i32, guild_comannder_id: i32) -> Result<()> {
        let adventurerrs_count = self
            .quest_viewing_repository
            .adventurers_counting_by_quest_id(quest_id)
            .await?;

        if adventurerrs_count > 0 {
            return Err(anyhow::anyhow!(
                "Quest has been taken by adventurers for now!"
            ));
        }

        self.quest_ops_repository
            .remove(quest_id, guild_comannder_id)
            .await?;

        Ok(())
    }
}
