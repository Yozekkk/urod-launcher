# UrodLauncher Figma design system

Source: [Figma page `0:1`](https://www.figma.com/design/nxRBVO5kqVue97M5fgKDvZ/Untitled?node-id=0-1). Values here are taken from detailed MCP output for the library and other accessible frames. `figma.css` holds the implemented CSS tokens.

| Token | Value | Source |
| --- | --- | --- |
| Background | `#0c0c0c` | `3:1152` root |
| Panel | `#131313` | `3:1156` |
| Control and card | `#202020` | `3:1190`, `3:1197` |
| Border and image placeholder | `#333333` | `3:1170`, library cards |
| Accent | `#c12926` | `3:1188` |
| Main text | `#ffffff` | `3:1180` |
| Secondary text | `#e7e7e7` | `3:1183` |
| Muted text | `#848488` | `3:1191` |
| Pack icon placeholder | `#d9d9d9` | `3:15` |

The main screens use Geologica at light, regular, and medium weights. The retrieved settings frames also use Inter. The variable TTFs come from the Google Fonts repositories and include SIL Open Font License notices; see the font files and `OFL-*.txt` in `apps/app-frontend/public/fonts`. The product still contains inherited font assets used by untouched pages. Do not delete those notices during the migration.

The 1280×720 reference shell uses a 115 px header and left rail. The application pane starts at x=115, y=115 and the news rail at x=946. Library content starts 38 px inside the pane. A library card is 134.8×175.7 px with a 116.3 px image slot and 12 px corner radius. Where a user instance has no icon, the slot remains a neutral surface; the Figma repeated example image is not substituted for real user content.

The original SVG and PNG exports are in `apps/app-frontend/public/figma`. Use them at their intrinsic aspect ratios. The Figma library screenshot in `docs/figma-reference` is comparison evidence only, never an application background.

The settings frames are 837×470 with a 27 px radius, `#202020` surface, a header divider at y=92, and a vertical divider at x=251 from y=128 to y=433. Global settings content starts at (284, 123). Pack settings use a 29×29 mark at (32, 31), a title at (185, 35), and the same content x coordinate. The source `3:47` switch group is the original 80×27 SVG, including its edit icon. The application deliberately disables this group until real cross-pack synchronization exists.
