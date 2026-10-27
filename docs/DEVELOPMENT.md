# Developing UrodLauncher

The app keeps the NCreate Tauri 2 / Rust / Vue 3 workspace. `apps/app` is the native shell, `apps/app-frontend` is the Vue UI, `packages/app-lib` contains account/auth support, and `packages/launcher-core` contains Minecraft installation and launch code. Historical upstream audit documents are retained for attribution.

## Start

```bash
pnpm install --frozen-lockfile
pnpm app:dev
```

On Arch/EndeavourOS, install `base-devel rust nodejs pnpm webkit2gtk-4.1 openssl librsvg patchelf libayatana-appindicator xdg-utils`. On Debian/Ubuntu, see the desktop workflow for `libwebkit2gtk-4.1-dev`, `libayatana-appindicator3-dev`, OpenSSL and build tools.

The debug-only `UROD_TEST_DATA_DIR` environment variable selects an isolated absolute test data directory whose basename starts with `urod`. It must never be used for normal profiles.

For the optional native persistence E2E check, build a debug binary once with `pnpm app:dev`, close that session, then start `pnpm --dir apps/app-frontend dev` and run `python3 scripts/library-first-e2e.py --launches 20` in another terminal. It requires `tauri-agent-tools` and a graphical desktop. The script creates and removes its own temporary data directory; it does not use normal launcher profiles. Its run and limitations are recorded in [TEST-REPORT.md](TEST-REPORT.md).

Settings retain the legacy `java_path` field for existing data and add `java_paths`, a map from required Java major to a validated executable path. Old settings deserialize with an empty map. During launch, an explicit pack Java path wins; otherwise `core_launch` passes the matching global path to the engine without writing it into pack metadata. When neither path exists, the engine discovers or downloads a compatible runtime. A changed major must be rejected by `save_settings` before persistence.

`download_parallelism` defaults to 6 for older settings JSON and is validated in the range 1–10. `AppState::engine` reads it when opening the engine and constructs the shared download semaphore at that size. Saving a new value therefore requires an application restart before it affects downloads. The UI keeps the independent write limit disabled because no corresponding semaphore exists. The backup-folder command opens only the canonical `backups` child of the launcher data directory; it creates the directory if needed but does not generate a backup.

## Checks

Run `pnpm prepr:frontend:app`, `pnpm build`, Rust fmt/check/clippy/test, and the native boundary audit from [RELEASE.md](RELEASE.md). Check visual changes in the running Tauri app at both 860 px and 1240 px. Current Figma comparisons and real Tauri screenshots are recorded in [FIGMA_IMPLEMENTATION_MAP.md](FIGMA_IMPLEMENTATION_MAP.md). Historical 0.1.0 UI captures with the retired U logo were removed from the current tree; their Git history was preserved. The historical Minecraft launch capture remains under `screenshots/urod-0.1.0`.

Application data lives in a separate `urod-launcher` directory. Native credentials use a separate UrodLauncher service. No automatic import from NCreate is performed. The community pack and signed auto-updates remain unavailable until UrodLauncher has real manifests and its own signing key.
