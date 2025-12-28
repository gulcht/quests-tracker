use std::sync::Arc;

use crate::{
    application::usecases::authentication::AuthenticationUseCase,
    config::{config_loaders::get_stage, stage::Stage},
    domain::repositories::{
        adventurers::AdventurersReopsitory, guild_commanders::GuildCommandersRepository,
    },
    infrastructure::{
        jwt_authentication::authentication_model::LoginModel,
        postgres::{
            postgres_connection::PgPoolSquad,
            repositories::{
                adventurers::AdventurerPostgres, guild_commanders::GuildCommanderPostgres,
            },
        },
    },
};
use axum::{
    Json, Router,
    extract::State,
    http::{HeaderMap, HeaderValue, StatusCode, header},
    response::IntoResponse,
    routing::post,
};
use axum_extra::extract::cookie::{Cookie, CookieJar};
use cookie::time::Duration;

pub fn routes(db_pool: Arc<PgPoolSquad>) -> Router {
    let adventurers_repository = AdventurerPostgres::new(Arc::clone(&db_pool));
    let guild_commanders_repository = GuildCommanderPostgres::new(Arc::clone(&db_pool));
    let authentication_use_case = AuthenticationUseCase::new(
        Arc::new(adventurers_repository),
        Arc::new(guild_commanders_repository),
    );
    Router::new()
        .route("/adventurers/login", post(adventurers_login))
        .route(
            "/adventurers/refresh-token",
            post(adventurers_refresh_token),
        )
        .route("/guild-commanders/login", post(guild_commanders_login))
        .route(
            "/guild-commanders/refresh-token",
            post(guild_commanders_refresh_token),
        )
        .with_state(Arc::new(authentication_use_case))
}

async fn adventurers_login<T, A>(
    State(authentication_use_case): State<Arc<AuthenticationUseCase<T, A>>>,
    Json(login_model): Json<LoginModel>,
) -> impl IntoResponse
where
    T: AdventurersReopsitory + Send + Sync,
    A: GuildCommandersRepository + Send + Sync,
{
    match authentication_use_case.adventurers_login(login_model).await {
        Ok(passport) => {
            let mut act_cookie = Cookie::build(("act", passport.access_token.clone()))
                .path("/")
                .same_site(cookie::SameSite::Lax)
                .http_only(true)
                .max_age(Duration::days(1));

            let mut rft_cookie = Cookie::build(("rft", passport.refresh_token.clone()))
                .path("/")
                .same_site(cookie::SameSite::Lax)
                .http_only(true)
                .max_age(Duration::days(7));

            if get_stage() == Stage::Production {
                act_cookie = act_cookie.secure(true);
                rft_cookie = rft_cookie.secure(true);
            }

            let mut headers = HeaderMap::new();
            headers.append(
                header::SET_COOKIE,
                HeaderValue::from_str(&act_cookie.to_string()).unwrap(),
            );

            headers.append(
                header::SET_COOKIE,
                HeaderValue::from_str(&rft_cookie.to_string()).unwrap(),
            );

            (StatusCode::OK, headers, "Login successfully").into_response()
        }
        Err(e) => (StatusCode::UNAUTHORIZED, e.to_string()).into_response(),
    }
}

pub async fn adventurers_refresh_token<T, A>(
    State(authentication_use_case): State<Arc<AuthenticationUseCase<T, A>>>,
    jar: CookieJar,
) -> impl IntoResponse
where
    T: AdventurersReopsitory + Send + Sync,
    A: GuildCommandersRepository + Send + Sync,
{
    if let Some(rft) = jar.get("rft") {
        let refresh_token = rft.value().to_string();
        let response = match authentication_use_case
            .adventurers_refresh_token(refresh_token)
            .await
        {
            Ok(passport) => {
                let mut act_cookie = Cookie::build(("act", passport.access_token.clone()))
                    .path("/")
                    .same_site(cookie::SameSite::Lax)
                    .http_only(true)
                    .max_age(Duration::days(1));

                let mut rft_cookie = Cookie::build(("rft", passport.refresh_token.clone()))
                    .path("/")
                    .same_site(cookie::SameSite::Lax)
                    .http_only(true)
                    .max_age(Duration::days(7));

                if get_stage() == Stage::Production {
                    act_cookie = act_cookie.secure(true);
                    rft_cookie = rft_cookie.secure(true);
                }

                let mut headers = HeaderMap::new();
                headers.append(
                    header::SET_COOKIE,
                    HeaderValue::from_str(&act_cookie.to_string()).unwrap(),
                );

                headers.append(
                    header::SET_COOKIE,
                    HeaderValue::from_str(&rft_cookie.to_string()).unwrap(),
                );

                (StatusCode::OK, headers, "Login successfully").into_response()
            }
            Err(e) => (StatusCode::UNAUTHORIZED, e.to_string()).into_response(),
        };
        return response;
    }
    (StatusCode::BAD_REQUEST, "Refresh token not found").into_response()
}

