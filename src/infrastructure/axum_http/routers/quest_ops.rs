use std::sync::Arc;

use axum::{
    Extension, Json, Router,
    extract::{Path, State},
    http::StatusCode,
    middleware,
    response::IntoResponse,
    routing::{delete, patch, post},
};

use crate::{
    application::usecases::quest_ops::QuestOpsUseCase,
    domain::{
        repositories::{quest_ops::QuestOpsRepository, quest_viewing::QuestViewingRepository},
        value_objects::quest_model::{AddQuestModel, EditQuestModel},
    },
    infrastructure::{
        axum_http::middlewares::guild_commanders_authorization,
        postgres::{
            postgres_connection::PgPoolSquad,
            repositories::{quest_ops::QuestOpsPostgres, quest_viewing::QuestViewingPostgres},
        },
    },
};
//     quest_ops_repository: Arc<T>,
//     quest_viewing_repository: Arc<A>,
pub fn routes(db_pool: Arc<PgPoolSquad>) -> Router {
    let quest_ops_repository = QuestOpsPostgres::new(Arc::clone(&db_pool));
    let quest_viewing_repository = QuestViewingPostgres::new(Arc::clone(&db_pool));
    let quest_ops_use_case = QuestOpsUseCase::new(
        Arc::new(quest_ops_repository),
        Arc::new(quest_viewing_repository),
    );
    Router::new()
        .route("/", post(add))
        .route("/{quest_id}", patch(edit))
        .route("/{quest_id}", delete(remove))
        .route_layer(middleware::from_fn(guild_commanders_authorization))
        .with_state(Arc::new(quest_ops_use_case))
}

async fn add<T, A>(
    State(quest_ops_use_case): State<Arc<QuestOpsUseCase<T, A>>>,
    Extension(guild_commander_id): Extension<i32>,
    Json(add_quest_model): Json<AddQuestModel>,
) -> impl IntoResponse
where
    T: QuestOpsRepository + Send + Sync,
    A: QuestViewingRepository + Send + Sync,
{
    match quest_ops_use_case
        .add(guild_commander_id, add_quest_model)
        .await
    {
        Ok(quest_id_result) => {
            let response = format!("Add quest success with id: {}", quest_id_result);
            (StatusCode::CREATED, response).into_response()
        }
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

async fn edit<T, A>(
    State(quest_ops_use_case): State<Arc<QuestOpsUseCase<T, A>>>,
    Extension(guild_commander_id): Extension<i32>,
    Path(quest_id): Path<i32>,
    Json(edit_quest_model): Json<EditQuestModel>,
) -> impl IntoResponse
where
    T: QuestOpsRepository + Send + Sync,
    A: QuestViewingRepository + Send + Sync,
{
    match quest_ops_use_case
        .edit(quest_id, guild_commander_id, edit_quest_model)
        .await
    {
        Ok(quest_id_result) => {
            let response = format!("Edit quest success with id: {}", quest_id_result);
            (StatusCode::OK, response).into_response()
        }
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

async fn remove<T, A>(
    State(quest_ops_use_case): State<Arc<QuestOpsUseCase<T, A>>>,
    Extension(guild_commander_id): Extension<i32>,
    Path(quest_id): Path<i32>,
) -> impl IntoResponse
where
    T: QuestOpsRepository + Send + Sync,
    A: QuestViewingRepository + Send + Sync,
{
    match quest_ops_use_case
        .remove(quest_id, guild_commander_id)
        .await
    {
        Ok(_) => {
            let response = format!("Remove quest success with id: {}", quest_id);
            (StatusCode::OK, response).into_response()
        }
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}
