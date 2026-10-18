# UrodLauncher 1.0.0 Stable readiness audit

Status on 2026-10-18: **release blocked; the repository remains version 0.1.0 Development**. Work is on `release/1.0.0-figma-stable`. No `v1.0.0` tag, Stable package, or GitHub Release has been published.

| Gate | Evidence | Result |
| --- | --- | --- |
| Source preservation | Started from clean `main` at `3c4c189`; worked on a separate branch; existing data migration unchanged | Pass |
| Figma inventory | MCP `get_metadata` returned 23 top-level frames; `get_design_context` was obtained for main screens and 15 of 16 settings frames | Partial: `3:402` remains unavailable after the Starter-plan limit recurred |
| Settings design | Thirteen settings contexts and original screenshots saved in `docs/figma-reference`; earlier `3:104` and `3:199` calls succeeded but their raw context was not retained | Partial |
| Visual transfer | Shell, library, gallery, skins, dialogs, global settings shell and pack settings shell use exported Figma assets; [comparison map](FIGMA_IMPLEMENTATION_MAP.md) includes real desktop captures and measured differences | Partial: exact subframes and several settings controls remain unimplemented |
| Functional skin library | PNG upload, safe native validation, model change and persistence exercised in isolated Tauri data | Pass for those paths; Microsoft live upload unavailable |
| Settings persistence | Global RAM changed to 6144 MB and survived WebView reload; pack RAM changed to 6144 MB, pack renamed and duplicated through the new native UI | Pass for these paths in isolated data |
| Frontend check | TypeScript, ESLint, Prettier and 17 tests | Pass |
| Rust check | `cargo check --locked -p urod-launcher` | Pass |
| Rust lint | `cargo clippy --locked -p urod-launcher --all-targets -- -D warnings` | Pass after fixing seven redundant borrows in the vendored Daedalus adapter |
| Rust tests | `cargo test --locked --workspace` | Pass for enabled tests; ignored live tests run separately |
| Native boundary | [Boundary audit](../scripts/check-stage2-boundary.mjs) and release asset script test | Pass after the latest settings UI changes |
| Installation and launch | Isolated live Vanilla, Fabric, Quilt, Forge and NeoForge 1.21.1 installed, started, remained running and stopped; Vanilla also repeated launch and displayed a real game window | Pass for these loader/version combinations only |
| Installer matrix | Current 0.1.0 Arch candidate builds and installs/removes in an isolated root; Windows, Debian and all 1.0.0 package tests remain open | **Partial** |
| Real accounts | No owner Microsoft/Ely.by credentials used | **Not run** |
| Signed updater | No UrodLauncher signing key or endpoint; updater is disabled | Safe state, not a completed update feature |

The release remains blocked until the remaining Figma frame can be read and all settings implemented and compared, native installer scenarios pass, and account checks are performed with authorized credentials. The existing 0.1.0 metadata is intentional while these gates are open. Do not retag it as Stable.
