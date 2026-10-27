#!/usr/bin/env python3
"""Exercise Library startup and account/skin persistence in isolated Tauri data.

Requires a debug UrodLauncher binary, tauri-agent-tools, a desktop session, and
the frontend dev server at http://localhost:1420 (run `pnpm app:dev` once to build
the binary, then run `pnpm --dir apps/app-frontend dev` separately).
"""

import argparse
import base64
import json
import os
from pathlib import Path
import shutil
import subprocess
import tempfile
import time
from urllib.request import urlopen


ROOT = Path(__file__).resolve().parent.parent


def bridge(pid: int, operation: str, *args: str) -> object:
	command = ["tauri-agent-tools", operation, *args, "--pid", str(pid), "--json"]
	result = subprocess.run(command, capture_output=True, text=True, timeout=15, check=False)
	if result.returncode:
		raise RuntimeError(f"{operation} failed: {result.stderr[:200]}")
	value = json.loads(result.stdout)
	if value.get("error") or value.get("success") is False:
		raise RuntimeError(f"{operation} failed: {str(value.get('error'))[:200]}")
	return value.get("result")


def invoke(pid: int, name: str, arguments: dict | None = None) -> object:
	return bridge(pid, "invoke", name, json.dumps(arguments or {}))


def start(binary: Path, environment: dict) -> subprocess.Popen:
	return subprocess.Popen([str(binary)], env=environment, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)


def stop(process: subprocess.Popen, force: bool = False) -> None:
	if process.poll() is not None:
		return
	if force:
		process.kill()
	else:
		process.terminate()
	try:
		process.wait(timeout=8)
	except subprocess.TimeoutExpired:
		process.kill()
		process.wait(timeout=5)


def ready(pid: int) -> None:
	deadline = time.monotonic() + 25
	while time.monotonic() < deadline:
		try:
			result = bridge(pid, "eval", "({hash:location.hash,title:document.querySelector('h1')?.textContent,nav:document.querySelectorAll('.figma-sidebar a').length})")
			if isinstance(result, dict) and result.get("title") == "Библиотека":
				assert result["hash"] == "#/library" and result["nav"] == 5, result
				return
		except (RuntimeError, subprocess.TimeoutExpired):
			pass
		time.sleep(0.25)
	raise RuntimeError(f"Library did not become ready for PID {pid}")


def main() -> None:
	parser = argparse.ArgumentParser(description=__doc__)
	parser.add_argument("--binary", type=Path, default=ROOT / "target/debug/urod-launcher")
	parser.add_argument("--launches", type=int, default=20)
	args = parser.parse_args()
	if args.launches < 2 or not args.binary.is_file():
		parser.error("Use an existing debug binary and at least two launches")
	if shutil.which("tauri-agent-tools") is None:
		parser.error("tauri-agent-tools is required")
	try:
		with urlopen("http://localhost:1420", timeout=2) as response:
			if response.status != 200:
				raise RuntimeError("Frontend dev server is unavailable")
	except OSError as error:
		parser.error(f"Start the frontend dev server first: {error}")

	directory = Path(tempfile.mkdtemp(prefix="urod-library-e2e-"))
	environment = os.environ.copy()
	environment["UROD_TEST_DATA_DIR"] = str(directory)
	first = None
	try:
		first = start(args.binary, environment)
		ready(first.pid)
		legacy = bridge(first.pid, "eval", "(async()=>{location.hash='/home';await new Promise(resolve=>setTimeout(resolve,100));return location.hash})()")
		assert legacy == "#/library", legacy
		accounts = []
		for number in range(10):
			name = f"LibraryQA{number:02}"
			snapshot = invoke(first.pid, "add_offline", {"nickname": name})
			accounts.append(next(item["uuid"] for item in snapshot["accounts"] if item["nickname"] == name))
		for account in accounts:
			snapshot = invoke(first.pid, "set_active", {"uuid": account})
			assert next(item["uuid"] for item in snapshot["accounts"] if item["active"]) == account
		renamed = invoke(first.pid, "rename_offline", {"uuid": accounts[9], "nickname": "ChangedQA09"})
		assert any(item["uuid"] == accounts[9] and item["nickname"] == "ChangedQA09" for item in renamed["accounts"])
		invoke(first.pid, "remove_account", {"uuid": accounts[9]})
		recreated = invoke(first.pid, "add_offline", {"nickname": "LibraryQA09"})
		assert any(item["uuid"] == accounts[9] for item in recreated["accounts"])
		texture = base64.b64encode((ROOT / "apps/app/assets/steve.png").read_bytes()).decode()
		customization = bridge(first.pid, "eval", """(async()=>{
			const {defaults,fields}=await import('/src/customization.ts');
			const values={...defaults};
			for(const field of fields){
				const current=values[field.key];
				if(field.kind==='select') values[field.key]=field.options.find(option=>option!==current);
				if(field.kind==='range') values[field.key]=current===field.min?field.max:field.min;
				if(field.kind==='color') values[field.key]='#123456';
				if(field.kind==='toggle') values[field.key]=!current;
			}
			values.homeOrder='server-first';
			values.showServer=false;
			values.showNews=false;
			return values;
		})()""")
		assert isinstance(customization, dict) and len(customization) == 40
		customization["customBackground"] = f"data:image/png;base64,{texture}"
		saved = invoke(first.pid, "save_customization", {"customization": customization})
		assert saved["settings"]["customization"] == customization
		skins = []
		for number in range(15):
			items = invoke(first.pid, "skin_library_save", {"name": f"QA Skin {number}", "slim": bool(number % 2), "data": texture})
			skins.append(items[0]["id"])
		assert len(set(skins)) == 15
		for position, account in enumerate(accounts[:3]):
			for skin in skins[position : position + 3]:
				invoke(first.pid, "skin_library_apply", {"id": skin, "account": account})
		stop(first)
		first = None

		for number in range(1, args.launches + 1):
			process = start(args.binary, environment)
			try:
				ready(process.pid)
				state = bridge(process.pid, "eval", "(async()=>{const i=window.__TAURI_INTERNALS__.invoke;const a=await i('snapshot');const s=await i('skin_library_list');return {accounts:a.accounts.length,skins:s.length,customization:a.settings.customization}})()")
				assert state["accounts"] == 10 and state["skins"] == 15, state
				assert state["customization"] == customization
				for account in accounts[:3]:
					entries = invoke(process.pid, "skin_library_history", {"account": account})
					assert len(entries) == 3 and all(item["account_uuid"] == account for item in entries)
			finally:
				stop(process, force=number == args.launches // 2)
			print(f"launch {number}/{args.launches}: Library, accounts, skins, history and 40 settings persisted", flush=True)
		print(json.dumps({"launches": args.launches, "accounts": 10, "skins": 15, "history_per_test_account": 3, "customization_keys": 40}))
	finally:
		if first is not None:
			stop(first)
		shutil.rmtree(directory)


if __name__ == "__main__":
	main()
