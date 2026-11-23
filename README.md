# Meowy (Rust)

Meowy is migrating to Rust using Poise (Serenity) and Tokio.

## Requirements
- Rust stable toolchain
- Discord application and bot token

## Setup
1. Copy `.env.example` to `.env` and set `DISCORD_TOKEN`.
2. Invite the bot with the `bot` and `applications.commands` scopes.
3. Run:

```sh
cargo check
cargo run
```

Currently implemented: `/ping`, `/echo`, `/coinflip`, `/color`, `/usage`, `/settings`.

Other commands from the Evade-inspired specification are being migrated and need service integrations and/or credentials. Registered or planned commands are not claimed to be fully functional. Privacy-sensitive or harmful commands are excluded.

Use `cargo fmt` and `cargo check` before committing.
