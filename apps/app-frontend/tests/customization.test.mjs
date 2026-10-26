import test from 'node:test'
import assert from 'node:assert/strict'
import {
	applyCustomization,
	defaults,
	fields,
	normalizeCustomization,
	validateCustomization,
} from '../src/customization.ts'

test('visible customization excludes three obsolete home controls while all forty keys remain valid', () => {
	assert.equal(fields.length, 37)
	assert.deepEqual(new Set(fields.map((field) => field.key)).size, 37)
	assert.deepEqual(
		new Set(Object.keys(defaults).filter((key) => !fields.some((field) => field.key === key))),
		new Set(['homeOrder', 'showServer', 'showNews']),
	)
	assert.deepEqual(validateCustomization(JSON.parse(JSON.stringify(defaults))), defaults)
	assert.deepEqual(normalizeCustomization({ theme: 'invalid' }), defaults)
	assert.equal(
		normalizeCustomization({ fontFamily: 'Inter', theme: 'invalid' }).fontFamily,
		'Inter',
	)
	assert.equal(normalizeCustomization({ fontFamily: 'Geologica' }).fontFamily, 'Geologica')
	assert.throws(() => validateCustomization({ unknown: true }), /неизвестные/)
	assert.throws(
		() => validateCustomization({ customBackground: 'javascript:alert(1)' }),
		/изображение/,
	)
	assert.throws(() => validateCustomization({ fontSize: 999 }), /Размер текста/)
})

test('theme, color, layout and motion change rendered root immediately', () => {
	const styles = new Map()
	const root = {
		dataset: {},
		style: { setProperty: (key, value) => styles.set(key, value) },
		lang: '',
	}
	globalThis.document = { documentElement: root }
	globalThis.matchMedia = () => ({ matches: true })
	applyCustomization({
		...defaults,
		theme: 'system',
		palette: 'custom',
		accentCustom: '#123456',
		backgroundType: 'solid',
		reducedMotion: true,
		density: 'compact',
		libraryColumns: 4,
		language: 'en',
	})
	assert.equal(root.dataset.theme, 'light')
	assert.equal(root.dataset.density, 'compact')
	assert.equal(root.lang, 'en')
	assert.equal(styles.get('--accent-custom'), '#123456')
	assert.equal(styles.get('--ui-columns'), '4')
	assert.equal(styles.get('--ui-motion'), '0')
	assert.equal(styles.get('--ui-bg-overlay'), '#00000000')
})

test('previous home order keeps other saved customization values', () => {
	const migrated = validateCustomization({ ...defaults, homeOrder: 'academy-first', fontSize: 16 })
	assert.equal(migrated.homeOrder, 'community-first')
	assert.equal(migrated.fontSize, 16)
})
