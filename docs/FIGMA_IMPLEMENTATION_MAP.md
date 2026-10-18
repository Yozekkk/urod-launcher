# Figma → UrodLauncher

Source: [owner's Figma file](https://www.figma.com/design/nxRBVO5kqVue97M5fgKDvZ/Untitled?node-id=0-1), page `0:1`. Inspected on 2026-10-17 with the official Figma MCP server.

The page metadata lists 23 top-level frames: 16 settings windows at 837×470, five 1280×720 application screens, and two 500×489 skin dialogs. Six top-level rounded rectangles named `image 280`–`image 285` are separate canvas references, not complete application screens. Frame geometry below comes from `get_metadata`; exact styles are recorded only where `get_design_context` succeeded.

| Figma node | Screen | Vue owner | CSS and assets | Behavior | State |
| --- | --- | --- | --- | --- | --- |
| `3:1152` | Library, 1280×720 | `AppNavbar.vue`, `LibraryPage.vue` | `figma.css`, `public/figma/*` | Real local instances, search, sort, create, import, select, install, play/stop | Shell and card geometry transferred; Tauri screenshot compared; empty real library differs from Figma's example cards |
| `3:1094` | Gallery, 1280×720 | `AppNavbar.vue`, `ContentPage.vue` | `figma.css`, exported SVGs | Live Modrinth catalog, target instance, filter, search, install | Main geometry transferred; Tauri screenshot captured; actual projects differ from placeholder cards |
| `3:713` | Pack content, 1280×720 | `LibraryPage.vue`, `ContentPage.vue` | Exported SVG/PNG in `public/figma` | Existing installed-content engine | Detailed node retrieved; transfer pending |
| `3:888` | Additional content screen, 1280×720 | `ContentPage.vue` | Exported SVG/PNG | Existing catalog detail | Detailed node retrieved; transfer pending |
| `3:1012` | Skins, 1280×720 | `SkinsPage.vue`, `SkinPreview.vue` | `figma.css`, exported add icon | Real local PNG library and draggable 3D viewer | Grid and shell transferred; card thumbnails still show head crops instead of full figure |
| `3:1259` | Edit skin, 500×489 | `SkinsPage.vue` | Exported dialog SVG/PNG | Change Steve/Alex; rename in main view; replace texture visibly disabled | Geometry transferred and Tauri screenshot compared; user texture differs from Figma sample |
| `3:1297` | Add skin, 500×489 | `SkinsPage.vue` | Exported dialog SVG/PNG | Select and validate PNG, 3D preview, save | Geometry transferred and Tauri screenshot compared; user texture differs from Figma sample |
| `3:104` | Java locations, 837×470 | `FigmaGlobalSettings.vue`, `JavaRuntimeDialog.vue` | Exported SVG | Existing Java path management | Detailed node retrieved earlier; current panel needs exact comparison |
| `3:199` | Additional settings, 837×470 | `FigmaGlobalSettings.vue` | Exported SVG | Existing settings | Detailed node retrieved earlier; exact panel pending |
| `3:11` | Pack main, 837×470 | `LibraryPage.vue` | `figma.css`, exported pack SVGs | Rename, duplicate, delete | Native Tauri frame compared, 3.37/255 mean RGB error; sample name and deletion semantics differ |
| `3:422` | Pack installation, 837×470 | `LibraryPage.vue` | `figma.css`, exported pack SVGs | Installation details; unsafe migration and repair clearly disabled | Native capture compared; behavior incomplete |
| `3:648`, `3:463`, `3:530`, `3:588`, `3:629` | Pack override subframes | `LibraryPage.vue` | Exported references in `docs/figma-reference` | Per-pack Java and RAM persist; window/JVM/commands unavailable | Detailed nodes retrieved; combined panel remains visually different |
| `3:47` | Global sync, 837×470 | `FigmaGlobalSettings.vue` | `figma.css`, exported switch and nav SVGs | Sync controls visibly disabled until engine support exists | Native Tauri comparison, 4.18/255 mean RGB error |
| `3:162`, `3:236`, `3:280`, `3:339`, `3:367` | Global resource, memory, arguments and commands subframes | `FigmaGlobalSettings.vue` | Exported references in `docs/figma-reference` | Global Java and RAM persist; resource controls and commands unavailable | Detailed nodes retrieved; some panels remain visually different |
| `3:402` | Additional global commands frame | Pending | Awaiting MCP context | Existing settings | Metadata only; Starter-plan limit recurred |

The page metadata confirms 1280×720 application frames, 837×470 settings frames, and 500×489 skin dialogs. Detailed frames use Geologica; settings `3:104` and `3:199` also use Inter. The library frame specifies a `#0c0c0c` root, `#131313` work area, `#202020` controls/cards, `#333333` borders/placeholders, `#c12926` accent, `#ffffff` primary text, and `#848488` muted text. Its workspace begins at (115, 115), has width 831, and its news area begins at x=946. The library content begins at (153, 147); cards are approximately 134.8×175.7 with 19 px horizontal spacing.

Original vector and raster assets returned by successful `get_design_context` calls were downloaded to `apps/app-frontend/public/figma`. Temporary Figma asset URLs are absent from runtime code. Font files and OFL notices are in `apps/app-frontend/public/fonts`. The library reference image is stored at `docs/figma-reference/library-3-1152.png`; it is for comparison only and is not loaded by the application.

### Assets used by the running interface

| Figma source | Vue and CSS owner | Exported asset | Use |
| --- | --- | --- | --- |
| Shared shell in `3:1152`; app mark `3:1175` | `AppNavbar.vue`, `figma.css` | `2927a.svg`; rasterized copies in `apps/app/icons` | Header and executable/installer/window icons |
| Shared sidebar in `3:1152` | `AppNavbar.vue`, `figma.css` | `f1283.svg`, `332c7.svg`, `03486.svg`, `df15d.svg`, `8aecb.png`, `fd48a.svg` | Home, library, account, skins, content and settings navigation |
| Shared header in `3:1152` | `AppNavbar.vue`, `figma.css` | `2aa90.png`, `319d1.svg` | Empty account avatar and history control |
| Skin add tile in `3:1012` | `SkinsPage.vue` | `55119.svg` | Add icon |
| Global settings `3:47` | `FigmaGlobalSettings.vue` | `38640.svg`, `6a8c5.svg`, `bd47c.svg`, `5e6f8.svg`, `20469.svg` | Navigation, original edit/switch group and close icon |
| Pack settings `3:11` | `LibraryPage.vue` | `8aecb.png`, `4b87e.svg`, `2de22.svg`, `f2524.svg`, `165b7.svg`, `94db2.svg` | Pack mark, section icons, copy and delete controls |
| Typography in inspected frames | `figma.css` | `geologica-variable.ttf`, `inter-variable.ttf` | Local fonts with OFL notices |

The files above live under `apps/app-frontend/public/figma` unless another directory is stated. Remaining downloaded exports are kept for frames and settings still to be implemented; their use must be checked against the corresponding node context before wiring them into UI.

## Rendered comparison

The [library app capture](screenshots/figma-rc/library-1280x720.png) was taken from the running Tauri window at 1280×720. Compare it with the [Figma export](figma-reference/library-3-1152.png), [50% overlay](figma-reference/library-overlay.png), and [contrast-enhanced difference](figma-reference/library-diff.png). Mean absolute RGB error is 1.69/255 for the top 115 px, 2.20/255 for the left rail, and 1.73/255 for the news rail. The workspace error is 12.52/255 because the Figma frame contains ten illustrative cards while the isolated local data directory contains no installed packs. These numbers are measurements, not a claim of finished pixel matching.

Additional real Tauri captures: [gallery](screenshots/figma-rc/gallery-1280x720.png), [skins](screenshots/figma-rc/skins-1280x720.png), [add skin](screenshots/figma-rc/add-skin-1280x720.png), and [edit skin](screenshots/figma-rc/edit-skin-1280x720.png). The skin images use a known sample PNG in an isolated test data directory. They are not screenshots from Figma. All captures are work in progress; control states, smaller windows, and settings still need comparison.

The global settings [Figma frame](figma-reference/settings-3-47.png) and [running Tauri crop](screenshots/figma-rc/settings-global-837x470.png) have a mean absolute RGB error of 4.18/255; the [difference image](figma-reference/settings-global-diff.png) shows the remaining font and antialiasing differences. The pack main [Figma frame](figma-reference/settings-3-11.png) and [running Tauri crop](screenshots/figma-rc/settings-pack-837x470.png) have 3.37/255 mean absolute RGB error. The live test pack has a different name from the Figma example, and the deletion copy reflects the existing recoverable trash behavior. [Pack installation](screenshots/figma-rc/settings-pack-install-837x470.png), [pack overrides](screenshots/figma-rc/settings-pack-overrides-837x470.png), and [resource settings](screenshots/figma-rc/settings-resources-837x470.png) are native captures with unresolved differences. These screens are not ready for a Stable visual pass.

The Figma MCP limit reset on 2026-10-18, allowing thirteen more settings contexts and screenshots to be saved in `docs/figma-reference`. It then recurred on `3:402`. The earlier successful `3:104` and `3:199` contexts were inspected but their raw responses were not saved before the first limit. Exact implementation of the remaining subframes and visual verification remain open; metadata alone cannot supply their layer styles.
