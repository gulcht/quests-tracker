use std::sync::Arc;

use axum::{
    Extension, Router,
    extract::{Path, State},
    http::StatusCode,
    middleware,
    response::IntoResponse,
    routing::{delete, post},
};

use crate::{
    application::usecases::crew_switchboard::CrewSwitchboardUseCase,
    domain::repositories::{
        crew_switchboard::CrewSwitchboardRepository, quest_viewing::QuestViewingRepository,
    },
    infrastructure::{
        axum_http::middlewares::adventurers_authorization,
        postgres::{
            postgres_connection::PgPoolSquad,
            repositories::{
                crew_switchboard::CrewSwitchboardPostgres, quest_viewing::QuestViewingPostgres,
            },
        },
    },
};

pub fn routes(db_pool: Arc<PgPoolSquad>) -> Router {
    let crew_switchboard_repository = CrewSwitchboardPostgres::new(Arc::clone(&db_pool));
    let quest_viewing_repository = QuestViewingPostgres::new(Arc::clone(&db_pool));
    let crew_switchboard_use_case = CrewSwitchboardUseCase::new(
        Arc::new(crew_switchboard_repository),
        Arc::new(quest_viewing_repository),
    );
    Router::new()
        .route("/join/{quest_id}", post(join))
        .route("/leave/{quest_id}", delete(leave))
        .route_layer(middleware::from_fn(adventurers_authorization))
        .with_state(Arc::new(crew_switchboard_use_case))
}

async fn join<T, A>(
    State(crew_switchboard_use_case): State<Arc<CrewSwitchboardUseCase<T, A>>>,
    Extension(adventurer_id): Extension<i32>,
    Path(quest_id): Path<i32>,
) -> impl IntoResponse
where
    T: CrewSwitchboardRepository + Send + Sync,
    A: QuestViewingRepository + Send + Sync,
{
    match crew_switchboard_use_case
        .join(quest_id, adventurer_id)
        .await
    {
        Ok(_) => (
            StatusCode::OK,
            format!(
                "Adventurer id: {}, has joined quest id: {}",
                adventurer_id, quest_id
            ),
        )
            .into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

async fn leave<T, A>(
    State(crew_switchboard_use_case): State<Arc<CrewSwitchboardUseCase<T, A>>>,
    Extension(adventurer_id): Extension<i32>,
    Path(quest_id): Path<i32>,
) -> impl IntoResponse
where
    T: CrewSwitchboardRepository + Send + Sync,
    A: QuestViewingRepository + Send + Sync,
{
    match crew_switchboard_use_case
        .leave(quest_id, adventurer_id)
        .await
    {
        Ok(_) => (
            StatusCode::OK,
            format!(
                "Adventurer id: {}, has join and leave quest id: {}",
                adventurer_id, quest_id
            ),
        )
            .into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}
