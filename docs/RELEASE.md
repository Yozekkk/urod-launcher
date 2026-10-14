# UrodLauncher release process

Version 0.1.0 is a development version. Do not label it Stable or publish a release until Windows, Debian and Arch jobs pass and their installers are inspected on each platform.

## Local checks

```bash
pnpm install --frozen-lockfile
pnpm prepr:frontend:app
pnpm build
node --test scripts/release-assets.test.mjs
node scripts/check-stage2-boundary.mjs
cargo fmt --package urod-launcher --package urod-app-lib --package urod-launcher-core --check
cargo check --locked -p urod-launcher -p urod-app-lib -p urod-launcher-core --all-targets
cargo clippy --locked -p urod-launcher -p urod-app-lib -p urod-launcher-core --all-targets --no-deps -- -D warnings
cargo test --locked -p urod-launcher -p urod-app-lib -p urod-launcher-core
```

The native GUI must be checked at both the 860 px minimum width and a larger window. Real Microsoft skin upload, Ely.by login, Minecraft launch, installer install/uninstall and update recovery need live environment checks; deterministic tests do not replace them.

## Packages

- Windows x64: `pnpm app:build --bundles nsis --config tauri.ci.conf.json` → `UrodLauncher-Setup-<version>.exe`.
- Debian/Ubuntu x64: `pnpm app:build --bundles deb --config tauri.ci.conf.json` → `UrodLauncher-<version>-amd64.deb`.
- Arch x64: `pnpm app:build --no-bundle --config tauri.ci.conf.json`, then `makepkg -f` in `packaging/arch` → `.pkg.tar.zst`.

The `check.yml` workflow runs checks and builds all three packages, including Arch in an Arch container. A version tag invokes `release.yml`, which reuses those checks and creates an **unpublished draft** with source and `SHA256SUMS.txt`. A reviewer must verify the packages before manually publishing the draft.

The inherited updater remains disabled: UrodLauncher has no own updater public/private key pair or update endpoint yet. Neither an NCreate key nor unsigned update metadata may be substituted. Windows Authenticode signing is a separate future task. Published release files must never be described as signed updates until this is configured and tested.
