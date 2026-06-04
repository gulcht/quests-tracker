use axum::{
    extract::Request,
    http::{StatusCode, header},
    middleware::Next,
    response::Response,
};

use crate::{
    config::config_loaders::{get_adventurer_secret_env, get_guild_commanders_secret_env},
    infrastructure::jwt_authentication,
};

pub async fn adventurers_authorization(
    mut req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let adventurer_id = req
        .headers()
        .get(header::COOKIE)
        .and_then(|cookie_header| cookie_header.to_str().ok())
        .and_then(|cookie_str| get_cookie_value(cookie_str, "act"))
        .and_then(|token| {
            let secret_env = get_adventurer_secret_env().ok()?;
            let claims = jwt_authentication::verify_token(secret_env.secret, token).ok()?;
            claims.sub.parse::<i32>().ok()
        });

    if let Some(adventurer_id) = adventurer_id {
        req.extensions_mut().insert(adventurer_id);
        return Ok(next.run(req).await);
    }
    Err(StatusCode::UNAUTHORIZED)
}

pub async fn guild_commanders_authorization(
    mut req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let guild_commander_id = req
        .headers()
        .get(header::COOKIE)
        .and_then(|cookie_header| cookie_header.to_str().ok())
        .and_then(|cookie_str| get_cookie_value(cookie_str, "act"))
        .and_then(|token| {
            let secret_env = get_guild_commanders_secret_env().ok()?;
            let claims = jwt_authentication::verify_token(secret_env.secret, token).ok()?;
            claims.sub.parse::<i32>().ok()
        });

    if let Some(guild_commander_id) = guild_commander_id {
        req.extensions_mut().insert(guild_commander_id);
        return Ok(next.run(req).await);
    }
    Err(StatusCode::UNAUTHORIZED)
}

fn get_cookie_value(cookie_header: &str, key: &str) -> Option<String> {
    cookie_header.split("; ").find_map(|cookie| {
        let mut parts = cookie.splitn(2, '=');
        let name = parts.next()?.trim();
        let value = parts.next()?.trim();
        if name == key {
            Some(value.to_string())
        } else {
            None
        }
    })
}
