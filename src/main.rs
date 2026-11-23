use poise::serenity_prelude as serenity;
use std::env;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, (), Error>;

#[poise::command(slash_command)]
async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("Pong!").await?;
    Ok(())
}

#[poise::command(slash_command)]
async fn echo(ctx: Context<'_>, #[description = "Text to send"] text: String) -> Result<(), Error> {
    ctx.say(text).await?;
    Ok(())
}

#[poise::command(slash_command)]
async fn coinflip(ctx: Context<'_>) -> Result<(), Error> {
    let result = if std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH)?.as_nanos() % 2 == 0 { "Heads" } else { "Tails" };
    ctx.say(result).await?;
    Ok(())
}

#[poise::command(slash_command)]
async fn color(ctx: Context<'_>, #[description = "Hex color, such as #5865F2"] value: String) -> Result<(), Error> {
    let cleaned = value.trim_start_matches('#');
    if cleaned.len() != 6 || !cleaned.chars().all(|c| c.is_ascii_hexdigit()) {
        ctx.say("Please provide a six-digit hex color (for example #5865F2).").await?;
        return Ok(());
    }
    let rgb = u32::from_str_radix(cleaned, 16)?;
    let embed = serenity::CreateEmbed::new().title(format!("#{cleaned}")).description(format!("RGB: {}, {}, {}", (rgb >> 16) & 255, (rgb >> 8) & 255, rgb & 255)).color(rgb);
    ctx.send(poise::CreateReply::default().embed(embed)).await?;
    Ok(())
}

#[poise::command(slash_command)]
async fn usage(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("Meowy is running. Usage tracking will be available after command handlers and provider integrations are implemented.").await?;
    Ok(())
}

#[poise::command(slash_command)]
async fn settings(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("Settings framework is planned; per-server preferences will be stored once the database layer is added.").await?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenvy::dotenv().ok();
    let token = env::var("DISCORD_TOKEN").expect("DISCORD_TOKEN must be set in .env");
    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![ping(), echo(), coinflip(), color(), usage(), settings()],
            ..Default::default()
        })
        .setup(|ctx, _ready, _framework| Box::pin(async move {
            poise::builtins::register_globally(ctx, &[
                ping(), echo(), coinflip(), color(), usage(), settings()
            ]).await?;
            Ok(())
        }))
        .build();
    let intents = serenity::GatewayIntents::GUILDS;
    let mut client = serenity::ClientBuilder::new(token, intents).framework(framework).await?;
    client.start().await?;
    Ok(())
}
