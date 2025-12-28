use std::sync::Arc;

use anyhow::{Ok, Result};
use async_trait::async_trait;
use diesel::{delete, insert_into, prelude::*};

use crate::{
    domain::{
        repositories::crew_switchboard::CrewSwitchboardRepository,
        value_objects::quest_adventurer_junction::QuestAdventurerJunction,
    },
    infrastructure::postgres::{
        postgres_connection::PgPoolSquad, schema::quest_adventurer_junction,
    },
};

pub struct CrewSwitchboardPostgres {
    db_pool: Arc<PgPoolSquad>,
}

impl CrewSwitchboardPostgres {
    pub fn new(db_pool: Arc<PgPoolSquad>) -> Self {
        Self { db_pool }
    }
}

#[async_trait]
impl CrewSwitchboardRepository for CrewSwitchboardPostgres {
    async fn join(&self, junction_body: QuestAdventurerJunction) -> Result<()> {
        let mut conn = Arc::clone(&self.db_pool).get()?;

        let _ = insert_into(quest_adventurer_junction::table)
            .values(junction_body)
            .execute(&mut conn)?;

        Ok(())
    }
    async fn leave(&self, junction_body: QuestAdventurerJunction) -> Result<()> {
        let mut conn = Arc::clone(&self.db_pool).get()?;

        let _ = delete(quest_adventurer_junction::table)
            .filter(
                quest_adventurer_junction::adventurer_id
                    .eq(junction_body.adventurer_id)
                    .and(quest_adventurer_junction::quest_id.eq(junction_body.quest_id)),
            )
            .execute(&mut conn)?;

        Ok(())
    }
}
