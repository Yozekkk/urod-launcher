# Changelog

## Unreleased work toward 1.0.0

- Imported original UrodLauncher Figma SVG/PNG assets and the Geologica/Inter fonts with OFL notices.
- Replaced the main shell with Figma sidebar, header and news rail; reworked library and live Modrinth gallery geometry.
- Reworked the skin page and add/edit dialogs using the inspected Figma frames. PNG upload, native validation, model choice and persistence remain functional.
- Transferred the pack content manager and selected-pack Modrinth browser from Figma frames `3:713` and `3:888`, retaining live installed content and real catalog data. Added shader and resource pack search, compatibility checks and installation through the existing backend.
- Fixed direct links to installed shaders and resource packs so their Modrinth type is recovered from project metadata even when the URL has no type parameter.
- Added Figma-based global and pack settings windows. Global RAM, per-pack RAM, renaming and duplication were exercised in the native app. Unsupported sync, repair, fullscreen and command controls are explicitly unavailable.
- Added the real Minecraft log to the selected-pack sidebar, with bounded log reads, common credential redaction, searchable severity filters and a compact drawer at narrow window sizes. Corrected offline log capture so the placeholder token `0` no longer corrupts version numbers.
- Matched global settings frames `3:280`, `3:339`, `3:367` and `3:402` against native captures. Window width and height now persist as a validated pair and reach the Minecraft process as numeric arguments; a Fabric game window opened with a configured pair.
- Added per-pack window dimension overrides from Figma `3:463`, including validation, persistence, priority over global dimensions and inheritance when the override is disabled. Both paths were exercised with a real Fabric launch.
- Transferred pack settings frames `3:530`, `3:588` and `3:629`. Java detection now chooses the major required by the selected pack and its override can be saved or cleared; launch command inputs remain disabled until safe backend support exists.
- Matched the resource settings layout from Figma `3:199` against a native Tauri capture. The cache and extended report controls are disabled until backend operations exist.
- Added separate, validated Java 21 and 25 paths from Figma `3:104`, with runtime detection, native path selection and launch-time selection. Per-pack Java overrides retain priority; a global default no longer becomes a saved pack override during launch.
- Verified live install, start and stop for Minecraft 1.21.1 with Vanilla, Fabric, Quilt, Forge and NeoForge; repeated the Vanilla launch.
- Patched the transitive `source-map-js` dependency to 1.2.2 after a production dependency audit found GHSA-68fv-2mgg-jv7q.
- Replaced bundled desktop icons with rasterizations of the original Figma launcher mark.
- Corrected active `AGENTS.md` project instructions and seven Daedalus Clippy warnings.

The 1.0.0 release is not issued. See [the readiness audit](STABLE_1.0.0_AUDIT.md) for remaining gates. Historical 0.1.0 records remain in the repository.
