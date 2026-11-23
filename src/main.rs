use poise::serenity_prelude as serenity;
use std::env;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, (), Error>;

// Components V2 message helper: text is wrapped in a Container/TextDisplay and
// the IS_COMPONENTS_V2 flag is always set. No legacy content or embeds.
fn v2(text: impl Into<String>) -> poise::CreateReply {
    let children = vec![serenity::CreateContainerComponent::TextDisplay(
        serenity::CreateTextDisplay::new(text.into()),
    )];
    poise::CreateReply::default()
        .flags(serenity::MessageFlags::IS_COMPONENTS_V2)
        .components(vec![serenity::CreateComponent::Container(
            serenity::CreateContainer::new(children),
        )])
}

#[poise::command(slash_command)]
async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    ctx.send(v2("# Meowy\nPong!")).await?;
    Ok(())
}

#[poise::command(slash_command)]
async fn echo(ctx: Context<'_>, #[description = "Text to send"] text: String) -> Result<(), Error> {
    ctx.send(v2(text)).await?;
    Ok(())
}

#[poise::command(slash_command)]
async fn coinflip(ctx: Context<'_>, #[description = "Number of flips"] #[min = 1] #[max = 20] rounds: Option<u8>) -> Result<(), Error> {
    let rounds = rounds.unwrap_or(1);
    let mut heads = 0u8;
    for _ in 0..rounds {
        if std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH)?.as_nanos() % 2 == 0 { heads += 1; }
    }
    ctx.send(v2(format!("## Coin flip\n**{rounds}** flip(s) completed.\nHeads: **{heads}**\nTails: **{}**", rounds-heads))).await?;
    Ok(())
}

#[poise::command(slash_command)]
async fn color(ctx: Context<'_>, #[description = "Hex color (e.g. #5865F2)"] value: String) -> Result<(), Error> {
    let cleaned = value.trim_start_matches('#');
    if cleaned.len()!=6 || !cleaned.chars().all(|c|c.is_ascii_hexdigit()) {
        ctx.send(v2("## Color\nEnter a valid six-digit hexadecimal value.")).await?;
        return Ok(());
    }
    let rgb=u32::from_str_radix(cleaned,16)?;
    ctx.send(v2(format!("## #{cleaned}\nRGB: {}, {}, {}\nDecimal: {rgb}", (rgb>>16)&255,(rgb>>8)&255,rgb&255))).await?;
    Ok(())
}

#[poise::command(slash_command)]
async fn usage(ctx: Context<'_>) -> Result<(), Error> {
    ctx.send(v2("## Meowy usage\nUsage analytics will be added with persistent storage.")).await?;
    Ok(())
}

#[poise::command(slash_command)]
async fn settings(ctx: Context<'_>) -> Result<(), Error> {
    ctx.send(v2("## Settings\nServer settings are not yet persisted; configuration module is planned.")).await?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenvy::dotenv().ok();
    let token=env::var("DISCORD_TOKEN").expect("DISCORD_TOKEN must be set");
    let commands=vec![ping(),echo(),coinflip(),color(),usage(),settings()];
    let framework=poise::Framework::builder().options(poise::FrameworkOptions {
        commands: commands.clone(),
        ..Default::default()
    }).setup(|ctx,_ready,_framework|Box::pin(async move {
        poise::builtins::register_globally(ctx,&[ping(),echo(),coinflip(),color(),usage(),settings()]).await?;
        Ok(())
    })).build();
    let intents=serenity::GatewayIntents::GUILDS;
    let mut client=serenity::ClientBuilder::new(token,intents).framework(framework).await?;
    client.start().await?;
    Ok(())
}
