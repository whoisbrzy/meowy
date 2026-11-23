# Meowy — Rust rewrite

The project is being rewritten in Rust. The old Node/discord.js runtime is retired.

## Stack
- Rust stable
- Poise / Serenity
- Tokio
- Reqwest and Serde for API integrations

## Run
1. Install Rust stable.
2. Copy `.env.example` to `.env` and set `DISCORD_TOKEN`.
3. Invite the bot with `bot` and `applications.commands` scopes.
4. Run `cargo check`, then `cargo run`.

## Current implemented commands
- `/ping`
- `/echo`
- `/coinflip rounds`
- `/color` (text-based hex information, no embed sidebar)
- `/usage` (stub)
- `/settings` (stub)

## Components V2
The bot avoids legacy embed color/sidebar styling. Responses currently use plain Discord messages; Components V2 layouts will be introduced using the API's component-based message layout rather than embed color bars.

## Status
This is an ongoing rewrite. External API-backed features are not yet implemented. Commands involving sensitive attribute inference, nicotine use, invasive tracking, or bypassing platform privacy are excluded or redesigned.
