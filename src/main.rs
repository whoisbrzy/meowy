mod config;
mod db;
mod commands;
mod ui;
use poise::serenity_prelude as serenity;
pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, (), Error>;
#[tokio::main]
async fn main() -> Result<(), Error> {
 tracing_subscriber::fmt().with_env_filter(tracing_subscriber::EnvFilter::from_default_env()).init();
 let config=config::Config::load().map_err(|e|{tracing::error!("{e}");e})?;
 let _db=db::Database::open(&config.database_path)?;
 let commands=vec![
 commands::core::ping(),commands::core::echo(),commands::core::coinflip(),commands::core::color(),commands::core::usage(),commands::core::settings(),
 commands::discord_utils::avatar(),commands::discord_utils::userinfo(),commands::discord_utils::guildinfo(),
 commands::ai::ask(),commands::ai::ai_usage(),commands::ai::transcribe(),commands::ai::ocr(),
 commands::games::minecraft_uuid(),commands::games::minecraft_server(),commands::games::roblox_user(),commands::games::fortnite_news(),
 commands::social::github_profile(),commands::social::github_repo(),commands::social::urban(),\n commands::social::lastfm_recent(),commands::social::lastfm_top(),commands::social::spotify_search(),commands::social::instagram(),commands::social::tiktok(),commands::social::x_profile(),commands::social::telegram(),
 ];
 let framework=poise::Framework::builder().options(poise::FrameworkOptions{commands:commands.clone(),..Default::default()})
 .setup(|ctx,_ready,_framework|Box::pin(async move{
 poise::builtins::register_globally(ctx,&[
 commands::core::ping(),commands::core::echo(),commands::core::coinflip(),commands::core::color(),commands::core::usage(),commands::core::settings(),
 commands::discord_utils::avatar(),commands::discord_utils::userinfo(),commands::discord_utils::guildinfo(),
 commands::ai::ask(),commands::ai::ai_usage(),commands::ai::transcribe(),commands::ai::ocr(),
 commands::games::minecraft_uuid(),commands::games::minecraft_server(),commands::games::roblox_user(),commands::games::fortnite_news(),
 commands::social::github_profile(),commands::social::github_repo(),commands::social::urban(),
 ]).await?; tracing::info!("Meowy commands registered");Ok(())})).build();
 let mut client=serenity::ClientBuilder::new(config.discord_token,serenity::GatewayIntents::GUILDS).framework(framework).await?;
 client.start().await?;Ok(())
}
