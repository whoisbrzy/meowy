use crate::{ui, Context, Error};
use serde::Deserialize;
use serde_json::json;

// Existing GitHub and Urban Dictionary commands retained.
#[derive(Deserialize)]
struct GithubUser { login:String, name:Option<String>, bio:Option<String>, html_url:String, public_repos:u32, followers:u32, following:u32 }
#[derive(Deserialize)]
struct GithubRepo { full_name:String, description:Option<String>, html_url:String, stargazers_count:u64, forks_count:u64, language:Option<String>, default_branch:String }

#[poise::command(slash_command)]
pub async fn github_profile(ctx:Context<'_>, #[description="GitHub username"] username:String)->Result<(),Error>{
 let r=reqwest::Client::new().get(format!("https://api.github.com/users/{}",urlencoding::encode(&username))).header("User-Agent","MeowyDiscordBot").send().await?;
 if !r.status().is_success(){ctx.send(ui::text(format!("GitHub returned HTTP {}.",r.status()))).await?;return Ok(())}
 let u:GithubUser=r.json().await?;
 ctx.send(ui::text(format!("## GitHub: {}\n{}\n{}\nRepositories: {} • Followers: {} • Following: {}\n{}",u.login,u.name.unwrap_or_default(),u.bio.unwrap_or_else(||"No bio.".into()),u.public_repos,u.followers,u.following,u.html_url))).await?;Ok(())
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
 let v:serde_json::Value=r.json().await?;let entry=v["list"].as_array().and_then(|a|a.first());
 if let Some(e)=entry{let definition=e["definition"].as_str().unwrap_or("No definition").chars().take(1700).collect::<String>();let example=e["example"].as_str().unwrap_or("").chars().take(500).collect::<String>();ctx.send(ui::text(format!("## {}\n{}\n\nExample: {}",term,definition,example))).await?;}else{ctx.send(ui::text("No entries found.")).await?;}Ok(())
}

#[poise::command(slash_command)]
pub async fn lastfm_recent(ctx:Context<'_>, #[description="Last.fm username"] username:String)->Result<(),Error>{
 let key=std::env::var("LASTFM_API_KEY").unwrap_or_default();
 if key.is_empty(){ctx.send(ui::text("Last.fm needs LASTFM_API_KEY in .env.")).await?;return Ok(())}
 let v:reqwest::Response=reqwest::Client::new().get("https://ws.audioscrobbler.com/2.0/").query(&[("method","user.getrecenttracks"),("user",username.as_str()),("api_key",key.as_str()),("format","json"),("limit","5")]).send().await?;
 if !v.status().is_success(){ctx.send(ui::text(format!("Last.fm HTTP {}.",v.status()))).await?;return Ok(())}
 let j:serde_json::Value=v.json().await?;
 let tracks=j["recenttracks"]["track"].as_array();
 if let Some(list)=tracks{let lines=list.iter().map(|t|format!("• {} — {}",t["name"].as_str().unwrap_or("?"),t["artist"]["#text"].as_str().unwrap_or("?"))).collect::<Vec<_>>().join("\n");ctx.send(ui::text(format!("## Recent tracks: {}\n{}",username,lines))).await?;}else{ctx.send(ui::text("No recent tracks returned.")).await?;}Ok(())
}
#[poise::command(slash_command)]
pub async fn lastfm_top(ctx:Context<'_>, #[description="Last.fm username"] username:String, #[description="Type: tracks, artists, or albums"] kind:String)->Result<(),Error>{
 let key=std::env::var("LASTFM_API_KEY").unwrap_or_default();if key.is_empty(){ctx.send(ui::text("Set LASTFM_API_KEY to enable Last.fm.")).await?;return Ok(())}
 let method=match kind.to_lowercase().as_str(){"tracks"=>"user.gettoptracks","artists"=>"user.gettopartists","albums"=>"user.gettopalbums",_=>{ctx.send(ui::text("Choose tracks, artists, or albums.")).await?;return Ok(())}};
 let r=reqwest::Client::new().get("https://ws.audioscrobbler.com/2.0/").query(&[("method",method),("user",username.as_str()),("api_key",key.as_str()),("format","json"),("limit","5")]).send().await?;
 let j:serde_json::Value=r.json().await?;let section=match kind.to_lowercase().as_str(){"tracks"=>"toptracks","artists"=>"topartists",_=>"topalbums"};let arr=j[section][kind.to_lowercase().trim_end_matches('s')].as_array().or_else(||j[section][kind.to_lowercase().trim_end_matches('s').to_string()+"s"].as_array());
 if let Some(items)=arr{let lines=items.iter().map(|x|format!("• {}",x["name"].as_str().unwrap_or("?"))).collect::<Vec<_>>().join("\n");ctx.send(ui::text(format!("## Top {} for {}\n{}",kind,username,lines))).await?;}else{ctx.send(ui::text("No results (or Last.fm API response could not be parsed).")).await?;}Ok(())
}
#[poise::command(slash_command)]
pub async fn spotify_search(ctx:Context<'_>, #[description="Track, artist or album query"] query:String)->Result<(),Error>{
 let id=std::env::var("SPOTIFY_CLIENT_ID").unwrap_or_default();let secret=std::env::var("SPOTIFY_CLIENT_SECRET").unwrap_or_default();
 if id.is_empty()||secret.is_empty(){ctx.send(ui::text("Spotify search requires SPOTIFY_CLIENT_ID and SPOTIFY_CLIENT_SECRET.")).await?;return Ok(())}
 let client=reqwest::Client::new();let token=client.post("https://accounts.spotify.com/api/token").basic_auth(id,Some(secret)).form(&[("grant_type","client_credentials")]).send().await?;
 if !token.status().is_success(){ctx.send(ui::text("Spotify token request failed.")).await?;return Ok(())}
 let t:serde_json::Value=token.json().await?;let bearer=t["access_token"].as_str().unwrap_or("");
 let r=client.get("https://api.spotify.com/v1/search").bearer_auth(bearer).query(&[("q",query.as_str()),("type","track,artist,album"),("limit","5")]).send().await?;
 if !r.status().is_success(){ctx.send(ui::text(format!("Spotify search returned HTTP {}.",r.status()))).await?;return Ok(())}
 let j:serde_json::Value=r.json().await?;let mut out=String::from("## Spotify search\n");
 for typ in ["tracks","artists","albums"]{if let Some(items)=j[typ]["items"].as_array(){for x in items.iter().take(2){out.push_str(&format!("• {} — {}\n{}\n",x["name"].as_str().unwrap_or("?"),typ,x["external_urls"]["spotify"].as_str().unwrap_or(""));)}}}
 ctx.send(ui::text(out)).await?;Ok(())
}

