use poise::serenity_prelude as serenity;
use crate::{db::Database, ui, Error, Context};

#[poise::command(slash_command)]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    ctx.send(ui::text("# Meowy\nPong!")).await?;
    Ok(())
}

#[poise::command(slash_command)]
pub async fn echo(ctx: Context<'_>, #[description = "Text to send"] text: String) -> Result<(), Error> {
    ctx.send(ui::text(text)).await?;
    Ok(())
}

#[poise::command(slash_command)]
pub async fn coinflip(ctx: Context<'_>, #[description = "Number of flips"] #[min = 1] #[max = 20] rounds: Option<u8>) -> Result<(), Error> {
    let rounds = rounds.unwrap_or(1);
    let heads = (0..rounds).filter(|_| rand::random::<bool>()).count() as u8;
    ctx.send(ui::text(format!("## Coin flip\n**{rounds}** flips\nHeads: **{heads}**\nTails: **{}**", rounds-heads))).await?;
    Ok(())
}

#[poise::command(slash_command)]
pub async fn color(ctx: Context<'_>, #[description = "Hex color (e.g. #5865F2)"] value: String) -> Result<(), Error> {
    let cleaned = value.trim_start_matches('#');
    if cleaned.len()!=6 || !cleaned.chars().all(|c| c.is_ascii_hexdigit()) {
        ctx.send(ui::text("## Color\nEnter a valid six-digit hexadecimal value.")).await?;
        return Ok(());
    }
    let rgb=u32::from_str_radix(cleaned,16)?;
    ctx.send(ui::text(format!("## #{cleaned}\nRGB: {}, {}, {}\nDecimal: {rgb}", (rgb>>16)&255,(rgb>>8)&255,rgb&255))).await?;
    Ok(())
}

#[poise::command(slash_command)]
pub async fn usage(ctx: Context<'_>) -> Result<(), Error> {
    // Storage integration will be wired into shared bot state after framework setup.
    ctx.send(ui::text("## Usage\nUsage reporting is being connected to the SQLite storage layer.")).await?;
    Ok(())
}

#[poise::command(slash_command)]
pub async fn settings(ctx: Context<'_>) -> Result<(), Error> {
    ctx.send(ui::text("## Settings\nSettings storage is available in the database module; interactive configuration is coming next.")).await?;
    Ok(())
}
