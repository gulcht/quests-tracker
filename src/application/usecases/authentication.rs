use std::sync::Arc;

use anyhow::{Ok, Result};
use chrono::{Duration, Utc};

use crate::{
    config::config_loaders::{get_adventurer_secret_env, get_guild_commanders_secret_env},
    domain::repositories::{
        adventurers::AdventurersReopsitory, guild_commanders::GuildCommandersRepository,
    },
    infrastructure::{
        argon2_hashing,
        jwt_authentication::{
            self,
            authentication_model::LoginModel,
            jwt_model::{Claims, Passport, Roles},
        },
    },
};

pub struct AuthenticationUseCase<T, A>
where
    // Send: Allows moving data between threads.
    // Sync: Allows data to be accessed safely by multiple threads simultaneously.
    T: AdventurersReopsitory + Send + Sync,
    A: GuildCommandersRepository + Send + Sync,
{
    adventurers_repository: Arc<T>,
    guild_commanders_repository: Arc<A>,
}

impl<T, A> AuthenticationUseCase<T, A>
where
    T: AdventurersReopsitory + Send + Sync,
    A: GuildCommandersRepository + Send + Sync,
{
    pub fn new(adventurers_repository: Arc<T>, guild_commanders_repository: Arc<A>) -> Self {
        Self {
            adventurers_repository,
            guild_commanders_repository,
        }
    }

    pub async fn adventurers_login(&self, login_model: LoginModel) -> Result<Passport> {
        let secret_env = get_adventurer_secret_env()?;

        let adventurer = self
            .adventurers_repository
            .find_by_username(login_model.username.clone())
            .await?;

        let hashed_password = adventurer.password;
        let password = login_model.password;

        if !argon2_hashing::verify(password, hashed_password)? {
            return Err(anyhow::anyhow!("Invalid password"));
        };

        let access_token_claims = Claims {
            sub: adventurer.id.to_string(),
            role: Roles::Adventurer,
            exp: (Utc::now() + Duration::days(1)).timestamp() as usize,
            iat: Utc::now().timestamp() as usize,
        };

        let refresh_token_claims = Claims {
            sub: adventurer.id.to_string(),
            role: Roles::Adventurer,
            exp: (Utc::now() + Duration::days(7)).timestamp() as usize,
            iat: Utc::now().timestamp() as usize,
        };

        let access_token =
            jwt_authentication::generate_token(secret_env.secret, &access_token_claims)?;
        let refresh_token =
            jwt_authentication::generate_token(secret_env.refresh_secret, &refresh_token_claims)?;

        Ok(Passport {
            access_token,
            refresh_token,
        })
    }

    pub async fn adventurers_refresh_token(&self, refresh_token: String) -> Result<Passport> {
        let secret_env = get_adventurer_secret_env()?;

        let claims =
            jwt_authentication::verify_token(secret_env.refresh_secret.clone(), refresh_token)?;

        let access_token_claims = Claims {
            sub: claims.sub.clone(),
            role: Roles::Adventurer,
            exp: (Utc::now() + Duration::days(1)).timestamp() as usize,
            iat: Utc::now().timestamp() as usize,
        };

        let refresh_token_claims = Claims {
            sub: claims.sub,
            role: Roles::Adventurer,
            exp: claims.exp,
            iat: Utc::now().timestamp() as usize,
        };

        let access_token =
            jwt_authentication::generate_token(secret_env.secret, &access_token_claims)?;
        let refresh_token =
            jwt_authentication::generate_token(secret_env.refresh_secret, &refresh_token_claims)?;

        Ok(Passport {
            access_token,
            refresh_token,
        })
    }

    pub async fn guild_commanders_login(&self, login_model: LoginModel) -> Result<Passport> {
        let secret_env = get_guild_commanders_secret_env()?;

        let guild_commander = self
            .guild_commanders_repository
            .find_by_username(login_model.username.clone())
            .await?;

        let hashed_password = guild_commander.password;
        let password = login_model.password;

        if !argon2_hashing::verify(password, hashed_password)? {
            return Err(anyhow::anyhow!("Invalid password"));
        };

        let access_token_claims = Claims {
            sub: guild_commander.id.to_string(),
            role: Roles::GuildCommander,
            exp: (Utc::now() + Duration::days(1)).timestamp() as usize,
            iat: Utc::now().timestamp() as usize,
        };

        let refresh_token_claims = Claims {
            sub: guild_commander.id.to_string(),
            role: Roles::GuildCommander,
            exp: (Utc::now() + Duration::days(1)).timestamp() as usize,
            iat: Utc::now().timestamp() as usize,
        };

        let access_token =
            jwt_authentication::generate_token(secret_env.secret, &access_token_claims)?;
        let refresh_token =
            jwt_authentication::generate_token(secret_env.refresh_secret, &refresh_token_claims)?;

        Ok(Passport {
            access_token,
            refresh_token,
        })
    }

    pub async fn guild_commanders_refresh_token(&self, refresh_token: String) -> Result<Passport> {
        let secret_env = get_guild_commanders_secret_env()?;

        let claims =
            jwt_authentication::verify_token(secret_env.refresh_secret.clone(), refresh_token)?;

        let access_token_claims = Claims {
            sub: claims.sub.clone(),
            role: Roles::GuildCommander,
            exp: (Utc::now() + Duration::days(1)).timestamp() as usize,
            iat: Utc::now().timestamp() as usize,
        };

        let refresh_token_claims = Claims {
            sub: claims.sub,
            role: Roles::GuildCommander,
            exp: claims.exp,
            iat: Utc::now().timestamp() as usize,
        };

        let access_token =
            jwt_authentication::generate_token(secret_env.secret, &access_token_claims)?;
        let refresh_token =
            jwt_authentication::generate_token(secret_env.refresh_secret, &refresh_token_claims)?;

        Ok(Passport {
            access_token,
            refresh_token,
        })
    }
}
