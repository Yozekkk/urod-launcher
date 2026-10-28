#!/usr/bin/env python3
"""Check README-local links, image headers and retired brand references offline."""

from html.parser import HTMLParser
from pathlib import Path
import re
import struct


ROOT = Path(__file__).resolve().parent.parent
README_FILES = [ROOT / "README.md", ROOT / "README.en.md"]
IMAGE_PATTERN = re.compile(r"!\[[^]]*\]\(([^)]+)\)")
LINK_PATTERN = re.compile(r"(?<!!)\[[^]]*\]\(([^)]+)\)")


class Links(HTMLParser):
	def __init__(self) -> None:
		super().__init__()
		self.images: list[str] = []
		self.links: list[str] = []

	def handle_starttag(self, tag: str, attrs: list[tuple[str, str | None]]) -> None:
		attributes = dict(attrs)
		if tag == "img" and attributes.get("src"):
			self.images.append(attributes["src"])
		if tag == "a" and attributes.get("href"):
			self.links.append(attributes["href"])


def check_image(path: Path) -> None:
	data = path.read_bytes()
	if data[:8] != b"\x89PNG\r\n\x1a\n":
		raise AssertionError(f"Not a PNG: {path}")
	width, height = struct.unpack(">II", data[16:24])
	if width < 32 or height < 32:
		raise AssertionError(f"Suspicious image dimensions: {path}: {width}x{height}")
	if data[-12:-8] != b"\x00\x00\x00\x00" or data[-8:-4] != b"IEND":
		raise AssertionError(f"Incomplete PNG: {path}")


def main() -> None:
	images_checked = 0
	links_checked = 0
	for readme in README_FILES:
		content = readme.read_text()
		parser = Links()
		parser.feed(content)
		images = parser.images + IMAGE_PATTERN.findall(content)
		links = parser.links + LINK_PATTERN.findall(content)
		if len(parser.images) < 8:
			raise AssertionError(f"README lacks its intended visual assets: {readme}")
		for source in images:
			if source.startswith(("http:", "https:", "//")):
				raise AssertionError(f"Remote image in README: {source}")
			path = readme.parent / source.split("#", 1)[0]
			if not path.is_file():
				raise AssertionError(f"Missing image: {path}")
			check_image(path)
			images_checked += 1
		for target in links:
			if target.startswith(("http:", "https:", "#")):
				continue
			path = readme.parent / target.split("#", 1)[0]
			if not path.exists():
				raise AssertionError(f"Missing local link: {path}")
			links_checked += 1
		if "brand/logo.svg" in content or ".github/readme/banner.svg" in content:
			raise AssertionError(f"Retired brand referenced by {readme}")
	print(f"README assets: {images_checked} PNG references and {links_checked} local links verified")


if __name__ == "__main__":
	main()
