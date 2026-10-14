# Network and privacy boundaries

UrodLauncher inherits the NCreate separation between the Vue WebView and native Rust network code. Frontend capabilities do not grant broad filesystem, shell, HTTP or updater access; `scripts/check-stage2-boundary.mjs` checks these grants and the CSP.

Rust uses Microsoft, Xbox/XSTS and Minecraft Services for licensed sign-in and Minecraft profile operations. The skin library uploads PNG files to `api.minecraftservices.com/minecraft/profile/skins` using a Minecraft access token obtained from the native account engine. The token is not returned to Vue or written to the skin library index. Ely.by authentication and public skin URLs use their provider endpoints. Modrinth public catalog and artifact downloads use the existing engine; verified file hashes and path checks remain in that engine.

The community card checks TCP connectivity to `gorodurodov.ru:25565`; it does not claim a Minecraft protocol status or discover an SRV record. Its news link opens the official channel through the browser. There is no social backend, telemetry, advertising or Millida service connection. No NCreate Server manifest, NCreate updater endpoint or NCreate signing key is configured in the product.

The original Modrinth extraction and network exclusions are documented in the retained [upstream audit](UPSTREAM-AUDIT.md). That document is provenance, not a claim that all live flows have been re-tested for UrodLauncher.
