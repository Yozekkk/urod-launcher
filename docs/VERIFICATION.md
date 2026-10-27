# UrodLauncher 0.1.0 verification record

Checked on 2026-10-13 and 2026-10-14 in an EndeavourOS desktop session. This is a development build, not a Stable release.

## Automated checks

- `pnpm install --frozen-lockfile`, Vue/TypeScript, ESLint, Prettier and 17 frontend tests passed.
- Rust fmt for the three UrodLauncher packages, check, Clippy with warnings denied and workspace package tests passed. Some inherited live-network and credential-vault tests are intentionally ignored by default. Formatting the vendored upstream workspaces with the root toolchain produces unrelated differences and is not part of this check.
- The native boundary audit and release package naming test passed.
- [GitHub Actions run 38515455380](https://github.com/Yozekkk/urod-launcher/actions/runs/38515455380) completed successfully in the recreated repository. The Windows NSIS installer was built and uploaded; the Debian package was installed and inspected on Ubuntu; the Arch package was installed and inspected in an Arch container. The three downloaded CI artifacts have SHA-256 hashes:

  | Package | SHA-256 |
  | --- | --- |
  | `UrodLauncher-Setup-0.1.0.exe` | `79fabb1c38b25f7e97332a1e9d114f88e167d71b07f127f11cac049783379ed3` |
  | `UrodLauncher-0.1.0-amd64.deb` | `bbde5b2b6aeab6583d3be84e9443f1447e5f419813b80a12d6d20d52de23861e` |
  | `UrodLauncher-0.1.0-x86_64.pkg.tar.zst` | `ed017bc2d3362bcb4857485ef821e1d5f70ce1b5e6b2ce60a3d9faa3f27926b0` |
- The Microsoft OAuth/Xbox/Minecraft and Ely.by deterministic auth suites exercise the inherited real pipeline against controlled network responses. They do not constitute live sign-in.
- The new skin-library test saves two skins, switches the active Offline skin, reloads persisted data, resets and deletes it.

## Real desktop checks

- UrodLauncher opened as a native Tauri window at 860×806 and 1200×806 logical pixels. Home, Library, Catalog, Accounts, Skins and Settings were captured from the running 0.1.0 application at the time. Those historical UI images contained the retired U logo and were removed from the current tree during the graphics cleanup; they remain in Git history. Current native captures are in [screenshots/readme](screenshots/readme).
- At the time, the Home, Library, Catalog, Accounts and Settings captures were compared with NCreate 1.0.0 screenshots. The generated city artwork had already been removed. These historical observations do not describe the current Figma layout or Library-first navigation.
- An isolated Offline account and three library entries persisted after a native app restart. A skin was applied locally and shown as active in the skin UI. The 3D character preview rendered in the WebView.
- Light and dark presets were inspected visually. The light background overlay issue found during inspection was fixed.
- A Vanilla 1.21.1 instance was created, installed with verified game files and launched using system Java 21 and the test Offline account. The game reached its [first-run screen](screenshots/urod-0.1.0/minecraft-launch.png). `core_stop` ended the Java process and the instance returned to `ready`.
- A release binary and Arch `.pkg.tar.zst` were built on EndeavourOS. `pacman -Qpi` and archive listing confirmed package metadata, dependencies, binary, desktop entry, icon, LICENSE and NOTICE. Host installation with `pacman -U` was not performed because administrator authentication was unavailable; the Arch CI job is configured to install the package in its container.
- The optimized release binary also opened successfully with an isolated clean data directory and showed the community home screen before exiting normally.

## Not yet verified

- Live Microsoft login, ownership check, three distinct Microsoft skin uploads, and reset with a real licensed account.
- Live Ely.by login and skin behavior on an Ely.by server.
- Physical Windows NSIS installation/uninstallation and Debian installation/uninstallation outside the CI runner. The Windows job built the installer but did not install it.
- A signed UrodLauncher updater: there is no UrodLauncher signing key or endpoint, so auto-updates are disabled.
- A dedicated community pack: no real manifest has been provided.
- Complete English translation. The language control changes the document language, main navigation and settings navigation, but most content remains Russian.

No release should be published as Stable until the outstanding checks relevant to that release are completed.