pub async fn guild_commanders_login<T, A>(
    State(authentication_use_case): State<Arc<AuthenticationUseCase<T, A>>>,
    Json(login_model): Json<LoginModel>,
) -> impl IntoResponse
where
    T: AdventurersReopsitory + Send + Sync,
    A: GuildCommandersRepository + Send + Sync,
{
    match authentication_use_case
        .guild_commanders_login(login_model)
        .await
    {
        Ok(passport) => {
            let mut act_cookie = Cookie::build(("act", passport.access_token.clone()))
                .path("/")
                .same_site(cookie::SameSite::Lax)
                .http_only(true)
                .max_age(Duration::days(1));

            let mut rft_cookie = Cookie::build(("rft", passport.refresh_token.clone()))
                .path("/")
                .same_site(cookie::SameSite::Lax)
                .http_only(true)
                .max_age(Duration::days(7));

            if get_stage() == Stage::Production {
                act_cookie = act_cookie.secure(true);
                rft_cookie = rft_cookie.secure(true);
            }

            let mut headers = HeaderMap::new();
            headers.append(
                header::SET_COOKIE,
                HeaderValue::from_str(&act_cookie.to_string()).unwrap(),
            );

            headers.append(
                header::SET_COOKIE,
                HeaderValue::from_str(&rft_cookie.to_string()).unwrap(),
            );

            (StatusCode::OK, headers, "Login successfully").into_response()
        }
        Err(e) => (StatusCode::UNAUTHORIZED, e.to_string()).into_response(),
    }
}

pub async fn guild_commanders_refresh_token<T, A>(
    State(authentication_use_case): State<Arc<AuthenticationUseCase<T, A>>>,
    jar: CookieJar,
) -> impl IntoResponse
where
    T: AdventurersReopsitory + Send + Sync,
    A: GuildCommandersRepository + Send + Sync,
{
    if let Some(rft) = jar.get("rft") {
        let refresh_token = rft.value().to_string();
        let response = match authentication_use_case
            .guild_commanders_refresh_token(refresh_token)
            .await
        {
            Ok(passport) => {
                let mut act_cookie = Cookie::build(("act", passport.access_token.clone()))
                    .path("/")
                    .same_site(cookie::SameSite::Lax)
                    .http_only(true)
                    .max_age(Duration::days(1));

                let mut rft_cookie = Cookie::build(("rft", passport.refresh_token.clone()))
                    .path("/")
                    .same_site(cookie::SameSite::Lax)
                    .http_only(true)
                    .max_age(Duration::days(7));

                if get_stage() == Stage::Production {
                    act_cookie = act_cookie.secure(true);
                    rft_cookie = rft_cookie.secure(true);
                }

                let mut headers = HeaderMap::new();
                headers.append(
                    header::SET_COOKIE,
                    HeaderValue::from_str(&act_cookie.to_string()).unwrap(),
                );

                headers.append(
                    header::SET_COOKIE,
                    HeaderValue::from_str(&rft_cookie.to_string()).unwrap(),
                );

                (StatusCode::OK, headers, "Login successfully").into_response()
            }
            Err(e) => (StatusCode::UNAUTHORIZED, e.to_string()).into_response(),
        };
        return response;
    }
    (StatusCode::BAD_REQUEST, "Refresh token not found").into_response()
}
