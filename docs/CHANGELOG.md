# Changelog

## Unreleased work toward 1.0.0

- Imported original UrodLauncher Figma SVG/PNG assets and the Geologica/Inter fonts with OFL notices.
- Replaced the main shell with Figma sidebar, header and news rail; reworked library and live Modrinth gallery geometry.
- Reworked the skin page and add/edit dialogs using the inspected Figma frames. PNG upload, native validation, model choice and persistence remain functional.
- Transferred the pack content manager and selected-pack Modrinth browser from Figma frames `3:713` and `3:888`, retaining live installed content and real catalog data. Added shader and resource pack search, compatibility checks and installation through the existing backend.
- Added Figma-based global and pack settings windows. Global RAM, per-pack RAM, renaming and duplication were exercised in the native app. Unsupported sync, repair, window and command controls are explicitly unavailable.
- Matched the resource settings layout from Figma `3:199` against a native Tauri capture. The cache and extended report controls are disabled until backend operations exist.
- Verified live install, start and stop for Minecraft 1.21.1 with Vanilla, Fabric, Quilt, Forge and NeoForge; repeated the Vanilla launch.
- Patched the transitive `source-map-js` dependency to 1.2.2 after a production dependency audit found GHSA-68fv-2mgg-jv7q.
- Replaced bundled desktop icons with rasterizations of the original Figma launcher mark.
- Corrected active `AGENTS.md` project instructions and seven Daedalus Clippy warnings.

The 1.0.0 release is not issued. See [the readiness audit](STABLE_1.0.0_AUDIT.md) for remaining gates. Historical 0.1.0 records remain in the repository.
