use std::env;

/// Runtime configuration. Secrets are read from environment variables and never logged.
#[derive(Clone, Debug)]
pub struct Config {
    pub discord_token: String,
    pub database_path: String,
}

impl Config {
    pub fn load() -> Result<Self, String> {
        dotenvy::dotenv().ok();
        let discord_token = env::var("DISCORD_TOKEN")
            .map_err(|_| "DISCORD_TOKEN is missing; set it in your environment or .env file".to_string())?;
        if discord_token.trim().is_empty() || discord_token == "your_bot_token_here" {
            return Err("DISCORD_TOKEN is empty or still the example value".into());
        }
        let database_path = env::var("MEOWY_DATABASE_PATH").unwrap_or_else(|_| "meowy.sqlite3".into());
        Ok(Self { discord_token, database_path })
    }
}
