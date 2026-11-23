use crate::{ui, Context, Error};
async fn get_json(url:&str)->Result<serde_json::Value,Error>{Ok(reqwest::Client::new().get(url).send().await?.error_for_status()?.json().await?)}
#[poise::command(slash_command)]
pub async fn minecraft_uuid(ctx:Context<'_>,#[description="Minecraft username"] username:String)->Result<(),Error>{
 let url=format!("https://api.mojang.com/users/profiles/minecraft/{}",urlencoding::encode(&username));
 match get_json(&url).await { Ok(v)=>ctx.send(ui::text(format!("## Minecraft profile\nName: {}\nUUID: {}",v["name"].as_str().unwrap_or(&username),v["id"].as_str().unwrap_or("Unknown")))).await?,Err(_)=>ctx.send(ui::text("Profile not found or service unavailable.")).await? }; Ok(())
}
#[poise::command(slash_command)]
pub async fn minecraft_server(ctx:Context<'_>,#[description="Server address"] address:String)->Result<(),Error>{
 let url=format!("https://api.mcsrvstat.us/3/{}",urlencoding::encode(&address));
 match get_json(&url).await { Ok(v)=>ctx.send(ui::text(format!("## Minecraft server\nAddress: {address}\nOnline: {}\nPlayers: {}\nVersion: {}",v["online"],v["players"]["online"],v["version"].as_str().unwrap_or("Unknown")))).await?,Err(_)=>ctx.send(ui::text("Server lookup failed.")).await? }; Ok(())
}
#[poise::command(slash_command)]
pub async fn roblox_user(ctx:Context<'_>,#[description="Roblox username"] username:String)->Result<(),Error>{
 let resp=reqwest::Client::new().post("https://users.roblox.com/v1/usernames/users").json(&serde_json::json!({"usernames":[username],"excludeBannedUsers":true})).send().await?;
 let body:serde_json::Value=resp.json().await?;
 if let Some(user)=body["data"].as_array().and_then(|a|a.first()){ctx.send(ui::text(format!("## Roblox user\nUsername: {}\nDisplay name: {}\nID: {}",user["name"],user["displayName"],user["id"]))).await?;}else{ctx.send(ui::text("User not found.")).await?;} Ok(())
}
#[poise::command(slash_command)]
pub async fn fortnite_news(ctx:Context<'_>)->Result<(),Error>{ctx.send(ui::text("Fortnite news data-source integration is queued; provider selection remains.")).await?;Ok(())}
