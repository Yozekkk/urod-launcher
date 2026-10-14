import assert from 'node:assert/strict'
import fs from 'node:fs'
import os from 'node:os'
import path from 'node:path'
import { test } from 'node:test'
import { releaseNames, stageReleaseArtifact } from './release-assets.mjs'

test('release stages exactly the three named packages and requires real files', () => {
	const root = fs.mkdtempSync(path.join(os.tmpdir(), 'urod-release-test-'))
	try {
		const names = releaseNames('v0.1.0')
		const bundleRoot = path.join(root, 'bundle')
		const outputDir = path.join(root, 'assets')
		for (const [platform, folder, name] of [
			['win32', 'nsis', names.windows],
			['linux-deb', 'deb', names.debian],
			['linux-arch', 'arch', names.arch],
		]) {
			fs.mkdirSync(path.join(bundleRoot, folder), { recursive: true })
			fs.writeFileSync(path.join(bundleRoot, folder, name), 'package')
			stageReleaseArtifact({ tag: 'v0.1.0', platform, bundleRoot, outputDir })
		}
		assert.deepEqual(fs.readdirSync(outputDir).sort(), [names.windows, names.debian, names.arch].sort())
		fs.rmSync(path.join(bundleRoot, 'arch', names.arch))
		assert.throws(() => stageReleaseArtifact({ tag: 'v0.1.0', platform: 'linux-arch', bundleRoot, outputDir }))
		assert.throws(() => releaseNames('v0.1.0/../file'), /Invalid release tag/)
	} finally {
		fs.rmSync(root, { recursive: true, force: true })
	}
})
