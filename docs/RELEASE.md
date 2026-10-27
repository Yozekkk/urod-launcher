# UrodLauncher release process

The 1.0.0 Stable source and package versions are synchronized in Cargo, Tauri, pnpm and the Arch PKGBUILD. The release tag must point to the checked `main` commit. Release notes: [RELEASE_NOTES_1.0.0.md](RELEASE_NOTES_1.0.0.md). The [readiness audit](STABLE_1.0.0_AUDIT.md) and [test report](TEST-REPORT.md) distinguish verified paths from known limitations.

## Required checks

```bash
pnpm install --frozen-lockfile
pnpm prepr:frontend:app
pnpm build
pnpm audit --prod
python3 scripts/check-readme-assets.py
node --test scripts/release-assets.test.mjs
node scripts/check-stage2-boundary.mjs
cargo fmt --package urod-launcher --package urod-app-lib --package urod-launcher-core --check
cargo check --locked -p urod-launcher -p urod-app-lib -p urod-launcher-core --all-targets
cargo clippy --locked -p urod-launcher -p urod-app-lib -p urod-launcher-core --all-targets --no-deps -- -D warnings
cargo test --locked -p urod-launcher -p urod-app-lib -p urod-launcher-core
```

Run the native GUI with isolated data at the minimum and standard window sizes. `check.yml` builds NSIS, DEB and Arch packages. Its Windows job silently installs the NSIS package and checks the installed binary version; this is not a Windows GUI test. The Debian job installs the DEB and checks its desktop entry. The Arch job installs the PKG with pacman and checks its executable and desktop entry.

## Publication

1. Merge the reviewed release PR into `main` with a regular merge commit. Confirm the `main` CI matrix passes.
2. Tag that exact `main` commit `v1.0.0` and push the tag. `release.yml` reruns the package matrix, checks the three expected names, creates a GPL source archive and verified `SHA256SUMS.txt`, then creates a **draft** GitHub Release.
3. Download the draft assets, verify their names, sizes, package metadata and `sha256sum -c SHA256SUMS.txt`. Check README direct links against the draft asset names.
4. Publish the reviewed draft as a public, latest, non-prerelease release. Verify the public downloads and release metadata.

Never substitute an NCreate signing key or claim that these packages are signed. The Tauri updater remains disabled because UrodLauncher has no signing key and update endpoint. Windows Authenticode signing is separate future work. Live account access and platform GUI checks not performed for this release are listed in the notes, not reported as passed.
