# UrodLauncher 0.1.0 verification record

Checked on 2026-10-13 and 2026-10-14 in an EndeavourOS desktop session. This is a development build, not a Stable release.

## Automated checks

- `pnpm install --frozen-lockfile`, Vue/TypeScript, ESLint, Prettier and 17 frontend tests passed.
- Rust fmt for the three UrodLauncher packages, check, Clippy with warnings denied and workspace package tests passed. Some inherited live-network and credential-vault tests are intentionally ignored by default. Formatting the vendored upstream workspaces with the root toolchain produces unrelated differences and is not part of this check.
- The native boundary audit and release package naming test passed.
- Before the GitHub repository was recreated, CI built a Windows NSIS installer, installed and inspected the Debian package on Ubuntu, and installed and inspected the Arch package in an Arch container. Local backup copies of those three CI artifacts have SHA-256 hashes below. A fresh run in the recreated repository must be checked separately:

  | Package | SHA-256 |
  | --- | --- |
  | `UrodLauncher-Setup-0.1.0.exe` | `cfab6ab101d2a235b57a6804411ffe29fcd2e8cdb126f1a829a397d6a207c38f` |
  | `UrodLauncher-0.1.0-amd64.deb` | `8f05ce20553de8bb57fa4f4b5c37e8a42a43ec89640c0af3bf2683627816c197` |
  | `UrodLauncher-0.1.0-x86_64.pkg.tar.zst` | `ba985209e8cf17dd9da588d60c8185d65be156c9d1d58da7efd10a007a417ac3` |
- The Microsoft OAuth/Xbox/Minecraft and Ely.by deterministic auth suites exercise the inherited real pipeline against controlled network responses. They do not constitute live sign-in.
- The new skin-library test saves two skins, switches the active Offline skin, reloads persisted data, resets and deletes it.

## Real desktop checks

- UrodLauncher opened as a native Tauri window at 860×806 and 1200×806 logical pixels. Screenshots of [Home](screenshots/urod-0.1.0/home.png), [Library](screenshots/urod-0.1.0/library.png), [Catalog](screenshots/urod-0.1.0/content.png), [Accounts](screenshots/urod-0.1.0/accounts.png), [Skins](screenshots/urod-0.1.0/skins.png) and [Settings](screenshots/urod-0.1.0/settings.png) were captured from the running application; small-window examples are in the same directory.
- The Home, Library, Catalog, Accounts and Settings captures were compared with the NCreate 1.0.0 screenshots in the source repository. Navigation placement and core card structure were retained while the colors and vector icon changed. The generated city artwork was removed; the Home captures were taken again with a CSS background and the original vector mark.
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
