# Developing UrodLauncher

The app keeps the NCreate Tauri 2 / Rust / Vue 3 workspace. `apps/app` is the native shell, `apps/app-frontend` is the Vue UI, `packages/app-lib` contains account/auth support, and `packages/launcher-core` contains Minecraft installation and launch code. Historical upstream audit documents are retained for attribution.

## Start

```bash
pnpm install --frozen-lockfile
pnpm app:dev
```

On Arch/EndeavourOS, install `base-devel rust nodejs pnpm webkit2gtk-4.1 openssl librsvg patchelf libayatana-appindicator xdg-utils`. On Debian/Ubuntu, see the desktop workflow for `libwebkit2gtk-4.1-dev`, `libayatana-appindicator3-dev`, OpenSSL and build tools.

The debug-only `UROD_TEST_DATA_DIR` environment variable selects an isolated absolute test data directory whose basename starts with `urod`. It must never be used for normal profiles.

## Checks

Run `pnpm prepr:frontend:app`, `pnpm build`, Rust fmt/check/clippy/test, and the native boundary audit from [RELEASE.md](RELEASE.md). Check visual changes in the running Tauri app at both 860 px and 1240 px. Current Figma comparisons and real Tauri screenshots are recorded in [FIGMA_IMPLEMENTATION_MAP.md](FIGMA_IMPLEMENTATION_MAP.md); historical 0.1.0 screenshots remain under `screenshots/urod-0.1.0`.

Application data lives in a separate `urod-launcher` directory. Native credentials use a separate UrodLauncher service. No automatic import from NCreate is performed. The community pack and signed auto-updates remain unavailable until UrodLauncher has real manifests and its own signing key.
