# Social API foundation

`apps/app-frontend/src/social.ts` defines users, friends, requests, direct messages, presence, and events. The `socialUnavailable` adapter returns empty read-only collections and rejects writes. No chat UI, registration, database, socket, or Millida endpoint is connected.

A future implementation should authenticate to an UrodLauncher service separately from Minecraft credentials, validate every event and message at the server boundary, limit message size and rate, support block/report controls, and never expose Microsoft or Ely.by tokens to social code. Replace the adapter behind `SocialApi` after a documented server protocol and privacy policy exist.
