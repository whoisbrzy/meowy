mod config;
mod db;
mod commands;
mod ui;

use poise::serenity_prelude as serenity;

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, (), Error>;

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let config = config::Config::load().map_err(|e| {
        tracing::error!("{e}");
        e
    })?;
    // Initialize schema and verify database path at startup.
    let _db = db::Database::open(&config.database_path)?;

    let commands = vec![
        commands::core::ping(),
        commands::core::echo(),
        commands::core::coinflip(),
        commands::core::color(),
        commands::core::usage(),
        commands::core::settings(),
    ];

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: commands.clone(),
            ..Default::default()
        })
        .setup(|ctx, _ready, _framework| Box::pin(async move {
            poise::builtins::register_globally(ctx, &[
                commands::core::ping(),
                commands::core::echo(),
                commands::core::coinflip(),
                commands::core::color(),
                commands::core::usage(),
                commands::core::settings(),
            ]).await?;
            tracing::info!("Meowy commands registered");
            Ok(())
        }))
        .build();

    let intents = serenity::GatewayIntents::GUILDS;
    let mut client = serenity::ClientBuilder::new(config.discord_token, intents)
        .framework(framework)
        .await?;
    client.start().await?;
    Ok(())
}
