# Source and licensing

UrodLauncher 0.1.0 is a separate GPL-3.0-only derivative of NCreate Launcher.
It keeps the existing Rust/Tauri/Vue architecture and adapted NCreate code. The
skin manager was informed by the open-source Millida Launcher; no Millida
service, account infrastructure, keys or branded artwork are included.
The work toward the next UrodLauncher interface uses assets exported from the
owner's Figma file `nxRBVO5kqVue97M5fgKDvZ`, including the launcher mark at
node `3:1175`. See apps/app-frontend/public/brand/PROVENANCE.md and
docs/FIGMA_IMPLEMENTATION_MAP.md. The earlier UrodLauncher logo and city
illustration are historical assets. The old NCreate branding described below
is provenance only and is not shipped in this UI.

NCreate Launcher is a modified version of the GPL-3.0-only desktop application
Modrinth App, copyright Rinth, Inc. and upstream contributors.
Source: https://github.com/modrinth/code at commit 0e817be41cebdb0e140866e87e727c2101fe0e6b (main, initial Stage 2 source audit); latest upstream main reviewed at 23ab5cdc331e40712f12b9026979359aec1e404d (2026-09-28).
Modified by NCreate contributors in September 2026. Distributed without warranty.
Original license and COPYING notices are preserved. Historical NCreate branding
was owned by NCreate and copied from Yozekkk/ncreate-site commit 85c9eb0992411e8f40d7a6d12ac5695aec66b0c6.
See LICENSE for redistribution terms, docs/UPSTREAM-AUDIT.md for the original extraction boundary and docs/STAGE2-UPSTREAM-AUDIT.md for Stage 2 reuse.

The Vue visual layer was recreated using OneLauncher as a visual reference
(Polyfrost/OneLauncher, commit 7a6a7cbc0bbc9cf7a27e74fb700b4cbec4c995b9),
with the permission stated by the NCreate project owner. No OneLauncher runtime,
backend or product services are included. Generic Poppins font files were
converted to WOFF2 from that reference's bundled fonts; their SIL Open Font License
is retained in apps/app-frontend/public/fonts/Poppins-OFL.txt.
The new interface embeds Geologica and Inter variable fonts from the Google
Fonts repositories. Their SIL Open Font License notices are retained at
apps/app-frontend/public/fonts/OFL-Geologica.txt and OFL-Inter.txt.
The historical seasonal artwork has been removed from UrodLauncher.
