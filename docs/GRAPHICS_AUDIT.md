# Graphics provenance and README assets

Updated 2026-10-27. The source of truth is the owner's [Figma file](https://www.figma.com/design/nxRBVO5kqVue97M5fgKDvZ/Untitled?node-id=0-1). Figma MCP reconfirmed the application mark at node <code>3:1175</code> and the Library frame <code>3:1152</code> during this cleanup.

| Current material | Provenance | Decision |
| --- | --- | --- |
| <code>apps/app-frontend/public/figma/*</code> | SVG and PNG exports from the owner's Figma file, mapped in [FIGMA_IMPLEMENTATION_MAP.md](FIGMA_IMPLEMENTATION_MAP.md) | Retained as original design assets. |
| <code>apps/app/icons/*</code> | PNG/ICO rasterizations of Figma mark <code>3:1175</code>; <code>figma-mark.svg</code> is byte-identical to <code>public/figma/2927a.svg</code> | Retained for the window, executable and installer. |
| <code>apps/app/assets/steve.png</code> | Existing Minecraft skin fixture used in native tests | Retained as a technical test resource. Its original author is not proven, so it is not presented as AI-generated or used as README artwork. |
| <code>docs/figma-reference/*</code> | Figma screenshots plus documented comparison overlays/differences | Retained as engineering evidence, not presented as application screenshots. |
| <code>docs/screenshots/figma-rc/*</code> | Real Tauri captures from the earlier Figma implementation checks | Retained as test evidence; some show earlier navigation or synthetic QA data. |
| <code>docs/screenshots/readme/*</code> | Fresh 1280×720 captures of the running Tauri app with an isolated test profile | Used in README. The catalog screen includes actual live Modrinth project icons supplied by the catalog API; no image was independently fetched for README decoration. |
| <code>.github/readme/*.png</code> | Local composition by [build-readme-graphics.py](../scripts/build-readme-graphics.py) | Uses the Figma mark, exact palette from <code>figma.css</code>, bundled Geologica and the real Library capture. No AI generation or stock images. |
| <code>docs/screenshots/urod-0.1.0/minecraft-launch.png</code> | Historical real Minecraft first-run capture | Retained for the historical verification record; it contains no retired launcher logo. |

The former hand-authored geometric U at <code>apps/app-frontend/public/brand/logo.svg</code> was removed from the application. Five old README SVGs used that invented mark, Arial/DejaVu and a non-Figma palette; they were replaced by the PNGs above. Ten historical 0.1.0 launcher screenshots also contained the retired mark and were removed from the current tree. Their Git history remains intact, as requested. The Minecraft launch capture remains. The earlier generated city artwork had already been removed before this audit.

The README graphics use **Geologica only**. The variable TTF is bundled with the app and covered by [OFL-Geologica.txt](../apps/app-frontend/public/fonts/OFL-Geologica.txt). The current Figma interface uses Geologica for main screens; some settings headings use Inter in the source design. The README generator uses no Inter, Manrope, Poppins, system font or downloaded font.

The README download graphics link directly to the three named v1.0.0 release files. The graphic labels show version 1.0.0; the files are not claimed to carry Authenticode or Tauri updater signatures. The release workflow generates `SHA256SUMS.txt` from the actual installer and source files.
