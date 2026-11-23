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
- `GROQ_API_KEY`: key for Groq AI.
- `GROQ_API_URL`: optional chat-completions endpoint.
- `GROQ_MODEL`: optional model (defaults to `llama-3.3-70b-versatile`).

## Architecture
- `src/config.rs`: configuration.
- `src/db.rs`: SQLite schema and basic storage operations.
- `src/ui.rs`: shared Components V2 reply helper.
- `src/commands/`: command modules.

## Additional integrations\n- Last.fm: recent tracks and top-list lookup (API key).\n- Spotify: metadata search (client credentials).\n- Instagram/TikTok/X/Telegram: public-profile/channel link helper; metadata access is limited and requires approved API access.\n\n## Registered commands
Core, Discord utilities, Groq-backed `/ai ask`, gaming lookups, `/github_profile`, `/github_repo`, and `/urban`.

Integrations remain in development; build and runtime validation have not yet been performed. OCR/transcription are placeholders.
