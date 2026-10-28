#!/usr/bin/env python3
"""Compose README graphics from the Figma mark, Geologica and a real Tauri capture.

Requires Pillow and librsvg's rsvg-convert. No remote assets or image generation.
"""

from io import BytesIO
from pathlib import Path
import subprocess

from PIL import Image, ImageDraw, ImageFont, ImageOps


ROOT = Path(__file__).resolve().parent.parent
OUT = ROOT / ".github/readme"
FONT = ROOT / "apps/app-frontend/public/fonts/geologica-variable.ttf"
MARK = ROOT / "apps/app-frontend/public/figma/2927a.svg"
CAPTURE = ROOT / "docs/screenshots/readme/library.png"
SCALE = 2
BACKGROUND = "#0c0c0c"
SURFACE = "#202020"
BORDER = "#333333"
ACCENT = "#c12926"
TEXT = "#ffffff"
SOFT = "#e7e7e7"
MUTED = "#848488"


def font(size: int, weight: int = 400) -> ImageFont.FreeTypeFont:
	result = ImageFont.truetype(str(FONT), size * SCALE)
	result.set_variation_by_axes([weight, 0, 0, 0])
	return result


def rect(draw: ImageDraw.ImageDraw, bounds: tuple[int, int, int, int], fill: str, radius: int, outline: str | None = None) -> None:
	draw.rounded_rectangle(tuple(value * SCALE for value in bounds), radius * SCALE, fill=fill, outline=outline, width=SCALE)


def label(draw: ImageDraw.ImageDraw, text: str, x: int, y: int, size: int, weight: int, fill: str) -> None:
	draw.text((x * SCALE, y * SCALE), text, font=font(size, weight), fill=fill, anchor="lt")


def mark(size: int) -> Image.Image:
	result = subprocess.run(
		["rsvg-convert", "-w", str(size * SCALE), "-h", str(size * SCALE), str(MARK)],
		check=True,
		capture_output=True,
	)
	return Image.open(BytesIO(result.stdout)).convert("RGBA")


def banner(name: str, subtitle: str, caption: str) -> None:
	image = Image.new("RGB", (1200 * SCALE, 320 * SCALE), BACKGROUND)
	draw = ImageDraw.Draw(image)
	rect(draw, (1, 1, 1199, 319), BACKGROUND, 27, BORDER)
	image.paste(mark(48), (60 * SCALE, 55 * SCALE), mark(48))
	label(draw, "UrodLauncher", 60, 116, 59, 600, TEXT)
	label(draw, subtitle, 62, 205, 21, 400, SOFT)
	rect(draw, (61, 273, 115, 277), ACCENT, 2)
	label(draw, caption, 130, 262, 14, 400, MUTED)
	with Image.open(CAPTURE) as source:
		preview = ImageOps.fit(source.convert("RGB"), (530 * SCALE, 264 * SCALE), Image.Resampling.LANCZOS)
	mask = Image.new("L", preview.size)
	ImageDraw.Draw(mask).rounded_rectangle((0, 0, preview.width - 1, preview.height - 1), radius=16 * SCALE, fill=255)
	image.paste(preview, (635 * SCALE, 28 * SCALE), mask)
	draw.rounded_rectangle((635 * SCALE, 28 * SCALE, 1165 * SCALE, 292 * SCALE), radius=16 * SCALE, outline=BORDER, width=SCALE)
	image.save(OUT / name, optimize=True)


def button(name: str, title: str, detail: str) -> None:
	image = Image.new("RGB", (360 * SCALE, 100 * SCALE), BACKGROUND)
	draw = ImageDraw.Draw(image)
	rect(draw, (1, 1, 359, 99), SURFACE, 12, BORDER)
	image.paste(mark(38), (22 * SCALE, 31 * SCALE), mark(38))
	label(draw, title, 75, 22, 23, 600, TEXT)
	label(draw, detail, 76, 59, 14, 400, MUTED)
	image.save(OUT / name, optimize=True)


def main() -> None:
	OUT.mkdir(parents=True, exist_ok=True)
	banner("banner.png", "Minecraft-лаунчер Города Уродов", "Библиотека  ·  Каталог  ·  Скины")
	banner("banner-en.png", "Minecraft launcher for Gorod Urodov", "Library  ·  Catalog  ·  Skins")
	button("download-windows.png", "Windows", "EXE  ·  x64  ·  1.0.0")
	button("download-debian.png", "Debian / Ubuntu", "Linux Mint  ·  DEB  ·  1.0.0")
	button("download-arch.png", "Arch / EndeavourOS", "Manjaro  ·  PKG  ·  1.0.0")


if __name__ == "__main__":
	main()
