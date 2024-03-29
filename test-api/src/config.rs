use std::{collections::BTreeSet, env};

use firestore_db_and_auth::Credentials;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("failed get env error (key={key})")]
    FailedGetEnv { key: String },
    #[error("failed deserialize")]
    FailedDeserialize(#[from] serde_json::Error),
}

#[derive(Debug, Clone)]
pub struct FirestoreConfig {
    pub credentials: Credentials,
}

#[derive(Debug, Clone)]
pub struct AuthConfig {
    pub allowed_user_uids: BTreeSet<String>,
}

#[derive(Debug, Clone)]
pub struct Config {
    pub firestore: FirestoreConfig,
    pub auth: AuthConfig,
}

fn get_env(key: &str) -> Result<String, ConfigError> {
    env::var(key).map_err(|_| ConfigError::FailedGetEnv {
        key: key.to_string(),
    })
}

pub fn get_firestore_config() -> Result<FirestoreConfig, ConfigError> {
    Ok(FirestoreConfig {
        credentials: serde_json::from_str(&get_env("CREDENTIALS_JSON")?)?,
    })
}

pub fn get_auth_config() -> Result<AuthConfig, ConfigError> {
    Ok(AuthConfig {
        allowed_user_uids: serde_json::from_str(&get_env("ALLOWED_USER_UIDS")?)?,
    })
}

pub fn config() -> Result<Config, ConfigError> {
    Ok(Config {
        firestore: get_firestore_config()?,
        auth: get_auth_config()?,
    })
}