// These provide canonical public-profile links, not scraping or access-control bypass.
#[poise::command(slash_command)]
pub async fn instagram(ctx:Context<'_>, #[description="Public Instagram username"] username:String)->Result<(),Error>{let u=username.trim_start_matches('@');if u.contains('/')||u.contains(' ')||u.is_empty(){ctx.send(ui::text("Enter a username, not a URL.")).await?;return Ok(())}ctx.send(ui::text(format!("## Instagram\nPublic profile link: https://www.instagram.com/{}/\nDirect profile/post data requires supported API access.",u))).await?;Ok(())}
#[poise::command(slash_command)]
pub async fn tiktok(ctx:Context<'_>, #[description="Public TikTok username"] username:String)->Result<(),Error>{let u=username.trim_start_matches('@');if u.contains('/')||u.contains(' ')||u.is_empty(){ctx.send(ui::text("Enter a username, not a URL.")).await?;return Ok(())}ctx.send(ui::text(format!("## TikTok\nPublic profile link: https://www.tiktok.com/@{}\nProfile metadata retrieval requires supported API access.",u))).await?;Ok(())}
#[poise::command(slash_command)]
pub async fn x_profile(ctx:Context<'_>, #[description="Public X username"] username:String)->Result<(),Error>{let u=username.trim_start_matches('@');if u.contains('/')||u.contains(' ')||u.is_empty(){ctx.send(ui::text("Enter a username, not a URL.")).await?;return Ok(())}ctx.send(ui::text(format!("## X\nPublic profile link: https://x.com/{}\nData retrieval requires official API access.",u))).await?;Ok(())}
#[poise::command(slash_command)]
pub async fn telegram(ctx:Context<'_>, #[description="Public Telegram channel username"] username:String)->Result<(),Error>{let u=username.trim_start_matches('@');if u.contains('/')||u.contains(' ')||u.is_empty(){ctx.send(ui::text("Enter a public channel username.")).await?;return Ok(())}ctx.send(ui::text(format!("## Telegram\nPublic channel link: https://t.me/{}\nChannel details/content retrieval requires bot/API access and permissions.",u))).await?;Ok(())}
