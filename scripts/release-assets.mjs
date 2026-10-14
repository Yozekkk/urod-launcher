import fs from 'node:fs'
import path from 'node:path'
import { fileURLToPath } from 'node:url'

const tagPattern = /^v\d+\.\d+\.\d+(?:-[0-9A-Za-z.-]+)?$/

export function releaseNames(tag) {
	if (!tagPattern.test(tag)) throw new Error(`Invalid release tag: ${tag}`)
	const version = tag.slice(1)
	return {
		windows: `UrodLauncher-Setup-${version}.exe`,
		debian: `UrodLauncher-${version}-amd64.deb`,
		arch: `UrodLauncher-${version}-x86_64.pkg.tar.zst`,
		source: `UrodLauncher-${tag}-source.tar.gz`,
	}
}

export function stageReleaseArtifact({ tag, platform, bundleRoot, outputDir }) {
	const names = releaseNames(tag)
	const source = platform === 'win32'
		? path.join(bundleRoot, 'nsis', names.windows)
		: platform === 'linux-deb'
			? path.join(bundleRoot, 'deb', names.debian)
			: platform === 'linux-arch'
				? path.join(bundleRoot, 'arch', names.arch)
				: null
	if (!source) throw new Error(`Unsupported release platform: ${platform}`)
	if (!fs.statSync(source).isFile() || fs.statSync(source).size === 0)
		throw new Error(`Empty release artifact: ${source}`)
	fs.mkdirSync(outputDir, { recursive: true })
	fs.copyFileSync(source, path.join(outputDir, path.basename(source)))
}

const invokedAsScript = process.argv[1] && path.resolve(process.argv[1]) === fileURLToPath(import.meta.url)
if (invokedAsScript) {
	const [platform] = process.argv.slice(2)
	const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..')
	stageReleaseArtifact({
		tag: process.env.RELEASE_TAG ?? process.env.GITHUB_REF_NAME,
		platform,
		bundleRoot: path.join(root, 'target', 'release', 'bundle'),
		outputDir: path.join(root, 'release-assets'),
	})
}
