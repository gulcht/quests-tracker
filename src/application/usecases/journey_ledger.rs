use std::sync::Arc;

use anyhow::{Ok, Result};

use crate::domain::{
    repositories::{journey_ledger::JouneyLedgerRepository, quest_viewing::QuestViewingRepository},
    value_objects::{
        quest_adventurer_junction::MAX_ADVENTURERS_PER_QUEST, quest_statuses::QuestStatuses,
    },
};

pub struct JourneyLedgerUseCase<T, A>
where
    // Send: Allows moving data between threads.
    // Sync: Allows data to be accessed safely by multiple threads simultaneously.
    T: JouneyLedgerRepository + Send + Sync,
    A: QuestViewingRepository + Send + Sync,
{
    journey_ledger_repository: Arc<T>,
    quest_viewing_repository: Arc<A>,
}

impl<T, A> JourneyLedgerUseCase<T, A>
where
    T: JouneyLedgerRepository + Send + Sync,
    A: QuestViewingRepository + Send + Sync,
{
    pub fn new(journey_ledger_repository: Arc<T>, quest_viewing_repository: Arc<A>) -> Self {
        Self {
            journey_ledger_repository,
            quest_viewing_repository,
        }
    }
    pub async fn in_journey(&self, quest_id: i32, guild_commander_id: i32) -> Result<i32> {
        let quest = self.quest_viewing_repository.view_details(quest_id).await?;

        let adventurers_number = self
            .quest_viewing_repository
            .adventurers_counting_by_quest_id(quest_id)
            .await?;

        let conditons_to_update = (quest.status == QuestStatuses::Open.to_string()
            || quest.status == QuestStatuses::Failed.to_string())
            && adventurers_number > 0
            && adventurers_number <= MAX_ADVENTURERS_PER_QUEST;

        if !conditons_to_update {
            return Err(anyhow::anyhow!("Invalid condition to change status"));
        }

        let result = self
            .journey_ledger_repository
            .in_journey(quest_id, guild_commander_id)
            .await?;

        Ok(result)
    }
    pub async fn to_completed(&self, quest_id: i32, guild_commander_id: i32) -> Result<i32> {
        let quest = self.quest_viewing_repository.view_details(quest_id).await?;

        let conditions_to_update = quest.status == QuestStatuses::InJourney.to_string();

        println!("{}", quest.status.to_string());
        if !conditions_to_update {
            return Err(anyhow::anyhow!("Invalid condition to change status"));
        }

        let result = self
            .journey_ledger_repository
            .to_completed(quest_id, guild_commander_id)
            .await?;

        Ok(result)
    }
    pub async fn to_failed(&self, quest_id: i32, guild_commander_id: i32) -> Result<i32> {
        let quest = self.quest_viewing_repository.view_details(quest_id).await?;

        let conditions_to_update = quest.status == QuestStatuses::InJourney.to_string();

        if !conditions_to_update {
            return Err(anyhow::anyhow!("Invalid condition to change status"));
        }

        let result = self
            .journey_ledger_repository
            .to_failed(quest_id, guild_commander_id)
            .await?;

        Ok(result)
    }
}
