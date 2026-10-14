<script setup lang="ts">
import { computed, shallowRef } from 'vue'
import {
	defaults,
	fields,
	presets,
	sections,
	validateCustomization,
	type Customization,
	type Field,
} from '../customization'
const model = defineModel<Customization>({ required: true })
const error = shallowRef('')
const fileInput = shallowRef<HTMLInputElement | null>(null)
const count = computed(() => fields.length)
const optionNames: Record<string, string> = {
	dark: 'Тёмная',
	light: 'Светлая',
	system: 'Системная',
	red: 'Красная',
	crimson: 'Бордовая',
	violet: 'Фиолетовая',
	blue: 'Синяя',
	custom: 'Свой цвет',
	solid: 'Однотонный',
	gradient: 'Градиент',
	image: 'Изображение',
	graphite: 'Графит',
	city: 'Город',
	ember: 'Жар',
	cover: 'Заполнить',
	contain: 'Уместить',
	center: 'По центру',
	top: 'Сверху',
	bottom: 'Снизу',
	left: 'Слева',
	right: 'Справа',
	lift: 'Подъём',
	glow: 'Свечение',
	none: 'Без эффекта',
	fade: 'Плавный',
	slide: 'Сдвиг',
	tabs: 'Вкладки',
	pills: 'Капсулы',
	underline: 'Подчёркивание',
	compact: 'Компактная',
	comfortable: 'Обычная',
	spacious: 'Просторная',
	'community-first': 'Сообщество сначала',
	'server-first': 'Сервер сначала',
	outline: 'Контур',
	glass: 'Стекло',
	'top-right': 'Сверху справа',
	'bottom-right': 'Снизу справа',
	ru: 'Русский',
	en: 'English',
}
function optionLabel(option: string) {
	return model.value.language === 'en' ? option : optionNames[option] || option
}
function update(field: Field, value: string | boolean) {
	const typed = field.kind === 'range' ? Number(value) : value
	model.value = { ...model.value, [field.key]: typed }
}
function reset(key: keyof Customization) {
	model.value = { ...model.value, [key]: defaults[key] }
}
function preset(name: string) {
	model.value = { ...defaults, ...presets[name] }
	error.value = ''
}
function exportJson() {
	const blob = new Blob([JSON.stringify(model.value, null, 2)], { type: 'application/json' })
	const url = URL.createObjectURL(blob)
	const link = document.createElement('a')
	link.href = url
	link.download = 'urodlauncher-customization.json'
	link.click()
	setTimeout(() => URL.revokeObjectURL(url), 1000)
}
async function importJson(event: Event) {
	const file = (event.target as HTMLInputElement).files?.[0]
	if (!file) return
	try {
		if (file.size > 3_000_000) throw Error('Файл слишком большой')
		model.value = validateCustomization(JSON.parse(await file.text()))
		error.value = ''
	} catch (reason) {
		error.value = reason instanceof Error ? reason.message : 'Не удалось импортировать файл'
	}
	if (fileInput.value) fileInput.value.value = ''
}
function uploadBackground(event: Event) {
	const file = (event.target as HTMLInputElement).files?.[0]
	if (!file) return
	if (!['image/png', 'image/jpeg', 'image/webp'].includes(file.type) || file.size > 2_000_000) {
		error.value = 'Выберите PNG, JPG или WebP до 2 МБ'
		return
	}
	const reader = new FileReader()
	reader.onload = () => {
		if (typeof reader.result === 'string')
			model.value = { ...model.value, customBackground: reader.result, backgroundType: 'image' }
	}
	reader.onerror = () => {
		error.value = 'Не удалось прочитать фон'
	}
	reader.readAsDataURL(file)
}
</script>
<template>
	<div class="customization-panel settings-section">
		<div class="customization-heading">
			<div>
				<h2>Кастомизация</h2>
				<p>{{ count }} параметров · изменения видны сразу и сохраняются автоматически.</p>
			</div>
			<button class="button secondary" @click="model = { ...defaults }">Сбросить всё</button>
		</div>
		<div class="customization-presets">
			<button
				v-for="(_, label) in presets"
				:key="label"
				class="button secondary"
				@click="preset(label)"
			>
				{{ label }}</button
			><button class="button secondary" @click="exportJson">Экспорт JSON</button
			><label class="button secondary"
				>Импорт JSON<input
					ref="fileInput"
					type="file"
					accept="application/json,.json"
					aria-label="Импорт настроек JSON"
					@change="importJson"
			/></label>
		</div>
		<p v-if="error" role="alert" class="customization-error">{{ error }}</p>
		<section v-for="section in sections" :key="section.title" class="customization-group">
			<h3>{{ section.title }}</h3>
			<div v-for="field in section.fields" :key="field.key" class="customization-row">
				<label :for="`custom-${field.key}`">{{ field.label }}</label
				><select
					v-if="field.kind === 'select'"
					:id="`custom-${field.key}`"
					:value="model[field.key]"
					@change="update(field, ($event.target as HTMLSelectElement).value)"
				>
					<option v-for="option in field.options" :key="option" :value="option">
						{{ optionLabel(option) }}
					</option></select
				><input
					v-else-if="field.kind === 'range'"
					:id="`custom-${field.key}`"
					type="range"
					:min="field.min"
					:max="field.max"
					:step="field.step"
					:value="model[field.key]"
					@input="update(field, ($event.target as HTMLInputElement).value)"
				/><input
					v-else-if="field.kind === 'color'"
					:id="`custom-${field.key}`"
					type="color"
					:value="model[field.key]"
					@input="update(field, ($event.target as HTMLInputElement).value)"
				/><input
					v-else-if="field.kind === 'toggle'"
					:id="`custom-${field.key}`"
					type="checkbox"
					:checked="model[field.key] === true"
					@change="update(field, ($event.target as HTMLInputElement).checked)"
				/><input
					v-else
					:id="`custom-${field.key}`"
					type="file"
					accept="image/png,image/jpeg,image/webp"
					@change="uploadBackground"
				/><output v-if="field.kind === 'range'">{{ model[field.key] }}</output
				><button
					class="customization-reset"
					:aria-label="`Сбросить ${field.label}`"
					title="Сбросить параметр"
					@click="reset(field.key)"
				>
					↺
				</button>
			</div>
		</section>
	</div>
