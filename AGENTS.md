# UrodLauncher

Standalone GPLv3 desktop adaptation of Modrinth App. Upstream provenance and
preserved license notices are in NOTICE.md, COPYING.md and docs/UPSTREAM-AUDIT.md.

## Architecture

- apps/app: Tauri v2 native shell, isolated local storage, skins and IPC.
- apps/app-frontend: Vue 3 desktop interface: Library, Content, Accounts, Skins and Settings.
- packages/app-lib: extracted upstream Microsoft/Minecraft authentication and Ely.by providers.
- packages/launcher-core: upstream-backed game/content installation, managed editions and transactions.
- packages/app-lib/src/upstream: inactive source comparison; never compile it.

Use tabs for indentation. Keep comments explanatory and avoid heading comments.
Keep changes focused on UrodLauncher; keep public content browsing separate from Hosting, advertising, social systems and telemetry.
The owner's Figma file `nxRBVO5kqVue97M5fgKDvZ` is the visual source of truth. Read the relevant nodes through Figma MCP before changing interface geometry or assets. Do not infer inaccessible screens from neighboring frames.

## Checks and launch

Run from the root:

```sh
pnpm install --frozen-lockfile
pnpm prepr:frontend:app
pnpm build
cargo fmt --package urod-launcher --package urod-app-lib --package urod-launcher-core --check
cargo check --locked -p urod-launcher -p urod-app-lib -p urod-launcher-core --all-targets
cargo clippy --locked -p urod-launcher -p urod-app-lib -p urod-launcher-core --all-targets --no-deps -- -D warnings
cargo test --locked -p urod-launcher -p urod-app-lib -p urod-launcher-core
pnpm app:dev
pnpm app:build --bundles appimage,deb
```

Use real rendered desktop verification for interface changes. Keep the development
bridge behind debug_assertions; release builds must not expose it. Never print,
commit or return credentials through IPC. Microsoft tokens belong in the native
credential store. Do not read or migrate the original application's user data.

Official editions remain unavailable until real UrodLauncher manifests are configured. Custom
instances support installation and launch through the native engine. Updater
endpoints and signing are disabled until real UrodLauncher signing keys exist. Preserve
all GPL notices when redistributing source or binaries. Check docs/STABLE_1.0.0_AUDIT.md
for observed results and external authentication/signing limits.

The application and package mark is the owner's Figma node `3:1175` in file
`nxRBVO5kqVue97M5fgKDvZ`. `apps/app/icons/figma-mark.svg` and
`apps/app-frontend/public/figma/2927a.svg` preserve the same export; package
PNG and ICO variants are rasterizations. The former geometric U logo and its
README SVG banners were removed from the current tree. README graphics use the
Figma mark and palette, bundled Geologica and real native screenshots. See
`docs/GRAPHICS_AUDIT.md` before adding graphical material.
