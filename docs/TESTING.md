# Verification record

Date: 2026-10-18. Branch: `release/1.0.0-figma-stable`. Results describe only checks observed on this Linux host. Native tests used isolated launcher data under `/tmp/urod-figma-rc`; live loader tests used an ignored local cache in `.qa-runtime`.

| Command or scenario | Outcome |
| --- | --- |
| `pnpm prepr:frontend:app` | Pass: Vue typecheck, ESLint, Prettier, 17 unit tests |
| `pnpm build` | Pass after the settings UI changes; Vite warns that the lazy skin viewer chunk exceeds 500 kB |
| `node scripts/check-stage2-boundary.mjs` | Pass after the latest settings UI changes |
| `node --test scripts/release-assets.test.mjs` | Pass |
| `pnpm audit --prod` | Pass after overriding transitive `source-map-js` to 1.2.2; no known production advisories reported |
| `cargo fmt --package urod-launcher --package urod-app-lib --package urod-launcher-core --check` | Pass |
| `cargo fmt --all -- --check` | Fail: large pre-existing formatting differences in vendored upstream crates; they were not reformatted wholesale |
| `cargo check --locked -p urod-launcher` | Pass |
| `cargo clippy --locked -p urod-launcher --all-targets -- -D warnings` | Pass after correcting seven Daedalus warnings |
| `cargo test --locked -p urod-launcher` | 30 pass, 2 ignored live Ely.by tests |
| `cargo test --locked --workspace` | Pass for enabled tests; the new Fabric/Quilt live integration test is ignored in the default run and passed when run explicitly |
| `node scripts/build.mjs --no-bundle` | Pass: optimized Linux binary built with the current 0.1.0 metadata |
| Release binary launch | Pass: `target/release/UrodLauncher` opened a native window with `XDG_DATA_HOME=/tmp/urod-release-smoke-data`; [capture](screenshots/figma-rc/release-binary-home-physical.png) shows the production WebView |
| Arch candidate package | `makepkg --force --nodeps --nocheck` passed for 0.1.0-1; package contains binary, desktop entry, Figma icon, LICENSE and NOTICE. `pacman -U` and `pacman -R` succeeded inside an isolated `/tmp/urod-package-root` with `-dd` because that root has no system dependency database. This does not verify launch after a normal system install. |
| Real Tauri at 1280×720 | Library, gallery, skins, dialogs, global settings and pack settings captured from the running app under [figma-rc](screenshots/figma-rc) |
| Skin save/edit/restart | Pass: known 64×64 PNG saved; Steve/Alex changed; state remained after WebView reload |
| Settings persistence | Pass: global RAM set to 6144 MB and survived WebView reload; pack RAM set to 6144 MB, name changed and a copy created through the Figma style native dialog |
| Vanilla 1.21.1 | Pass: official assets installed, Java 21 client started, Niri reported a `Minecraft 1.21.1` window, game stopped; second launch/stop passed in 22.11 seconds |
| Fabric 1.21.1 | Pass: loader 0.19.5 installed, game PID 7654 launched, status remained running, stop passed in 246.63 seconds |
| Quilt 1.21.1 | Pass: loader 0.31.0-beta.4 installed, game PID 10222 launched, status remained running, stop passed in 29.14 seconds; the provider marked no candidate stable, so the launcher chose the first available version |
| Forge 1.21.1 | Pass: version 52.1.14 installed, game PID 11221 launched, status remained running and stop passed in 50.99 seconds |
| NeoForge 1.21.1 | Pass: 21.1.250 installed, game PID 12562 launched, status remained running and stop passed in 47.10 seconds |
| 860×620 native resize | Pass for library, gallery and skins: no document horizontal overflow; settings at this size remain to be checked |
| 1240×820 at 125% monitor scale | DOM viewport had no document overflow; full visual inspection remains open because the host panel clipped the bottom of the native window |

The [Figma comparison map](FIGMA_IMPLEMENTATION_MAP.md#rendered-comparison) links exported references, native captures, and difference images. The global settings default frame has mean absolute RGB error 4.18/255; pack main frame has 3.37/255. These measurements include real data differences and do not certify pixel matching. Controls in later settings frames remain incomplete.

Unperformed checks are **not** passes: Windows installer execution, Debian package install/removal, Arch package launch after normal system installation, complete 1240×820 and 125% visual inspection, 150% HiDPI inspection, real Microsoft/Ely.by login, authenticated skin upload, and live game tests across additional Minecraft versions and hardware.