</template>
<style scoped>
.customization-panel {
	padding: 24px;
}
.customization-heading {
	display: flex;
	justify-content: space-between;
	align-items: center;
	gap: 16px;
}
.customization-heading p {
	color: var(--muted);
}
.customization-presets {
	display: flex;
	flex-wrap: wrap;
	gap: 8px;
	margin: 16px 0;
}
.customization-presets label {
	position: relative;
	overflow: hidden;
}
.customization-presets input {
	position: absolute;
	inset: 0;
	opacity: 0;
	cursor: pointer;
}
.customization-group {
	border-top: 1px solid var(--line);
	padding: 16px 0;
}
.customization-group h3 {
	font-size: 16px;
	margin: 0 0 12px;
}
.customization-row {
	display: grid;
	grid-template-columns: minmax(160px, 1fr) minmax(140px, 220px) 50px 30px;
	align-items: center;
	gap: 12px;
	min-height: 42px;
}
.customization-row > select,
.customization-row > input:not([type='checkbox']):not([type='color']) {
	width: 100%;
}
.customization-row select {
	background: var(--control);
	color: var(--foreground);
	border: 1px solid var(--line);
	border-radius: 8px;
	padding: 6px;
}
.customization-row output {
	font-size: 12px;
	color: var(--muted);
	text-align: right;
}
.customization-reset {
	border: 0;
	background: transparent;
	color: var(--muted);
	font-size: 18px;
}
.customization-reset:hover {
	color: var(--foreground);
}
.customization-error {
	color: #ff8a88;
}
@media (max-width: 750px) {
	.customization-row {
		grid-template-columns: 1fr 1fr 36px;
	}
	.customization-row output {
		display: none;
	}
}
</style>
