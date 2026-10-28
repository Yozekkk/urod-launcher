export const defaults = {
	theme: 'dark',
	palette: 'red',
	accentCustom: '#d73547',
	secondaryColor: '#ff817f',
	fontFamily: 'Geologica',
	fontSize: 14,
	fontWeight: 400,
	scale: 1,
	radius: 12,
	borderWidth: 1,
	shadow: 0.5,
	panelOpacity: 0.95,
	backgroundType: 'solid',
	builtinBackground: 'graphite',
	customBackground: '',
	backgroundSize: 'cover',
	backgroundPosition: 'center',
	backgroundDim: 0.4,
	overlayColor: '#11171c',
	backgroundBlur: 0,
	reducedMotion: false,
	animationSpeed: 1,
	hoverStyle: 'lift',
	pageTransition: 'fade',
	focusGlow: true,
	buttonMotion: true,
	navStyle: 'tabs',
	sidebarWidth: 220,
	density: 'comfortable',
	homeOrder: 'community-first',
	showServer: true,
	showNews: true,
	cardStyle: 'solid',
	libraryColumns: 3,
	playPosition: 'right',
	showVersions: true,
	showModCount: true,
	showAccountHead: true,
	toastPosition: 'top-right',
	language: 'ru',
} as const
export type Customization = {
	[K in keyof typeof defaults]: (typeof defaults)[K] extends number
		? number
		: (typeof defaults)[K] extends boolean
			? boolean
			: string
}
export type Field = {
	key: keyof Customization
	label: string
	kind: 'select' | 'range' | 'color' | 'toggle' | 'image'
	options?: readonly string[]
	min?: number
	max?: number
	step?: number
}
const choices = (key: keyof Customization, label: string, options: readonly string[]): Field => ({
	key,
	label,
	kind: 'select',
	options,
})
const range = (
	key: keyof Customization,
	label: string,
	min: number,
	max: number,
	step: number,
): Field => ({ key, label, kind: 'range', min, max, step })
const color = (key: keyof Customization, label: string): Field => ({ key, label, kind: 'color' })
const toggle = (key: keyof Customization, label: string): Field => ({ key, label, kind: 'toggle' })
export const sections: { title: string; fields: Field[] }[] = [
	{
		title: 'Внешний вид',
		fields: [
			choices('theme', 'Тема', ['dark', 'light', 'system']),
			choices('palette', 'Акцентная палитра', ['red', 'crimson', 'violet', 'blue', 'custom']),
			color('accentCustom', 'Свой акцент'),
			color('secondaryColor', 'Дополнительный цвет'),
			choices('fontFamily', 'Шрифт', ['Geologica', 'Inter', 'Manrope', 'Poppins', 'system-ui']),
			range('fontSize', 'Размер текста', 12, 18, 1),
			range('fontWeight', 'Толщина шрифта', 400, 700, 100),
			range('scale', 'Масштаб', 0.8, 1.3, 0.05),
			range('radius', 'Скругление', 0, 24, 1),
			range('borderWidth', 'Толщина границ', 0, 3, 1),
			range('shadow', 'Интенсивность теней', 0, 1, 0.1),
			range('panelOpacity', 'Прозрачность панелей', 0.65, 1, 0.05),
		],
	},
	{
		title: 'Фон приложения',
		fields: [
			choices('backgroundType', 'Тип фона', ['solid', 'gradient', 'image']),
			choices('builtinBackground', 'Встроенный фон', ['graphite', 'city', 'ember']),
			{ key: 'customBackground', label: 'Своё изображение', kind: 'image' },
			choices('backgroundSize', 'Масштаб изображения', ['cover', 'contain']),
			choices('backgroundPosition', 'Положение изображения', [
				'center',
				'top',
				'bottom',
				'left',
				'right',
			]),
			range('backgroundDim', 'Затемнение', 0, 0.9, 0.05),
			color('overlayColor', 'Цвет наложения'),
			range('backgroundBlur', 'Размытие', 0, 20, 1),
		],
	},
	{
		title: 'Анимации',
		fields: [
			toggle('reducedMotion', 'Уменьшить движение'),
			range('animationSpeed', 'Скорость анимаций', 0.5, 2, 0.1),
			choices('hoverStyle', 'Эффект наведения', ['lift', 'glow', 'none']),
			choices('pageTransition', 'Переход страниц', ['fade', 'slide', 'none']),
			toggle('focusGlow', 'Подсветка фокуса'),
			toggle('buttonMotion', 'Микроанимации кнопок'),
		],
	},
	{
		title: 'Компоновка',
		fields: [
			choices('navStyle', 'Стиль навигации', ['tabs', 'pills', 'underline']),
			range('sidebarWidth', 'Ширина панели', 180, 300, 10),
			choices('density', 'Плотность', ['compact', 'comfortable', 'spacious']),
			choices('cardStyle', 'Стиль карточек', ['solid', 'outline', 'glass']),
			range('libraryColumns', 'Колонки библиотеки', 1, 4, 1),
		],
	},
	{
		title: 'Дополнительное отображение',
		fields: [
			choices('playPosition', 'Кнопка запуска', ['left', 'right']),
			toggle('showVersions', 'Версии Minecraft'),
			toggle('showModCount', 'Количество модов'),
			toggle('showAccountHead', 'Голова аккаунта в шапке'),
			choices('toastPosition', 'Положение уведомлений', ['top-right', 'bottom-right']),
			choices('language', 'Язык интерфейса', ['ru', 'en']),
		],
	},
]
export const fields = sections.flatMap((section) => section.fields)
const legacyHomeFields = [
	choices('homeOrder', 'Порядок главной', ['community-first', 'server-first']),
	toggle('showServer', 'Карточка сервера'),
	toggle('showNews', 'Блок новостей'),
]
const hex = /^#[0-9a-fA-F]{6}$/
export function validateCustomization(input: unknown): Customization {
	if (!input || typeof input !== 'object' || Array.isArray(input))
		throw Error('Ожидается объект настроек')
	const source = input as Record<string, unknown>
	if (Object.keys(source).some((key) => !(key in defaults)))
		throw Error('Файл содержит неизвестные параметры')
	const result = { ...defaults } as Customization
	for (const field of [...fields, ...legacyHomeFields]) {
		const value =
			field.key === 'homeOrder' && source[field.key] === 'academy-first'
				? 'community-first'
				: source[field.key]
		if (value === undefined) continue
		if (
			field.kind === 'range' &&
			(typeof value !== 'number' ||
				!Number.isFinite(value) ||
				value < field.min! ||
				value > field.max!)
		)
			throw Error(`Некорректное значение: ${field.label}`)
		if (field.kind === 'toggle' && typeof value !== 'boolean')
			throw Error(`Некорректное значение: ${field.label}`)
		if (field.kind === 'color' && (typeof value !== 'string' || !hex.test(value)))
			throw Error(`Некорректный цвет: ${field.label}`)
		if (field.kind === 'select' && !field.options?.includes(String(value)))
			throw Error(`Некорректный выбор: ${field.label}`)
		if (
			field.kind === 'image' &&
			(typeof value !== 'string' ||
				(value !== '' &&
					(!/^data:image\/(png|jpeg|webp);base64,[A-Za-z0-9+/=]+$/.test(value) ||
						value.length > 2_800_000)))
		)
			throw Error('Некорректное фоновое изображение')
		;(result as unknown as Record<string, unknown>)[field.key] = value
	}
	return result
}
export function normalizeCustomization(input: unknown): Customization {
	const result = { ...defaults } as Customization
	if (!input || typeof input !== 'object' || Array.isArray(input)) return result
	const source = input as Record<string, unknown>
	for (const key of Object.keys(defaults) as (keyof Customization)[]) {
		if (!(key in source)) continue
		try {
			;(result as unknown as Record<string, unknown>)[key] = validateCustomization({
				[key]: source[key],
			})[key]
		} catch {
			// Keep valid saved options when one older or damaged option cannot be used.
		}
	}
	return result
}
export const presets: Record<string, Partial<Customization>> = {
	Город: {
		palette: 'red',
		theme: 'dark',
		backgroundType: 'gradient',
		builtinBackground: 'city',
		cardStyle: 'solid',
	},
	Минимализм: {
		palette: 'crimson',
		backgroundType: 'solid',
		cardStyle: 'outline',
		hoverStyle: 'none',
		reducedMotion: true,
	},
	Светлая: { theme: 'light', palette: 'blue', backgroundType: 'solid', cardStyle: 'solid' },
}
const paletteColors: Record<string, string> = {
	red: '#d73547',
	crimson: '#b42a3b',
	violet: '#9269d9',
	blue: '#498ad6',
}
export function applyCustomization(input: unknown) {
	const value = normalizeCustomization(input)
	const root = document.documentElement
	const theme =
		value.theme === 'system'
			? matchMedia('(prefers-color-scheme: light)').matches
				? 'light'
				: 'dark'
			: value.theme
	root.dataset.theme = theme
	root.dataset.navStyle = value.navStyle
	root.dataset.density = value.density
	root.dataset.cardStyle = value.cardStyle
	root.dataset.hoverStyle = value.hoverStyle
	root.dataset.pageTransition = value.pageTransition
	root.dataset.focusGlow = String(value.focusGlow)
	root.dataset.buttonMotion = String(value.buttonMotion)
	root.dataset.playPosition = value.playPosition
	root.dataset.showVersions = String(value.showVersions)
	root.dataset.showModCount = String(value.showModCount)
	root.dataset.showAccountHead = String(value.showAccountHead)
	root.dataset.toastPosition = value.toastPosition
	root.lang = value.language
	root.style.setProperty(
		'--accent-custom',
		value.palette === 'custom' ? value.accentCustom : paletteColors[value.palette]!,
	)
	root.style.setProperty('--accent-secondary', value.secondaryColor)
	root.style.setProperty('--ui-font', value.fontFamily)
	root.style.setProperty('--ui-font-size', `${value.fontSize}px`)
	root.style.setProperty('--ui-font-weight', String(value.fontWeight))
	root.style.setProperty('--ui-scale', String(value.scale))
	root.style.setProperty('--ui-radius', `${value.radius}px`)
	root.style.setProperty('--ui-border', `${value.borderWidth}px`)
	root.style.setProperty('--ui-shadow', String(value.shadow))
	root.style.setProperty('--ui-opacity', String(value.panelOpacity))
	root.style.setProperty(
		'--ui-panel-bg',
		`${theme === 'light' ? '#ffffff' : '#151c22'}${Math.round(value.panelOpacity * 255)
			.toString(16)
			.padStart(2, '0')}`,
	)
	root.style.setProperty('--ui-sidebar-width', `${value.sidebarWidth}px`)
	root.style.setProperty('--ui-columns', String(value.libraryColumns))
	root.style.setProperty('--ui-motion', value.reducedMotion ? '0' : String(value.animationSpeed))
	root.style.setProperty('--ui-bg-size', value.backgroundSize)
	root.style.setProperty('--ui-bg-position', value.backgroundPosition)
	root.style.setProperty('--ui-bg-blur', `${value.backgroundBlur}px`)
	root.style.setProperty(
		'--ui-bg-overlay',
		theme === 'light' && value.backgroundType === 'solid'
			? '#00000000'
			: `${value.overlayColor}${Math.round(value.backgroundDim * 255)
					.toString(16)
					.padStart(2, '0')}`,
	)
	root.style.setProperty('--ui-bg-dim', String(value.backgroundDim))
	const builtin: Record<string, string> = {
		graphite: 'none',
		city: 'linear-gradient(135deg,#26252b,#11171c 52%,#351b26)',
		ember: 'radial-gradient(circle at 75% 20%,#632432,#11171c 60%)',
	}
	root.style.setProperty(
		'--ui-bg-image',
		value.backgroundType === 'image' && value.customBackground
			? `url("${value.customBackground}")`
			: value.backgroundType === 'gradient'
				? builtin[value.builtinBackground]!
				: 'none',
	)
	root.dataset.backgroundType = value.backgroundType
}
