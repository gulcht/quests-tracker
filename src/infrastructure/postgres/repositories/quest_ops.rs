use std::sync::Arc;

use anyhow::{Ok, Result};
use async_trait::async_trait;
use diesel::{ExpressionMethods, RunQueryDsl, insert_into, update};

use crate::{
    domain::{
        entities::quest::{AddQuestEntity, EditQuestEntity},
        repositories::quest_ops::QuestOpsRepository,
        value_objects::quest_statuses::QuestStatuses,
    },
    infrastructure::postgres::{postgres_connection::PgPoolSquad, schema::quests},
};

pub struct QuestOpsPostgres {
    db_pool: Arc<PgPoolSquad>,
}

impl QuestOpsPostgres {
    pub fn new(db_pool: Arc<PgPoolSquad>) -> Self {
        Self { db_pool }
    }
}

#[async_trait]
impl QuestOpsRepository for QuestOpsPostgres {
    async fn add(&self, add_quest_entity: AddQuestEntity) -> Result<i32> {
        let mut conn = Arc::clone(&self.db_pool).get()?;

        let result = insert_into(quests::table)
            .values(add_quest_entity)
            .returning(quests::id)
            .get_result::<i32>(&mut conn)?;

        Ok(result)
    }
    async fn edit(&self, quest_id: i32, edit_quest_entity: EditQuestEntity) -> Result<i32> {
        let mut conn = Arc::clone(&self.db_pool).get()?;

        let result = update(quests::table)
            .filter(quests::id.eq(quest_id))
            .filter(quests::deleted_at.is_null())
            .filter(quests::status.eq(QuestStatuses::Open.to_string()))
            .set(edit_quest_entity)
            .returning(quests::id)
            .get_result::<i32>(&mut conn)?;

        Ok(result)
    }
    async fn remove(&self, quest_id: i32, guild_comannder_id: i32) -> Result<()> {
        let mut conn = Arc::clone(&self.db_pool).get()?;

        update(quests::table)
            .filter(quests::id.eq(quest_id))
            .filter(quests::deleted_at.is_null())
            .filter(quests::status.eq(QuestStatuses::Open.to_string()))
            .set((
                quests::deleted_at.eq(diesel::dsl::now),
                quests::guild_commander_id.eq(guild_comannder_id),
            ))
            .execute(&mut conn)?;
        Ok(())
    }
}
