# Meowy — Development Plan

## Target
Complete the original command suite in Rust with Poise/Serenity and Discord Components V2 for all bot responses. This is a phased implementation plan, not a claim that these items are complete.

## Phase 0 — Foundation and build verification
- [ ] Verify Rust toolchain and dependency compatibility; compile and resolve current errors.
- [ ] Establish project module structure and command registration.
- [x] Centralize the base Components V2 Container/TextDisplay layout helper.\n- [ ] Extend helper with pagination, buttons, modals and select menus.
- [x] Configuration loading, token validation and startup diagnostics.
- [x] Structured logging foundation.\n- [ ] Error boundaries, cooldowns and rate-limit handling.
- [ ] Test strategy (unit tests, mock API tests, command smoke tests).

## Phase 1 — Core, persistence and command UX
- [x] SQLite persistence foundation/schema for settings and usage.\n- [ ] Migration management.
- [ ] User and server settings, configuration UI, permissions.
- [ ] Command help/discovery and usage analytics.
- [x] Core commands: ping, echo, coinflip, color.
- [ ] Utility commands: asciify, translate, weather.
- [ ] Shared reusable API client, cache and pagination.
- [x] Shared base Components V2 response helper.\n- [ ] Complete V2 UX, accessibility and interaction timeouts.

## Phase 2 — Discord and general utilities
- [x] Avatar lookup.\n- [ ] Banner lookup.
- [x] Userinfo and guild/server info (cache-limited).
- [ ] Server search and Discord-to-Roblox lookup.
- [ ] Permission-aware voice features and limited visible presence.
- [ ] Quote/message quote and clearly marked fake quote.
- [ ] Interactions/actions.
- [ ] Ship feature, framed as playful non-sensitive entertainment.

## Phase 3 — AI and media tools
- [x] AI ask/chat (OpenAI-compatible endpoint).
- [ ] Imagine and image editing.
- [ ] OCR/extract text (placeholder; processing pending).
- [ ] Transcription (placeholder; processing pending).
- [ ] Translation/context-menu translation.
- [ ] TTS.
- [ ] AI roast with safeguards.
- [ ] AI usage tracking.
- [ ] Context menu actions for image edit, text extraction, transcription and translation.

## Phase 4 — Gaming integrations
### Fortnite
- [ ] Cosmetics.
- [ ] Map.
- [ ] News.
- [ ] Season.
- [ ] Shop.
- [ ] Stats.
- [ ] Weapons.

### Valorant
- [ ] Agent, map, rank and game information utilities from documented data sources.

### Minecraft
- [ ] Body/head render helpers.
- [ ] Cape/capes.
- [ ] Compare.
- [ ] History.
- [x] Minecraft server status lookup.
- [ ] Skin/skins.
- [x] Minecraft username-to-UUID lookup.

### Roblox
- [x] Roblox username lookup.
- [ ] Avatar.
- [ ] Game and server utilities.
- [ ] Other original list functions using documented public APIs.

### Rolimons
- [ ] Item information.
- [ ] User information.
- [ ] Value/list tools.

## Phase 5 — Social, developer and music integrations
### GitHub
- [x] Profile.
- [x] Repository.
- [ ] Public user lookup, including public account metadata (not private email discovery).

### Instagram
- [ ] Public profile lookup where supported.
- [ ] Public post lookup where supported.

### TikTok, X and Telegram
- [ ] Public profile/post/content lookup, subject to provider access and platform restrictions.

### Last.fm
- [ ] User.
- [ ] Recent tracks.
- [ ] Top tracks/artists/albums.
- [ ] Track, artist and album info.

### Spotify
- [ ] Track, artist, album and playlist metadata as supported by official API.

### Music/text
- [ ] Lyrics lookup using licensed/allowed providers and brief excerpts only.
- [x] Urban Dictionary lookup.

## Phase 6 — Crypto and data utilities
- [ ] Gas prices.
- [ ] Crypto prices.
- [ ] Exchange rates.
- [ ] Transaction lookup.
- [ ] Wallet lookup for public addresses only (no signing, custody, or secret collection).

## Phase 7 — Quality, release and maintenance
- [ ] Complete command registration and discoverability.
- [ ] Check permissions, privacy and API terms for each integration.
- [ ] API mocks and tests for error cases, empty responses and rate limits.
- [ ] Verify V2 responses across commands and interaction types.
- [ ] Documentation and setup guide, API credential matrix and .env.example.
- [ ] Release build, deployment instructions and operational checklist.

## Guardrails and scope notes
- Do not implement sensitive-trait inference (e.g. sexuality or disability guessing).
- Exclude nicotine promotion.
- Do not enable covert tracking of voice, online presence or users; respect visibility, permissions and platform restrictions.
- Do not circumvent authentication, paywalls, access controls or rate limits.
- API-dependent features are only complete when they perform real requests, handle failures and are documented; stubs are not completion.
- Keep all interactive bot replies in Discord Components V2 format.

## Current work snapshot\n- Modular files created: config, database, UI helper and command module.\n- SQLite schema initializes at startup; settings storage has basic methods and a round-trip test.\n- Usage/settings commands still need shared database state wiring and functional UI.\n- Compilation and tests have not yet been run in a Rust environment.\n
## Recent progress (Phase 2-4 partial)
- Registered Discord avatar/user/server lookup commands.
- Added configurable AI chat command.
- Added OCR and transcription placeholders.
- Added Minecraft username UUID and server status lookups, plus Roblox username lookup.
- Fortnite news remains a placeholder; other gaming integrations pending.
- Compilation/runtime validation has not been performed.
