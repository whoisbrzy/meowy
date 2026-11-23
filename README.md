# Meowy — Rust rewrite

Modular Rust Discord bot using Poise/Serenity, Tokio, SQLite and Discord Components V2.

## Setup
1. Install stable Rust.
2. Copy `.env.example` to `.env` and set `DISCORD_TOKEN`.
3. Invite with `bot` and `applications.commands` scopes.
4. Run `cargo check`, then `cargo run`.

## Configuration
- `DISCORD_TOKEN`: required bot token.
- `MEOWY_DATABASE_PATH`: optional SQLite path (defaults to `meowy.sqlite3`).

## Architecture
- `src/config.rs`: validated environment config.
- `src/db.rs`: SQLite schema and basic settings/usage storage operations.
- `src/ui.rs`: shared Components V2 reply helper.
- `src/commands/`: command modules.

## Current command registrations
/ping, /echo, /coinflip, /color, /usage, /settings, /avatar, /userinfo, /guildinfo, /ai ask, /ai usage, /ai transcribe, /ai ocr, /minecraft_uuid, /minecraft_server, /roblox_user, /fortnite_news.

The command suite and integrations remain under active development. No build/test result is asserted until run in a Rust environment.

## API setup\nAI features require an OpenAI-compatible endpoint and API key: `AI_API_KEY`, optionally `AI_API_URL` and `AI_MODEL`. Provider failures are surfaced in V2 replies. OCR/transcription currently have provider configuration placeholders, not completed media processing.\n