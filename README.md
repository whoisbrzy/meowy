# Meowy

A Discord utility bot project. This repository is the implementation home for the command specification supplied in chat.

## Setup
1. Install Node.js 20+.
2. Run `npm install`.
3. Create `.env` with `DISCORD_TOKEN`, `CLIENT_ID`, and optionally `GUILD_ID`.
4. Run `npm run deploy` to register commands (guild registration if `GUILD_ID` is set, global otherwise).
5. Run `npm start`.

External APIs (AI, Last.fm, Spotify, game data, crypto, etc.) require separate providers and credentials. See `.env.example`.

## Scope
The supplied feature list is tracked in `FEATURES.md`. The initial build includes the Discord command framework and some local commands. Feature integrations will be added as credentials and provider choices are configured.

Commands that infer sensitive traits, invade privacy, or promote nicotine use are intentionally excluded or redesigned.
