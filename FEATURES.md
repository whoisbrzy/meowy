# Meowy — Development Plan

## Target
Complete the original command suite in Rust with Poise/Serenity and Discord Components V2 for all bot responses. This is a phased implementation plan, not a claim that these items are complete.

## Phase 0 — Foundation and build verification
- [ ] Verify Rust toolchain and dependency compatibility; compile and resolve current errors.
- [ ] Establish project module structure and command registration.
- [ ] Centralize Components V2 layouts, error messages, pagination, buttons, modals and select menus.
- [ ] Configuration loading, validation, secret handling and startup diagnostics.
- [ ] Logging, error boundaries, cooldowns and rate-limit handling.
- [ ] Test strategy (unit tests, mock API tests, command smoke tests).

## Phase 1 — Core, persistence and command UX
- [ ] Database/persistence layer and migrations.
- [ ] User and server settings, configuration UI, permissions.
- [ ] Command help/discovery and usage analytics.
- [ ] Core commands: ping, echo, coinflip, color.
- [ ] Utility commands: asciify, translate, weather.
- [ ] Shared reusable API client, cache and pagination.
- [ ] Complete Components V2 UX, accessibility and interaction timeouts.

## Phase 2 — Discord and general utilities
- [ ] Avatar and banner.
- [ ] Public user and guild/server lookup.
- [ ] Server search and Discord-to-Roblox lookup.
- [ ] Permission-aware voice features and limited visible presence.
- [ ] Quote/message quote and clearly marked fake quote.
- [ ] Interactions/actions.
- [ ] Ship feature, framed as playful non-sensitive entertainment.

## Phase 3 — AI and media tools
- [ ] AI ask/chat.
- [ ] Imagine and image editing.
- [ ] OCR/extract text.
- [ ] Transcription.
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
- [ ] Server.
- [ ] Skin/skins.
- [ ] User and UUID.

### Roblox
- [ ] User and profile.
- [ ] Avatar.
- [ ] Game and server utilities.
- [ ] Other original list functions using documented public APIs.

### Rolimons
- [ ] Item information.
- [ ] User information.
- [ ] Value/list tools.

## Phase 5 — Social, developer and music integrations
### GitHub
- [ ] Profile.
- [ ] Repository.
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
- [ ] Urban Dictionary lookup.

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
