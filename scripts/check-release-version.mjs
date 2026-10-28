import fs from 'node:fs'

const read = (file) => fs.readFileSync(file, 'utf8')
const version = JSON.parse(read('package.json')).version
const tag = process.argv[2]
if (tag && tag !== `v${version}`) throw new Error(`Tag ${tag} does not match v${version}`)

for (const file of ['apps/app/package.json', 'apps/app-frontend/package.json', 'apps/app/tauri.conf.json']) {
	if (JSON.parse(read(file)).version !== version) throw new Error(`${file} has a different version`)
}
for (const file of ['apps/app/Cargo.toml', 'packages/app-lib/Cargo.toml', 'packages/launcher-core/Cargo.toml']) {
	if (read(file).match(/^version = "([^"]+)"/m)?.[1] !== version) throw new Error(`${file} has a different version`)
}
const lock = read('Cargo.lock')
for (const crate of ['urod-launcher', 'urod-app-lib', 'urod-launcher-core']) {
	if (!lock.includes(`name = "${crate}"\nversion = "${version}"`)) throw new Error(`Cargo.lock has a different ${crate} version`)
}
if (read('packaging/arch/PKGBUILD').match(/^pkgver=(.+)$/m)?.[1] !== version) {
	throw new Error('Arch PKGBUILD has a different version')
}
if (!read('apps/app/src/skins.rs').includes(`UrodLauncher/${version}`)) {
	throw new Error('Skin API user agent has a different version')
}
for (const file of ['README.md', 'README.en.md']) {
	const content = read(file)
	for (const name of [
		`UrodLauncher-Setup-${version}.exe`,
		`UrodLauncher-${version}-amd64.deb`,
		`UrodLauncher-${version}-x86_64.pkg.tar.zst`,
	]) {
		if (!content.includes(`releases/download/v${version}/${name}`)) {
			throw new Error(`${file} does not link directly to ${name}`)
		}
	}
}
console.log(`Release versions and direct download links agree on ${version}`)
