# UrodLauncher 1.0.0 Stable readiness audit

Status on 2026-10-18: **release blocked; the repository remains version 0.1.0 Development**. Work is on `release/1.0.0-figma-stable`. No `v1.0.0` tag, Stable package, or GitHub Release has been published.

| Gate | Evidence | Result |
| --- | --- | --- |
| Source preservation | Started from clean `main` at `3c4c189`; worked on a separate branch; existing data migration unchanged | Pass |
| Figma inventory | MCP `get_metadata` returned 23 top-level frames; `get_design_context` and screenshots for all listed top-level frames are retained in `docs/figma-reference` | Pass for extraction; child component analysis continues |
| Settings design | All 16 settings contexts and original screenshots saved in `docs/figma-reference`, including `3:104`, `3:199`, and `3:402` | Pass for extraction; transfer and comparison incomplete |
| Visual transfer | Shell, library, gallery, skins, dialogs, global settings shell and pack settings shell use exported Figma assets; [comparison map](FIGMA_IMPLEMENTATION_MAP.md) includes real desktop captures and measured differences | Partial: exact subframes and several settings controls remain unimplemented |
| Functional skin library | PNG upload, safe native validation, model change and persistence exercised in isolated Tauri data | Pass for those paths; Microsoft live upload unavailable |
| Settings persistence | Global RAM changed to 6144 MB and survived WebView reload; pack RAM changed to 6144 MB, pack renamed and duplicated through the new native UI | Pass for these paths in isolated data |
| Frontend check | TypeScript, ESLint, Prettier and 18 tests, including shader/resource pack compatibility | Pass |
| Rust check | `cargo check --locked -p urod-launcher` | Pass |
| Rust lint | `cargo clippy --locked -p urod-launcher --all-targets -- -D warnings` | Pass after fixing seven redundant borrows in the vendored Daedalus adapter |
| Rust tests | `cargo test --locked --workspace` | Pass for enabled tests; ignored live tests run separately |
| Native boundary | [Boundary audit](../scripts/check-stage2-boundary.mjs) and release asset script test | Pass after the latest settings UI changes |
| Installation and launch | Isolated live Vanilla, Fabric, Quilt, Forge and NeoForge 1.21.1 installed, started, remained running and stopped; Vanilla also repeated launch and displayed a real game window | Pass for these loader/version combinations only |
| Installer matrix | [CI run 39104625372](https://github.com/Yozekkk/urod-launcher/actions/runs/39104625372) built Windows NSIS, built and installed Debian, and built and installed Arch for current 0.1.0 metadata; Windows execution and all 1.0.0 packages remain open | **Partial** |
| Selected-pack content | Installed Fabric API, a shader and a resource pack through live Modrinth; filtered each kind and persisted a resource pack toggle | Pass for those paths; data pack tab needs world targeting |
| Resource settings | Native comparison with Figma `3:199` at 837×470 and inspection at 860×620; accurate data directory shown | Partial: safe cache deletion and extended report commands do not exist |
| Java locations | Figma `3:104` requires independent Java 25 and 21 paths, but backend persists only one global path | **Blocked for exact functionality** |
| Real accounts | No owner Microsoft/Ely.by credentials used | **Not run** |
| Signed updater | No UrodLauncher signing key or endpoint; updater is disabled | Safe state, not a completed update feature |

The release remains blocked until the remaining Figma screens and settings are implemented and compared, native installer scenarios pass, and account checks are performed with authorized credentials. The current UI has no real game log sidebar and cannot persist a separate Java runtime per required major. The existing 0.1.0 metadata is intentional while these gates are open. Do not retag it as Stable.
