# Meowy — Full command implementation tracker

All responses should use Discord Components V2. Rust/Poise implementation is the target.

## Core & utility
- [x] /ping
- [x] /echo
- [x] /coinflip (simulation)
- [x] /color
- [ ] /usage (persistent analytics)
- [ ] /settings (persistent server/user configuration)
- [ ] /asciify
- [ ] /translate
- [ ] /weather

## AI & media
- [ ] /ai ask
- [ ] /ai edit-image
- [ ] /ai imagine
- [ ] /ai ocr
- [ ] /ai roast
- [ ] /ai transcribe
- [ ] /ai tts
- [ ] /ai usage
- [ ] Context menu: AI edit
- [ ] Context menu: extract text
- [ ] Context menu: transcribe
- [ ] Context menu: translate

## Discord utilities
- [ ] Avatar lookup
- [ ] Banner lookup
- [ ] Guild/server lookup
- [ ] Public presence where visible and permitted
- [ ] Public user lookup
- [ ] Server search
- [ ] Discord-to-Roblox lookup
- [ ] Voice utilities (permission-aware; no covert tracking)
- [ ] Quote message
- [ ] Fake quote (clearly labeled simulation)

## Crypto
- [ ] Gas
- [ ] Price
- [ ] Rates
- [ ] Transaction lookup
- [ ] Wallet public-address lookup

## Fortnite
- [ ] Cosmetics
- [ ] Map
- [ ] News
- [ ] Season
- [ ] Shop
- [ ] Stats
- [ ] Weapons

## GitHub
- [ ] Username/email public profile lookup
- [ ] Profile
- [ ] Repository

## Public social integrations
- [ ] Instagram public post lookup
- [ ] Instagram public profile lookup
- [ ] TikTok public data
- [ ] X public data
- [ ] Telegram public data

## Last.fm
- [ ] User
- [ ] Recent tracks
- [ ] Top tracks
- [ ] Top artists
- [ ] Top albums
- [ ] Track info
- [ ] Artist info
- [ ] Album info

## Music & text
- [ ] Lyrics
- [ ] Spotify
- [ ] Urban Dictionary

## Minecraft
- [ ] Body
- [ ] Cape
- [ ] Capes
- [ ] Compare
- [ ] Head
- [ ] History
- [ ] Server
- [ ] Skin
- [ ] Skins
- [ ] User
- [ ] UUID

## Roblox
- [ ] User
- [ ] Avatar
- [ ] Profile
- [ ] Game lookup
- [ ] Server lookup
- [ ] Other original list utilities

## Rolimons
- [ ] Item lookup
- [ ] User lookup
- [ ] Value/list utilities

## Other
- [ ] Interact actions (non-romantic, non-sexual)
- [ ] Ship (fictional/lighthearted compatibility, not sensitive inference)
- [ ] Quote
- [ ] Valorant utilities

## Implementation notes
- Sensitive-trait guessing commands (e.g. sexuality or disability inference) are not implemented.
- Nicotine-promoting commands are not implemented.
- Presence/voice or other data must not enable covert tracking or circumvent privacy controls.
- Third-party integrations require their API endpoints, rate limits, and credentials where applicable.
- Items above are tracking targets, not claims that they're currently implemented.
