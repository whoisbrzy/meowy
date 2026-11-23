use crate::{ui, Context, Error};
use serde::Deserialize;

#[derive(Deserialize)]
struct GithubUser { login:String, name:Option<String>, bio:Option<String>, html_url:String, public_repos:u32, followers:u32, following:u32 }
#[derive(Deserialize)]
struct GithubRepo { full_name:String, description:Option<String>, html_url:String, stargazers_count:u64, forks_count:u64, language:Option<String>, default_branch:String }

#[poise::command(slash_command)]
pub async fn github_profile(ctx:Context<'_>, #[description="GitHub username"] username:String)->Result<(),Error>{
 let client=reqwest::Client::new();
 let r=client.get(format!("https://api.github.com/users/{}",urlencoding::encode(&username))).header("User-Agent","MeowyDiscordBot").send().await?;
 if !r.status().is_success(){ctx.send(ui::text(format!("GitHub returned HTTP {}.",r.status()))).await?;return Ok(())}
 let u:GithubUser=r.json().await?;
 ctx.send(ui::text(format!("## GitHub: {}\n{}\n{}\n{}\nRepositories: {} • Followers: {} • Following: {}\n{}",u.login,u.name.unwrap_or_default(),u.bio.unwrap_or_else(||"No bio.".into()),u.html_url,u.public_repos,u.followers,u.following,u.html_url))).await?;Ok(())
}
#[poise::command(slash_command)]
pub async fn github_repo(ctx:Context<'_>, #[description="Repository in owner/repo format"] repository:String)->Result<(),Error>{
 let r=reqwest::Client::new().get(format!("https://api.github.com/repos/{}",repository)).header("User-Agent","MeowyDiscordBot").send().await?;
 if !r.status().is_success(){ctx.send(ui::text(format!("GitHub returned HTTP {}.",r.status()))).await?;return Ok(())}
 let repo:GithubRepo=r.json().await?;
 ctx.send(ui::text(format!("## {}\n{}\n{}\nStars: {} • Forks: {}\nLanguage: {} • Default branch: {}",repo.full_name,repo.description.unwrap_or_else(||"No description.".into()),repo.html_url,repo.stargazers_count,repo.forks_count,repo.language.unwrap_or_else(||"Unknown".into()),repo.default_branch))).await?;Ok(())
}
#[poise::command(slash_command)]
pub async fn urban(ctx:Context<'_>, #[description="Term to search"] term:String)->Result<(),Error>{
 let r=reqwest::Client::new().get("https://api.urbandictionary.com/v0/define").query(&[("term",term.as_str())]).send().await?;
 if !r.status().is_success(){ctx.send(ui::text("Urban Dictionary lookup failed.")).await?;return Ok(())}
 let v:serde_json::Value=r.json().await?;
 let entry=v["list"].as_array().and_then(|a|a.first());
 if let Some(e)=entry{let definition=e["definition"].as_str().unwrap_or("No definition").chars().take(1700).collect::<String>();let example=e["example"].as_str().unwrap_or("").chars().take(500).collect::<String>();ctx.send(ui::text(format!("## {}\n{}\n\nExample: {}",term,definition,example))).await?;}else{ctx.send(ui::text("No entries found.")).await?;}Ok(())
}
