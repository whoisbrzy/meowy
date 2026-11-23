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
/ping, /echo, /coinflip, /color, /usage, /settings.

The command suite and integrations remain under active development. No build/test result is asserted until run in a Rust environment.
