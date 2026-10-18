<script setup lang="ts">
import { ref } from 'vue'
import type { Settings } from '../models'

type Section = 'global' | 'java' | 'resources'
defineProps<{
	settings: Settings
	dataDir: string
	busy: boolean
	changed: boolean
}>()
const emit = defineEmits<{
	close: []
	save: []
	pickJava: []
	updateMemory: [value: number]
	updateJava: [value: string]
}>()
const section = ref<Section>('global')
const localChanged = ref(false)
const memoryMax = 32768
const syncRows = [
	['Синхронизация настроек игры', 'Использовать одинаковые настройки игры в сборках.'],
	['Синхронизация серверов', 'Использовать одинаковые серверы в сборках.'],
	['Синхронизация наборов ресурсов', 'Использовать одинаковые наборы ресурсов в сборках.'],
	['Синхронизация истории команд', 'Использовать одинаковую историю команд в сборках.'],
	[
		'Синхронизация сохранённых инструментов',
		'Использовать одинаковые сохранённые инструменты в сборках.',
	],
] as const
function updateMemory(event: Event) {
	localChanged.value = true
	emit('updateMemory', Number((event.target as HTMLInputElement).value))
}
function updateJava(event: Event) {
	localChanged.value = true
	emit('updateJava', (event.target as HTMLInputElement).value)
}
function pickJava() {
	localChanged.value = true
	emit('pickJava')
}
</script>

<template>
	<section class="figma-settings-window" aria-label="Настройки лаунчера">
		<header class="figma-settings-header">
			<h1>Настройки</h1>
			<button
				v-if="changed && localChanged"
				class="figma-settings-save"
				:disabled="busy"
				@click="emit('save')"
			>
				{{ busy ? 'Сохранение…' : 'Сохранить' }}
			</button>
			<button class="figma-settings-close" aria-label="Закрыть настройки" @click="emit('close')">
				<img src="/figma/20469.svg" alt="" />
			</button>
		</header>
		<nav class="figma-settings-nav" aria-label="Разделы настроек">
			<h2>Сборки</h2>
			<button :class="{ selected: section === 'global' }" @click="section = 'global'">
				<img src="/figma/38640.svg" alt="" />Глобальные настройки
			</button>
			<button :class="{ selected: section === 'java' }" @click="section = 'java'">
				<img src="/figma/6a8c5.svg" alt="" />Расположение Java
			</button>
			<button :class="{ selected: section === 'resources' }" @click="section = 'resources'">
				<img src="/figma/bd47c.svg" alt="" />Управление ресурсами
			</button>
		</nav>
		<div class="figma-settings-body">
			<div v-if="section === 'global'" class="figma-settings-panel" data-figma-node="3:47">
				<div v-for="[title, description] in syncRows" :key="title" class="figma-settings-row">
					<div>
						<strong>{{ title }}</strong>
						<p>{{ description }}</p>
					</div>
					<button
						class="figma-settings-sync-control"
						:aria-label="title"
						aria-describedby="figma-sync-unavailable"
						:disabled="true"
						:title="'Синхронизация сборок пока недоступна'"
					>
						<img src="/figma/5e6f8.svg" alt="" />
					</button>
				</div>
				<p id="figma-sync-unavailable" class="sr-only">
					Синхронизация между сборками пока недоступна.
				</p>
				<div class="figma-settings-extra" data-figma-node="3:280,3:339,3:367">
					<h2>Java и память</h2>
					<label class="figma-settings-memory">
						<strong>Выделенная память</strong>
						<span
							><small>512 МБ</small
							><input
								type="range"
								min="512"
								:max="memoryMax"
								step="256"
								:value="settings.memory_mb"
								@input="updateMemory"
							/><small>{{ (memoryMax / 1024).toFixed(1) }} ГБ</small
							><output>{{ settings.memory_mb }}</output></span
						>
						<small>Максимальный объём памяти, выделяемый игре.</small>
					</label>
					<div class="figma-settings-field">
						<strong>Аргументы Java</strong>
						<input disabled placeholder="Аргументы Java пока недоступны" />
					</div>
					<div class="figma-settings-field">
						<strong>Команды запуска</strong>
						<input disabled placeholder="Команды запуска пока недоступны" />
					</div>
				</div>
			</div>
			<div v-else-if="section === 'java'" class="figma-settings-panel" data-figma-node="3:104">
				<h2>Расположение Java</h2>
				<div class="figma-settings-row">
					<div>
						<strong>Автоматический выбор Java</strong>
						<p>Лаунчер подбирает подходящую версию Java для каждой сборки.</p>
					</div>
					<span
						class="figma-settings-switch on"
						role="status"
						aria-label="Автоматический выбор Java включён"
					/>
				</div>
				<label class="figma-settings-field">
					<strong>Путь к Java</strong>
					<input
						:value="settings.java_path"
						placeholder="Автоматически"
						spellcheck="false"
						@input="updateJava"
					/>
					<small>Для всех новых сборок. Существующие сборки настраиваются отдельно.</small>
				</label>
				<button class="figma-settings-action" :disabled="busy" @click="pickJava">
					Выбрать Java вручную
				</button>
			</div>
			<div v-else class="figma-settings-panel figma-resource-panel" data-figma-node="3:199">
				<label class="figma-resource-section figma-resource-location">
					<strong>Каталог приложения</strong>
					<input :value="dataDir" readonly title="Каталог приложения" />
					<small
						>Здесь UrodLauncher хранит сборки и другие файлы. Каталог доступен только для
						просмотра.</small
					>
				</label>
				<div class="figma-resource-section figma-resource-report">
					<div>
						<strong>Отчёты на всех этапах установки</strong>
						<p>
							Показывать кнопку «Получить отчёт» на всех этапах установки. Для прерванных установок
							кнопка видна всегда.
						</p>
					</div>
					<button
						class="figma-resource-switch"
						disabled
						title="Расширенные отчёты пока недоступны"
						aria-label="Расширенные отчёты пока недоступны"
					>
						<img src="/figma/7c452.svg" alt="" />
					</button>
				</div>
				<div class="figma-resource-section figma-resource-cache">
					<strong>Кеш приложения</strong>
					<button
						class="figma-settings-action figma-resource-clear"
						disabled
						title="Безопасная очистка кеша ещё не поддерживается"
					>
						<img src="/figma/94db2.svg" alt="" /> Очистить кеш
					</button>
					<p>
						Удаляет кешированные данные и повторно скачивает их с urodlauncher. Приложение может
						замедлиться, пока кеш не восстановится.
					</p>
				</div>
			</div>
		</div>
	</section>
</template>
