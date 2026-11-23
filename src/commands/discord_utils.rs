use crate::{ui, Context, Error};
use poise::serenity_prelude as serenity;

#[poise::command(slash_command)]
pub async fn avatar(ctx: Context<'_>, #[description="User to look up"] user: serenity::User) -> Result<(), Error> {
    let url=user.avatar_url().unwrap_or_else(|| user.default_avatar_url());
    ctx.send(ui::text(format!("## Avatar — {}\n{url}", user.name))).await?;
    Ok(())
}

#[poise::command(slash_command)]
pub async fn userinfo(ctx: Context<'_>, #[description="User to look up"] user: serenity::User) -> Result<(), Error> {
    ctx.send(ui::text(format!("## User information\n**Name:** {}\n**ID:** {}\n**Avatar:** {}",user.name,user.id,user.avatar_url().unwrap_or_else(|| user.default_avatar_url())))).await?;
    Ok(())
}

#[poise::command(slash_command)]
pub async fn guildinfo(ctx: Context<'_>) -> Result<(), Error> {
    let Some(guild_id)=ctx.guild_id() else {
        ctx.send(ui::text("This command must be used in a server.")).await?;
        return Ok(());
    };
    let guild=ctx.guild().ok_or("Guild data is unavailable")?;
    ctx.send(ui::text(format!("## Server information\n**Name:** {}\n**ID:** {}\n**Cached member count:** {}\n**Owner:** {}",guild.name,guild_id,guild.member_count.unwrap_or_default(),guild.owner_id))).await?;
    Ok(())
}
