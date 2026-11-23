use crate::{ui, Context, Error};
use serde_json::json;

#[poise::command(slash_command)]
pub async fn ask(ctx: Context<'_>, #[description="Prompt"] prompt: String) -> Result<(), Error> {
    let key=std::env::var("GROQ_API_KEY").unwrap_or_default();
    let endpoint=std::env::var("GROQ_API_URL").unwrap_or_else(|_|"https://api.groq.com/openai/v1/chat/completions".into());
    let model=std::env::var("GROQ_MODEL").unwrap_or_else(|_|"llama-3.3-70b-versatile".into());
    if key.is_empty() { ctx.send(ui::text("Groq is not configured. Set GROQ_API_KEY, optionally GROQ_API_URL / GROQ_MODEL.")).await?; return Ok(()); }
    let response=reqwest::Client::new().post(endpoint).bearer_auth(key).json(&json!({"model":model,"messages":[{"role":"user","content":prompt}]})).send().await?;
    if !response.status().is_success() { ctx.send(ui::text(format!("Groq returned HTTP {}.",response.status()))).await?; return Ok(()); }
    let body:serde_json::Value=response.json().await?;
    let answer=body["choices"][0]["message"]["content"].as_str().unwrap_or("No response text returned.");
    ctx.send(ui::text(answer.chars().take(3800).collect::<String>())).await?;
    Ok(())
}
#[poise::command(slash_command)]
pub async fn ai_usage(ctx: Context<'_>) -> Result<(), Error> { ctx.send(ui::text("## AI usage\nUsage counters will be connected to analytics.")).await?; Ok(()) }
#[poise::command(slash_command)]
pub async fn transcribe(ctx: Context<'_>, #[description="Audio attachment"] attachment: poise::serenity_prelude::Attachment) -> Result<(), Error> {
    let _=attachment; ctx.send(ui::text("Transcription processing is not implemented yet.")).await?; Ok(())
}
#[poise::command(slash_command)]
pub async fn ocr(ctx: Context<'_>, #[description="Image attachment"] attachment: poise::serenity_prelude::Attachment) -> Result<(), Error> {
    let _=attachment; ctx.send(ui::text("OCR processing is not implemented yet.")).await?; Ok(())
}
