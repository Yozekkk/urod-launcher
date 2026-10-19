<script setup lang="ts">
import { ref } from 'vue'
import { displayError, gameApi } from '../game-api'
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
	updateMemory: [value: number]
	updateWindowSize: [dimension: 'width' | 'height', value: string]
	updateJavaPath: [major: number, path: string]
	updateDownloadParallelism: [value: number]
}>()
const section = ref<Section>('global')
const localChanged = ref(false)
const javaBusy = ref(false)
const javaMessage = ref('')
const resourceMessage = ref('')
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
function updateDownloadParallelism(event: Event) {
	localChanged.value = true
	emit('updateDownloadParallelism', Number((event.target as HTMLInputElement).value))
}
async function openBackupFolder() {
	resourceMessage.value = ''
	try {
		await gameApi.openBackupFolder()
	} catch (reason) {
		resourceMessage.value = displayError(reason)
	}
}
function updateWindowSize(dimension: 'width' | 'height', event: Event) {
	localChanged.value = true
	emit('updateWindowSize', dimension, (event.target as HTMLInputElement).value)
}
function updateJavaPath(major: number, event: Event) {
	localChanged.value = true
	emit('updateJavaPath', major, (event.target as HTMLInputElement).value)
}
async function detectJava(major: number) {
	javaBusy.value = true
	javaMessage.value = ''
	try {
		const runtime = (await gameApi.javaRuntimes()).find((item) => item.major === major)
		if (!runtime) {
			javaMessage.value = `Java ${major} не найдена на этом устройстве.`
			return
		}
		localChanged.value = true
		emit('updateJavaPath', major, runtime.path)
		javaMessage.value = `Найдена Java ${major}. Сохраните настройку.`
	} catch (reason) {
		javaMessage.value = displayError(reason)
	} finally {
		javaBusy.value = false
	}
}
async function chooseJava(major: number) {
	javaBusy.value = true
	javaMessage.value = ''
	try {
		const runtime = await gameApi.pickJava()
		if (!runtime) return
		if (runtime.major !== major) {
			javaMessage.value = `Выбрана Java ${runtime.major}; для этого поля нужна Java ${major}.`
			return
		}
		localChanged.value = true
		emit('updateJavaPath', major, runtime.path)
	} catch (reason) {
		javaMessage.value = displayError(reason)
	} finally {
		javaBusy.value = false
	}
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
		<div
			class="figma-settings-body"
			:class="{ 'figma-settings-body-resources': section === 'resources' }"
		>
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
				<div class="figma-settings-extra">
					<section class="figma-settings-window-section" data-figma-node="3:280">
						<h2>Окно</h2>
						<div class="figma-settings-window-row">
							<div>
								<strong>Полноэкранный режим</strong>
								<small>Запускать игру в полноэкранном режиме (через options.txt).</small>
							</div>
							<button
								class="figma-settings-switch"
								disabled
								title="Управление полноэкранным режимом пока недоступно"
								aria-label="Полноэкранный режим пока недоступен"
							/>
						</div>
						<label class="figma-settings-window-row">
							<span><strong>Ширина</strong><small>Ширина окна игры при запуске.</small></span>
							<input
								type="number"
								min="320"
								max="7680"
								:value="settings.window_width ?? ''"
								placeholder="854"
								aria-label="Ширина окна игры"
								@input="updateWindowSize('width', $event)"
							/>
						</label>
						<label class="figma-settings-window-row">
							<span><strong>Высота</strong><small>Высота окна игры при запуске.</small></span>
							<input
								type="number"
								min="240"
								max="4320"
								:value="settings.window_height ?? ''"
								placeholder="480"
								aria-label="Высота окна игры"
								@input="updateWindowSize('height', $event)"
							/>
						</label>
						<h2>Java и память</h2>
						<label class="figma-settings-memory">
							<strong>Выделенная память</strong>
							<span>
								<small>512 МБ</small>
								<input
									type="range"
									min="512"
									:max="memoryMax"
									step="256"
									:value="settings.memory_mb"
									@input="updateMemory"
								/>
								<small>{{ (memoryMax / 1024).toFixed(1) }} ГБ</small>
								<output>{{ settings.memory_mb }}</output>
							</span>
							<small>Максимальный объём памяти, выделяемый игре.</small>
						</label>
					</section>
					<section class="figma-settings-advanced" data-figma-node="3:339">
						<label class="figma-settings-field"
							><strong>Аргументы Java</strong
							><input
								disabled
								placeholder="Введите аргументы Java..."
								title="Пользовательские аргументы Java пока недоступны"
							/><small>Аргументы, передаваемые Java при запуске игры.</small></label
						>
						<label class="figma-settings-field"
							><strong>Переменные среды</strong
							><input
								disabled
								placeholder="Введите переменные среды..."
								title="Переменные среды пока недоступны"
							/><small>Переменные среды при запуске игры.</small></label
						>
					</section>
					<section class="figma-settings-advanced figma-settings-commands" data-figma-node="3:367">
						<h2>Команды запуска</h2>
						<label class="figma-settings-field"
							><strong>Перед запуском</strong
							><input
								disabled
								placeholder="Введите команду перед запуском..."
								title="Команды перед запуском пока недоступны"
							/><small>Выполняется перед запуском игры.</small></label
						>
						<label class="figma-settings-field"
							><strong>Обёртка</strong
							><input
								disabled
								placeholder="Введите команду-обёртку..."
								title="Команды-обёртки пока недоступны"
							/><small>Команда-обёртка для запуска Minecraft.</small></label
						>
						<label class="figma-settings-field"
							><strong>После выхода</strong
							><input
								disabled
								placeholder="Введите команду после выхода..."
								title="Команды после выхода пока недоступны"
							/><small>Выполняется после выхода из игры.</small></label
						>
					</section>
					<section
						class="figma-settings-command-help"
						data-figma-node="3:402"
						aria-label="Справка по командам запуска; ввод команд пока недоступен"
					>
						<p>Команды выполняются в рабочем каталоге сборки. Переменные:</p>
						<div>
							<p>·$INST_NAME: название сборки</p>
							<p>·$INST_ID: имя папки сборки</p>
							<p>·$INST_DIR: абсолютный путь к папке сборки</p>
							<p>·$INST_MC_DIR: аналог $INST_DIR</p>
							<p>·$INST_JAVA: абсолютный путь к Java</p>
							<p>·$INST_JAVA_ARGS: аргументы JVM, передаваемые игре</p>
						</div>
					</section>
				</div>
			</div>
			<div
				v-else-if="section === 'java'"
				class="figma-settings-panel figma-java-panel"
				data-figma-node="3:104"
			>
				<div v-for="major in [25, 21]" :key="major" class="figma-java-section">
					<h2>Расположение Java {{ major }}</h2>
					<div class="figma-java-card">
						<div class="figma-java-path">
							<input
								:value="settings.java_paths[String(major)] || ''"
								:aria-label="`Путь к Java ${major}`"
								placeholder="Автоматически"
								spellcheck="false"
								@input="updateJavaPath(major, $event)"
							/>
							<img v-if="settings.java_paths[String(major)]" src="/figma/c00b7.svg" alt="" />
						</div>
						<div class="figma-java-actions">
							<button
								disabled
								aria-label="Установить рекомендуемую Java — пока недоступно"
								title="Автоматическая установка Java из настроек пока недоступна"
							>
								<img src="/figma/2c95b.svg" alt="" />Установить рекомендуемую
							</button>
							<button :disabled="busy || javaBusy" @click="detectJava(major)">
								<img src="/figma/2c95b.svg" alt="" />Обнаружить
							</button>
							<button :disabled="busy || javaBusy" @click="chooseJava(major)">
								<img src="/figma/d4b65.svg" alt="" />Указать
							</button>
						</div>
					</div>
				</div>
				<p v-if="javaMessage" class="figma-java-message" role="status">{{ javaMessage }}</p>
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
				<section class="figma-resource-advanced" data-figma-node="3:236">
					<div class="figma-resource-concurrency">
						<strong>Каталог приложения</strong>
						<div class="figma-resource-range">
							<span>1</span
							><input
								type="range"
								min="1"
								max="10"
								:value="settings.download_parallelism"
								:disabled="busy"
								aria-label="Количество одновременных загрузок"
								@input="updateDownloadParallelism"
							/><span>10</span><output>{{ settings.download_parallelism }}</output>
						</div>
						<p>
							Количество одновременно скачиваемых файлов. Уменьшите, если скачивание работает
							нестабильно. Требуется перезапуск.
						</p>
					</div>
					<div class="figma-resource-concurrency">
						<strong>Лимит одновременных записей</strong>
						<div class="figma-resource-range">
							<span>1</span
							><input
								type="range"
								min="1"
								max="50"
								value="50"
								disabled
								aria-label="Лимит одновременных записей пока недоступен"
							/><span>50</span><output>50</output>
						</div>
						<p>Количество одновременно записываемых на диск файлов. Настройка пока недоступна.</p>
					</div>
					<div class="figma-resource-backups">
						<strong>Резервные копии базы данных</strong>
						<button class="figma-resource-backup-open" @click="openBackupFolder">
							<img src="/figma/d4b65.svg" alt="" />Открыть папку резервных копий
						</button>
						<p>Здесь хранятся резервные копии важных данных приложения на случай их утери.</p>
						<p v-if="resourceMessage" role="alert">{{ resourceMessage }}</p>
					</div>
				</section>
			</div>
		</div>
	</section>
</template>
