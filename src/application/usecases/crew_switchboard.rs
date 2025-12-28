use std::sync::Arc;

use anyhow::{Ok, Result};

use crate::domain::{
    repositories::{
        crew_switchboard::CrewSwitchboardRepository, quest_viewing::QuestViewingRepository,
    },
    value_objects::{
        quest_adventurer_junction::{MAX_ADVENTURERS_PER_QUEST, QuestAdventurerJunction},
        quest_statuses::QuestStatuses,
    },
};

pub struct CrewSwitchboardUseCase<T, A>
where
    // Send: Allows moving data between threads.
    // Sync: Allows data to be accessed safely by multiple threads simultaneously.
    T: CrewSwitchboardRepository + Send + Sync,
    A: QuestViewingRepository + Send + Sync,
{
    pub crew_switchboard_repository: Arc<T>,
    pub quest_viewing_repository: Arc<A>,
}

impl<T, A> CrewSwitchboardUseCase<T, A>
where
    T: CrewSwitchboardRepository + Send + Sync,
    A: QuestViewingRepository + Send + Sync,
{
    pub fn new(crew_switchboard_repository: Arc<T>, quest_viewing_repository: Arc<A>) -> Self {
        Self {
            crew_switchboard_repository,
            quest_viewing_repository,
        }
    }

    pub async fn join(&self, quest_id: i32, adventurer_id: i32) -> Result<()> {
        let quest = self.quest_viewing_repository.view_details(quest_id).await?;

        let adventurer_count = self
            .quest_viewing_repository
            .adventurers_counting_by_quest_id(quest_id)
            .await?;

        let quest_status_condition = quest.status == QuestStatuses::Open.to_string()
            || quest.status == QuestStatuses::Failed.to_string();

        let adventurer_count_condition = adventurer_count < MAX_ADVENTURERS_PER_QUEST;

        if !quest_status_condition {
            return Err(anyhow::anyhow!("Quest is not joinable"));
        }

        if !adventurer_count_condition {
            return Err(anyhow::anyhow!("Quest is full"));
        }

        self.crew_switchboard_repository
            .join(QuestAdventurerJunction {
                quest_id,
                adventurer_id,
            })
            .await?;

        Ok(())
    }
    pub async fn leave(&self, quest_id: i32, adventurer_id: i32) -> Result<()> {
        let quest = self.quest_viewing_repository.view_details(quest_id).await?;

        let leaving_conditions = quest.status == QuestStatuses::Open.to_string()
            || quest.status == QuestStatuses::Failed.to_string();

        if !leaving_conditions {
            return Err(anyhow::anyhow!("Quest is not leavable"));
        };

        self.crew_switchboard_repository
            .leave(QuestAdventurerJunction {
                adventurer_id,
                quest_id,
            })
            .await?;

        Ok(())
    }
}
