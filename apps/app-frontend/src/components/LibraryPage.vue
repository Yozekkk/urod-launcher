<script setup lang="ts">
import { computed, nextTick, onMounted, onUnmounted, ref, watch } from 'vue'
import { operationTitle, operationMessage } from '../operation-labels'
import AppIcon from './AppIcon.vue'
import OperationPanel from './OperationPanel.vue'
import {
	gameApi,
	displayError,
	loaderNames,
	statusLabel,
	type CreateInstance,
	type Instance,
	type InstalledContent,
	type GameVersion,
	type LoaderVersion,
	type Loader,
	type ContentUpdate,
	type UpdatePlan,
} from '../game-api'
import { isTerminalPhase, operationPercent, type GameOperations } from '../game-operations'
import type { Settings } from '../models'

const props = defineProps<{
	detail: string | null
	settings: Settings
	accountName: string | null
	operations: GameOperations
}>()
const emit = defineEmits<{ 'open-log': [] }>()
const instances = ref<Instance[]>([])
const mods = ref<InstalledContent[]>([])
const updates = ref<ContentUpdate[]>([])
const loading = ref(true)
const busy = ref(false)
const error = ref('')
const notice = ref('')
const librarySearch = ref('')
const librarySort = ref<'name' | 'recent'>('name')
const createDialog = ref<HTMLDialogElement | null>(null)
const manageDialog = ref<HTMLDialogElement | null>(null)
const updateDialog = ref<HTMLDialogElement | null>(null)
const modPluralRules = new Intl.PluralRules('ru')
function modCountLabel(count: number): string {
	const labels: Record<string, string> = { one: 'мод', few: 'мода', many: 'модов', other: 'модов' }
	return `${count} ${labels[modPluralRules.select(count)]}`
}
const removeModDialog = ref<HTMLDialogElement | null>(null)
const editionUpdateDialog = ref<HTMLDialogElement | null>(null)
const editionPlan = ref<UpdatePlan | null>(null)
type OfficialUpdateStatus = {
	phase: 'checking' | 'current' | 'available' | 'error'
	version?: string
}
const officialUpdateStatus = ref<Record<string, OfficialUpdateStatus>>({})
const editionHasChanges = computed(
	() =>
		!!editionPlan.value &&
		!!(
			editionPlan.value.added.length +
			editionPlan.value.changed.length +
			editionPlan.value.removed.length
		),
)
const rollbackDialog = ref<HTMLDialogElement | null>(null)
const rollbackAvailable = ref(false)
const nameInput = ref<HTMLInputElement | null>(null)
const createName = ref('')
const gameVersion = ref('')
const loader = ref<Loader>('vanilla')
const loaderVersion = ref('')
const versions = ref<GameVersion[]>([])
const loaderVersions = ref<LoaderVersion[]>([])
const metadataLoading = ref(false)
const metadataError = ref('')
const includeSnapshots = ref(false)
const managed = ref<Instance | null>(null)
const manageName = ref('')
const manageMemory = ref(4096)
const manageJava = ref('')
const manageWindowEnabled = ref(false)
const manageWindowWidth = ref(854)
const manageWindowHeight = ref(480)
const manageAction = ref<'rename' | 'duplicate' | 'delete' | 'configure'>('rename')
const packSettingsTab = ref<'main' | 'installation' | 'overrides'>('main')
const removingMod = ref<InstalledContent | null>(null)
const selectedUpdateIds = ref<string[]>([])
const modFilter = ref('')
const modCategory = ref<'all' | 'mod' | 'shader' | 'resourcepack'>('all')
let metadataGeneration = 0
let contentGeneration = 0
let mounted = true
let gameTimer: ReturnType<typeof setTimeout> | undefined
let instanceGeneration = 0
let rollbackGeneration = 0
let officialUpdateGeneration = 0
const current = computed(() => instances.value.find((instance) => instance.id === props.detail))
const visibleVersions = computed(() =>
	versions.value.filter((version) => includeSnapshots.value || version.type === 'release'),
)
const visibleMods = computed(() =>
	mods.value
		.filter((mod) => mod.name.toLowerCase().includes(modFilter.value.toLowerCase()))
		.filter((mod) => modCategory.value === 'all' || mod.kind === modCategory.value)
		.sort((a, b) => a.name.localeCompare(b.name, 'ru')),
)
const libraryInstances = computed(() =>
	[...instances.value]
		.filter((instance) =>
			instance.name.toLocaleLowerCase().includes(librarySearch.value.toLocaleLowerCase()),
		)
		.sort((a, b) =>
			librarySort.value === 'recent'
				? (b.last_played || 0) - (a.last_played || 0)
				: a.name.localeCompare(b.name, 'ru'),
		),
)
const localJobs = computed(() =>
	props.operations.active.value.filter(
		(job) => !current.value || job.instance_id === current.value.id,
	),
)
const instanceBusy = (id: string) =>
	props.operations.active.value.some((job) => job.instance_id === id)
const dates = new Intl.DateTimeFormat('ru', { day: 'numeric', month: 'short', year: 'numeric' })
const lastPlayed = (instance: Instance) =>
	instance.last_played ? dates.format(new Date(instance.last_played * 1000)) : 'Ещё не запускалась'
async function loadRollbackAvailability() {
	const instance = current.value
	const generation = ++rollbackGeneration
	rollbackAvailable.value = false
	if (!instance) return
	try {
		const available = await gameApi.rollbackAvailable(instance.id)
		if (mounted && generation === rollbackGeneration) rollbackAvailable.value = available
	} catch {
		if (mounted && generation === rollbackGeneration) rollbackAvailable.value = false
	}
}
async function loadOfficialUpdateStatus() {
	const generation = ++officialUpdateGeneration
	const official = instances.value.filter(
		(instance) =>
			instance.kind === 'official' && instance.status === 'ready' && !!instance.manifest_version,
	)
	officialUpdateStatus.value = Object.fromEntries(
		official.map((instance) => [instance.id, { phase: 'checking' }]),
	)
	await Promise.all(
		official.map(async (instance) => {
			let status: OfficialUpdateStatus
			try {
				const plan = await gameApi.checkEditionUpdate(instance.id, props.settings.release_channel)
				status =
					plan.from_version !== plan.to_version ||
					plan.added.length + plan.changed.length + plan.removed.length > 0
						? { phase: 'available', version: plan.to_version }
						: { phase: 'current' }
			} catch {
				status = { phase: 'error' }
			}
			if (mounted && generation === officialUpdateGeneration)
				officialUpdateStatus.value = { ...officialUpdateStatus.value, [instance.id]: status }
		}),
	)
}
function requestFor(instance: Instance, name: string): CreateInstance {
	return {
		name,
		game_version: instance.game_version,
		loader: instance.loader,
		loader_version: instance.loader_version,
		memory_mb: instance.memory_mb,
		window_width: instance.window_width,
		window_height: instance.window_height,
		java_path: instance.java_path,
	}
}
function dismissError() {
	error.value = ''
	props.operations.clearError()
}
function showRemoveMod(mod: InstalledContent) {
	removingMod.value = mod
	removeModDialog.value?.showModal()
}
function scheduleGameRefresh() {
	clearTimeout(gameTimer)
	gameTimer = undefined
	if (mounted && instances.value.some((instance) => instance.status === 'running'))
		gameTimer = setTimeout(() => void loadInstances(true), 3000)
}
async function loadInstances(quiet = false) {
	const generation = ++instanceGeneration
	if (!quiet) {
		loading.value = true
		error.value = ''
	}
	let loaded = false
	try {
		const result = await gameApi.instances()
		if (!mounted || generation !== instanceGeneration) return
		instances.value = result
		loaded = true
		if (!quiet) {
			void loadRollbackAvailability()
			void loadOfficialUpdateStatus()
		}
		if (!quiet && props.detail) await loadContent()
	} catch (reason) {
		if (mounted && generation === instanceGeneration) error.value = displayError(reason)
	} finally {
		if (mounted && generation === instanceGeneration) {
			loading.value = false
			if (loaded) scheduleGameRefresh()
			else clearTimeout(gameTimer)
		}
	}
}
async function loadContent() {
	const id = props.detail
	const generation = ++contentGeneration
	mods.value = []
	updates.value = []
	if (!id) return
	try {
		const result = await gameApi.content(id)
		if (generation === contentGeneration) mods.value = result
	} catch (reason) {
		if (generation === contentGeneration) error.value = displayError(reason)
	}
}
async function operation(action: () => Promise<unknown>, success = '', reload = true) {
	busy.value = true
	error.value = ''
	try {
		await action()
		if (success) notice.value = success
		if (reload) await loadInstances()
		return true
	} catch (reason) {
		error.value = displayError(reason)
		return false
	} finally {
		busy.value = false
	}
}
async function loadMetadata() {
	metadataLoading.value = true
	metadataError.value = ''
	try {
		versions.value = await gameApi.gameVersions()
		gameVersion.value = versions.value.find((version) => version.type === 'release')?.id || ''
	} catch (reason) {
		metadataError.value = displayError(reason)
	} finally {
		metadataLoading.value = false
	}
}
async function loadLoaderVersions() {
	const generation = ++metadataGeneration
	loaderVersions.value = []
	loaderVersion.value = ''
	if (loader.value === 'vanilla' || !gameVersion.value) {
		metadataLoading.value = false
		metadataError.value = ''
		return
	}
	metadataLoading.value = true
	metadataError.value = ''
	try {
		const result = await gameApi.loaderVersions(loader.value, gameVersion.value)
		if (generation === metadataGeneration) {
			loaderVersions.value = result
			loaderVersion.value = result.find((version) => version.stable)?.id || result[0]?.id || ''
			if (!result.length)
				metadataError.value =
					'Для этой версии Minecraft загрузчик недоступен. Выберите другую версию.'
		}
	} catch (reason) {
		if (generation === metadataGeneration) metadataError.value = displayError(reason)
	} finally {
		if (generation === metadataGeneration) metadataLoading.value = false
	}
}
async function showCreate() {
	createName.value = ''
	loader.value = 'vanilla'
	metadataError.value = ''
	error.value = ''
	createDialog.value?.showModal()
	await nextTick()
	nameInput.value?.focus()
	await loadMetadata()
}
async function createInstance() {
	if (
		!createName.value.trim() ||
		!gameVersion.value ||
		(loader.value !== 'vanilla' && !loaderVersion.value)
	) {
		metadataError.value = 'Укажите название, версию Minecraft и доступную версию загрузчика.'
		return
	}
	busy.value = true
	metadataError.value = ''
	try {
		const result = await gameApi.create({
			name: createName.value.trim(),
			game_version: gameVersion.value,
			loader: loader.value,
			loader_version: loader.value === 'vanilla' ? null : loaderVersion.value,
			memory_mb: props.settings.memory_mb,
			window_width: null,
			window_height: null,
			java_path: Object.values(props.settings.java_paths).some(Boolean)
				? null
				: props.settings.java_path || null,
		})
		createDialog.value?.close()
		await loadInstances()
		location.hash = `/library/${result.id}`
	} catch (reason) {
		metadataError.value = displayError(reason)
	} finally {
		busy.value = false
	}
}
function showManage(instance: Instance, action: 'rename' | 'duplicate' | 'delete' | 'configure') {
	managed.value = instance
	manageName.value = action === 'duplicate' ? `${instance.name} — копия` : instance.name
	manageMemory.value = instance.memory_mb
	manageJava.value = instance.java_path || ''
	manageWindowEnabled.value = instance.window_width != null && instance.window_height != null
	manageWindowWidth.value = instance.window_width ?? props.settings.window_width ?? 854
	manageWindowHeight.value = instance.window_height ?? props.settings.window_height ?? 480
	manageAction.value = action
	packSettingsTab.value = 'main'
	error.value = ''
	if (!manageDialog.value?.open) manageDialog.value?.showModal()
}
async function saveManagedName() {
	if (!managed.value || manageName.value.trim() === managed.value.name) return
	manageAction.value = 'rename'
	await manageInstance()
}
async function pickManagedJava() {
	if (!managed.value) return
	try {
		const runtime = await gameApi.pickJava(managed.value.id)
		if (runtime) manageJava.value = runtime.path
	} catch (reason) {
		error.value = displayError(reason)
	}
}
async function manageInstance() {
	const instance = managed.value
	if (!instance) return
	if (
		manageAction.value === 'configure' &&
		manageWindowEnabled.value &&
		(!Number.isInteger(manageWindowWidth.value) ||
			!Number.isInteger(manageWindowHeight.value) ||
			manageWindowWidth.value < 320 ||
			manageWindowWidth.value > 7680 ||
			manageWindowHeight.value < 240 ||
			manageWindowHeight.value > 4320)
	) {
		error.value = 'Укажите ширину 320–7680 и высоту 240–4320.'
		return
	}
	const success = await operation(
		async () => {
			if (manageAction.value === 'delete') {
				await gameApi.remove(instance.id)
				if (props.detail === instance.id) location.hash = '/library'
			} else if (manageAction.value === 'duplicate')
				await gameApi.duplicate(instance.id, manageName.value.trim())
			else if (manageAction.value === 'configure')
				await gameApi.edit(instance.id, {
					...requestFor(instance, instance.name),
					memory_mb: manageMemory.value,
					window_width: manageWindowEnabled.value ? manageWindowWidth.value : null,
					window_height: manageWindowEnabled.value ? manageWindowHeight.value : null,
					java_path: manageJava.value.trim() || null,
				})
			else await gameApi.edit(instance.id, requestFor(instance, manageName.value.trim()))
		},
		manageAction.value === 'delete' ? 'Сборка удалена из библиотеки' : 'Библиотека обновлена',
	)
	if (success) manageDialog.value?.close()
}
async function installOrPlay(instance: Instance) {
	if (instance.status === 'running') {
		await operation(() => gameApi.stop(instance.id), 'Игра остановлена')
		return
	}
	if (instance.status !== 'ready' || (instance.kind === 'official' && !instance.manifest_version)) {
		await operation(async () =>
			props.operations.track(
				await (instance.kind === 'official'
					? gameApi.resumeEditionInstall(instance.id, props.settings.release_channel)
					: gameApi.installGame(instance.id)),
			),
		)
		return
	}
	if (!props.accountName) {
		error.value = 'Добавьте или выберите активный аккаунт на странице «Аккаунты».'
		return
	}
	await operation(() => gameApi.launch(instance.id), 'Minecraft запущен')
}
async function importPack() {
	await operation(async () => {
		const path = await gameApi.pickImport()
		if (!path) return
		const result = await gameApi.importPack(path, '')
		location.hash = `/library/${result.id}`
	})
}
async function exportPack(instance: Instance) {
	await operation(async () => {
		const path = await gameApi.pickExport(instance.id)
		if (path) {
			await gameApi.exportPack(instance.id, path)
			notice.value = 'Экспорт завершён'
		}
	})
}
async function chooseIcon(instance: Instance) {
	await operation(() => gameApi.pickIcon(instance.id))
}
async function checkUpdates() {
	const instance = current.value
	if (instance)
		await operation(
			async () => {
				updates.value = await gameApi.checkUpdates(instance.id)
				await loadContent()
			},
			'',
			false,
		)
	if (!updates.value.length && !error.value) notice.value = 'Совместимых обновлений не найдено'
}
function showUpdates(ids: string[]) {
	selectedUpdateIds.value = ids
	error.value = ''
	updateDialog.value?.showModal()
}
async function applyUpdates() {
	if (!current.value) return
	const id = current.value.id
	if (
		await operation(async () =>
			props.operations.track(await gameApi.updateContent(id, selectedUpdateIds.value)),
		)
	)
		updateDialog.value?.close()
}
async function toggleMod(mod: InstalledContent) {
	if (await operation(() => gameApi.toggleContent(mod.id, !mod.enabled))) await loadContent()
}
async function removeMod() {
	if (
		removingMod.value &&
		(await operation(() => gameApi.removeContent(removingMod.value!.id), 'Мод удалён'))
	)
		removeModDialog.value?.close()
}
async function checkOfficialUpdate() {
	const instance = current.value
	if (!instance) return
	editionPlan.value = null
	if (
		await operation(
			async () => {
				editionPlan.value = await gameApi.checkEditionUpdate(
					instance.id,
					props.settings.release_channel,
				)
			},
			'',
			false,
		)
	)
		editionUpdateDialog.value?.showModal()
}
async function applyOfficialUpdate() {
	const instance = current.value
	if (!instance || !editionPlan.value) return
	if (
		await operation(async () =>
			props.operations.track(
				await gameApi.applyEditionUpdate(instance.id, props.settings.release_channel),
			),
		)
	)
		editionUpdateDialog.value?.close()
}
async function rollback() {
	if (
		current.value &&
		(await operation(
			() => gameApi.rollback(current.value!.id),
			'Предыдущее состояние восстановлено',
		))
	)
		rollbackDialog.value?.close()
}
watch([loader, gameVersion], () => void loadLoaderVersions())
watch(
	() => props.settings.release_channel,
	() => void loadOfficialUpdateStatus(),
)
watch(
	() => props.detail,
	() => {
		error.value = ''
		notice.value = ''
		void loadContent()
		void loadRollbackAvailability()
	},
)
watch(
	() =>
		props.operations.jobs.value
			.filter((job) => isTerminalPhase(job.phase))
			.map((job) => job.id)
			.join(','),
	() => void loadInstances(),
)
onMounted(() => void loadInstances())
onUnmounted(() => {
	mounted = false
	instanceGeneration++
	rollbackGeneration++
	officialUpdateGeneration++
	contentGeneration++
	clearTimeout(gameTimer)
})
</script>

<template>
	<section class="page library-page" :class="{ 'figma-pack-page': !!current }">
		<div class="page-heading">
			<div>
				<h1>
					{{ current ? current.name : 'Библиотека' }}
				</h1>
				<p v-if="current" class="page-intro">
					{{
						current
							? `Minecraft ${current.game_version} · ${loaderNames[current.loader]}${current.loader_version ? ` ${current.loader_version}` : ''}`
							: 'Создавай свои сборки и возвращайся в любимые миры.'
					}}
				</p>
			</div>
			<div class="heading-actions">
				<button
					id="create-instance-button"
					class="button primary"
					:disabled="busy"
					@click="showCreate"
				>
					+ Новая Сборка
				</button>
			</div>
		</div>
		<div v-if="!current" class="figma-library-toolbar">
			<input v-model="librarySearch" type="search" aria-label="Поиск сборок" placeholder="Поиск" />
			<select v-model="librarySort" aria-label="Сортировка сборок">
				<option value="name">Название</option>
				<option value="recent">Последний запуск</option>
			</select>
			<button class="figma-library-import" :disabled="busy" @click="importPack">
				Импорт .mrpack
			</button>
		</div>
		<div v-if="error || operations.error.value" class="banner error inline-banner" role="alert">
			{{ error || operations.error.value
			}}<button class="icon-button" aria-label="Закрыть ошибку" @click="dismissError">
				<AppIcon name="close" :size="16" />
			</button>
		</div>
		<div v-if="notice" class="banner inline-banner" role="status">
			{{ notice
			}}<button class="icon-button" aria-label="Закрыть сообщение" @click="notice = ''">
				<AppIcon name="close" :size="16" />
			</button>
		</div>
		<OperationPanel
			v-for="job in localJobs"
			:key="job.id"
			:title="operationTitle(job.operation)"
			:message="operationMessage(job)"
			:progress="operationPercent(job)"
			:bytes-per-second="job.phase === 'downloading' ? job.bytes_per_second : undefined"
			:cancellable="job.cancellable"
			:cancelling="operations.cancelling.value === job.id"
			@cancel="operations.cancel(job.id)"
		/>
		<div v-if="loading" class="section-loading" role="status">
			<span class="spinner" />Загружаем библиотеку…
		</div>
		<template v-else-if="current">
			<a class="back-link" href="#/library"><AppIcon name="arrow" :size="16" />Все сборки</a>
			<div class="figma-pack-content">
				<nav class="figma-pack-tabs" aria-label="Разделы сборки">
					<span class="selected"><img src="/figma/6d925.svg" alt="" />Контент</span>
					<button @click="operation(() => gameApi.openFolder(current!.id))">
						<img src="/figma/8c998.svg" alt="" />Файлы
					</button>
					<button disabled title="Просмотр миров внутри лаунчера пока недоступен">
						<img src="/figma/46b9c.svg" alt="" />Миры
					</button>
					<button class="figma-pack-mobile-log" @click="emit('open-log')">Журнал</button>
				</nav>
				<div class="figma-pack-search-row">
					<input
						v-model="modFilter"
						type="search"
						aria-label="Поиск контента сборки"
						placeholder="Поиск"
					/>
					<a :href="`#/content?instance=${current.id}`">Добавить файлы</a>
				</div>
				<div class="figma-pack-categories" aria-label="Категории контента">
					<span>Названия (А-Я)</span>
					<button
						v-for="entry in [
							['all', 'Всё'],
							['mod', 'Моды'],
							['shader', 'Шейдеры'],
							['resourcepack', 'Наборы ресурсов'],
						] as const"
						:key="entry[0]"
						:class="{ selected: modCategory === entry[0] }"
						@click="modCategory = entry[0]"
					>
						{{ entry[1] }}
					</button>
				</div>
				<div class="figma-pack-list" aria-label="Установленный контент">
					<div
						v-for="mod in visibleMods"
						:key="mod.id"
						class="figma-pack-item"
						:class="{ disabled: !mod.enabled }"
					>
						<img src="/figma/268b2.png" alt="" />
						<div class="figma-pack-item-name">
							<a v-if="mod.project_id" :href="`#/content/${mod.project_id}`">{{ mod.name }}</a
							><strong v-else>{{ mod.name }}</strong
							><small>{{ mod.managed ? 'Файл сборки сообщества' : 'Установленный файл' }}</small>
						</div>
						<div class="figma-pack-item-version">
							<span>{{ mod.version_number || 'Локальная версия' }}</span
							><small>{{ mod.path.split(/[\\/]/).pop() }}</small>
						</div>
						<button
							class="figma-pack-item-update"
							:disabled="
								!updates.some((update) => update.content_id === mod.id) ||
								mod.managed ||
								busy ||
								instanceBusy(current.id) ||
								current.status === 'running'
							"
							:title="
								updates.some((update) => update.content_id === mod.id)
									? 'Установить обновление'
									: 'Обновление не найдено'
							"
							:aria-label="`Обновить ${mod.name}`"
							@click="showUpdates([mod.id])"
						>
							<img src="/figma/3d4ee.svg" alt="" />
						</button>
						<button
							class="switch"
							:class="{ on: mod.enabled }"
							role="switch"
							:aria-checked="mod.enabled"
							:aria-label="`${mod.enabled ? 'Выключить' : 'Включить'} ${mod.name}`"
							:disabled="
								mod.managed || busy || instanceBusy(current.id) || current.status === 'running'
							"
							@click="toggleMod(mod)"
						>
							<img v-if="mod.enabled" src="/figma/f8869.svg" alt="" /><span v-else />
						</button>
						<button
							class="figma-pack-item-delete"
							:aria-label="`Удалить ${mod.name}`"
							:disabled="
								mod.managed || busy || instanceBusy(current.id) || current.status === 'running'
							"
							@click="showRemoveMod(mod)"
						>
							<img src="/figma/dc443.svg" alt="" />
						</button>
					</div>
					<p v-if="!visibleMods.length" class="figma-pack-empty">
						{{
							mods.length
								? 'По запросу контент не найден'
								: 'В сборке пока нет установленного контента'
						}}
					</p>
				</div>
				<div class="figma-pack-bottom">
					<img v-if="current.icon" :src="current.icon" alt="" /><img
						v-else
						src="/figma/8aecb.png"
						alt=""
					/><span
						><strong>{{ current.name }}</strong
						><small>{{ loaderNames[current.loader] }} {{ current.game_version }}</small></span
					><button
						class="figma-pack-settings-trigger"
						aria-label="Настройки сборки"
						:disabled="busy || instanceBusy(current.id) || current.status === 'running'"
						@click="showManage(current, 'configure')"
					>
						<img src="/figma/bd7df.svg" alt="" /></button
					><button
						class="figma-pack-play"
						:disabled="
							busy ||
							instanceBusy(current.id) ||
							(current.status === 'ready' &&
								(current.kind !== 'official' || !!current.manifest_version) &&
								!accountName)
						"
						@click="installOrPlay(current)"
					>
						{{
							current.status === 'running'
								? 'СТОП'
								: current.status === 'ready'
									? 'ИГРАТЬ'
									: 'УСТАНОВИТЬ'
						}}
					</button>
				</div>
			</div>
			<details class="figma-pack-advanced">
				<summary
					aria-label="Дополнительные действия и сведения о сборке"
					title="Дополнительные действия и сведения о сборке"
				>
					⋯
				</summary>
				<div class="instance-hero">
					<img v-if="current.icon" :src="current.icon" width="90" height="90" alt="Иконка сборки" />
					<div v-else class="instance-icon"><AppIcon name="game" :size="38" /></div>
					<div class="instance-hero-copy">
						<span v-if="current.kind === 'official'" class="official-library-badge"
							><AppIcon name="check" :size="12" />Сообщество</span
						>
						<span class="quiet-badge">{{
							current.kind === 'official' && !current.manifest_version
								? 'Требуется завершить установку'
								: statusLabel(current.status)
						}}</span>
						<p>
							{{ modCountLabel(current.mod_count ?? mods.length) }} ·
							{{ (current.memory_mb / 1024).toFixed(1) }} ГБ RAM
						</p>
						<small>{{ lastPlayed(current) }}</small>
					</div>
					<button
						class="button primary"
						:disabled="
							busy ||
							instanceBusy(current.id) ||
							(current.status === 'ready' &&
								(current.kind !== 'official' || !!current.manifest_version) &&
								!accountName)
						"
						@click="installOrPlay(current)"
					>
						<AppIcon name="game" />{{
							current.status === 'running'
								? 'Остановить'
								: current.status === 'ready' &&
									  (current.kind !== 'official' || !!current.manifest_version)
									? 'Играть'
									: current.kind === 'official'
										? 'Завершить установку'
										: 'Установить Minecraft'
						}}
					</button>
				</div>
				<p v-if="!accountName" class="instance-account-note">
					Для запуска выбери <a href="#/accounts">аккаунт Minecraft</a>. Устанавливать файлы можно
					уже сейчас.
				</p>
				<p
					v-if="current.kind === 'official' && officialUpdateStatus[current.id]"
					class="official-update-inline"
					role="status"
				>
					{{
						officialUpdateStatus[current.id].phase === 'checking'
							? 'Проверяем обновления сборки…'
							: officialUpdateStatus[current.id].phase === 'available'
								? `Доступно обновление ${officialUpdateStatus[current.id].version}`
								: officialUpdateStatus[current.id].phase === 'current'
									? 'Сборка актуальна'
									: 'Не удалось проверить обновления сборки'
					}}
				</p>
				<div class="instance-toolbar">
					<button
						class="button subtle"
						:disabled="busy || instanceBusy(current.id) || current.status === 'running'"
						@click="showManage(current, 'configure')"
					>
						<AppIcon name="settings" :size="16" />Параметры
					</button>
					<button
						v-if="current.kind === 'official'"
						class="button secondary"
						:disabled="busy || instanceBusy(current.id)"
						@click="checkOfficialUpdate"
					>
						<AppIcon name="refresh" :size="16" />Обновления сборки сообщества
					</button>
					<button
						class="button subtle"
						:disabled="busy || instanceBusy(current.id)"
						@click="showManage(current, 'rename')"
					>
						Переименовать</button
					><button
						class="button subtle"
						:disabled="busy || instanceBusy(current.id) || current.status === 'running'"
						@click="chooseIcon(current)"
					>
						Иконка</button
					><button class="button subtle" @click="operation(() => gameApi.openFolder(current!.id))">
						<AppIcon name="folder" :size="16" />Папка</button
					><button
						class="button subtle"
						:disabled="busy || instanceBusy(current.id)"
						@click="showManage(current, 'duplicate')"
					>
						Дублировать</button
					><button
						class="button subtle"
						:disabled="busy || instanceBusy(current.id)"
						@click="exportPack(current)"
					>
						Экспорт .mrpack</button
					><button
						class="button delete-button"
						:disabled="busy || instanceBusy(current.id) || current.status === 'running'"
						@click="showManage(current, 'delete')"
					>
						Удалить
					</button>
				</div>
				<div class="content-section-heading">
					<div>
						<h2>
							Моды сборки <span>{{ mods.length }}</span>
						</h2>
						<p>
							{{
								current.kind === 'official'
									? 'Официальные файлы обновляются отдельно. Свои моды можно добавлять и удалять.'
									: 'Включай, обновляй и управляй установленным контентом.'
							}}
						</p>
					</div>
					<div class="heading-actions">
						<a class="button secondary" :href="`#/content?instance=${current.id}`"
							><AppIcon name="plus" :size="17" />Добавить моды</a
						><button
							class="button secondary"
							:disabled="busy || instanceBusy(current.id)"
							@click="checkUpdates"
						>
							<AppIcon name="refresh" :size="17" />Проверить обновления
						</button>
					</div>
				</div>
				<div v-if="updates.length" class="update-summary">
					<AppIcon name="refresh" :size="20" /><span
						>Доступно совместимых обновлений: {{ updates.length }}</span
					><button
						class="button primary"
						:disabled="busy || instanceBusy(current.id)"
						@click="showUpdates(updates.map((update) => update.content_id))"
					>
						Обновить все
					</button>
				</div>
				<div class="instance-footer">
					<span>{{ current.directory }}</span
					><button
						v-if="rollbackAvailable"
						class="button subtle"
						:disabled="busy || instanceBusy(current.id) || current.status === 'running'"
						@click="rollbackDialog?.showModal()"
					>
						Восстановить предыдущую версию
					</button>
				</div>
			</details>
		</template>
		<div v-else-if="detail" class="empty-state">
			<AppIcon name="info" :size="36" />
			<h2>Сборка не найдена</h2>
			<p>Она могла быть удалена или перемещена.</p>
			<a href="#/library" class="button secondary">Открыть библиотеку</a>
		</div>
		<div v-else-if="!libraryInstances.length" class="empty-state library-empty">
			<p>{{ librarySearch ? 'Сборки не найдены' : 'Сборок пока нет' }}</p>
		</div>
		<div v-else class="instance-grid">
			<article v-for="instance in libraryInstances" :key="instance.id" class="instance-card">
				<a :href="`#/library/${instance.id}`" class="instance-card-link"
					><div class="instance-cover">
						<img v-if="instance.icon" :src="instance.icon" width="80" height="80" alt="" />
					</div>
					<span v-if="instance.kind === 'official'" class="official-library-badge"
						><AppIcon name="check" :size="12" />Сообщество</span
					>
					<h2>{{ instance.name }}</h2>
					<p>{{ loaderNames[instance.loader] }} {{ instance.game_version }}</p>
					<small>{{ lastPlayed(instance) }}</small></a
				>
				<div class="instance-card-bottom">
					<span class="instance-state">{{
						instance.status === 'ready' &&
						!!instance.manifest_version &&
						instance.kind === 'official' &&
						officialUpdateStatus[instance.id]?.phase === 'available'
							? 'Доступно обновление'
							: instance.kind === 'official' && !instance.manifest_version
								? 'Требуется завершить установку'
								: statusLabel(instance.status)
					}}</span
					><button
						class="button primary"
						:disabled="
							busy ||
							instanceBusy(instance.id) ||
							(instance.status === 'ready' &&
								(instance.kind !== 'official' || !!instance.manifest_version) &&
								!accountName)
						"
						@click="installOrPlay(instance)"
					>
						<AppIcon name="game" :size="16" />{{
							instance.status === 'running'
								? 'Стоп'
								: instance.status === 'ready' &&
									  (instance.kind !== 'official' || !!instance.manifest_version)
									? 'Играть'
									: instance.kind === 'official'
										? 'Завершить установку'
										: 'Установить'
						}}
					</button>
					<details class="instance-menu">
						<summary aria-label="Действия со сборкой">⋯</summary>
						<div>
							<button
								:disabled="instanceBusy(instance.id) || instance.status === 'running'"
								@click="showManage(instance, 'configure')"
							>
								Параметры</button
							><a :href="`#/library/${instance.id}`">Открыть сборку</a
							><button
								:disabled="busy || instanceBusy(instance.id) || instance.status === 'running'"
								@click="showManage(instance, 'rename')"
							>
								Переименовать</button
							><button
								:disabled="busy || instanceBusy(instance.id) || instance.status === 'running'"
								@click="chooseIcon(instance)"
							>
								Изменить иконку</button
							><button @click="operation(() => gameApi.openFolder(instance.id))">
								Открыть папку</button
							><button
								:disabled="instanceBusy(instance.id) || instance.status === 'running'"
								@click="showManage(instance, 'duplicate')"
							>
								Дублировать</button
							><button :disabled="instanceBusy(instance.id)" @click="exportPack(instance)">
								Экспорт .mrpack</button
							><button
								class="danger-text"
								:disabled="instanceBusy(instance.id) || instance.status === 'running'"
								@click="showManage(instance, 'delete')"
							>
								Удалить
							</button>
						</div>
					</details>
				</div>
			</article>
		</div>
		<dialog
			ref="editionUpdateDialog"
			class="modal update-dialog"
			aria-labelledby="edition-update-title"
		>
			<button
				class="modal-close icon-button"
				aria-label="Закрыть обновление сборки сообщества"
				:disabled="busy"
				@click="editionUpdateDialog?.close()"
			>
				<AppIcon name="close" />
			</button>
			<p class="eyebrow">ОФИЦИАЛЬНАЯ СБОРКА UROD</p>
			<h2 id="edition-update-title">Обновление сборки</h2>
			<template v-if="editionPlan"
				><p class="modal-intro">
					{{ editionPlan.from_version || 'Первая установка' }} → {{ editionPlan.to_version }} ·
					{{ (editionPlan.download_bytes / 1048576).toFixed(1) }} МБ
				</p>
				<p v-if="!editionHasChanges" class="field-help" role="status">
					Сборка актуальна. Изменений для установки нет.
				</p>
				<p v-else class="field-help">Доступно обновление. Проверьте изменения перед установкой.</p>
				<div class="update-plan-counts">
					<span>+ {{ editionPlan.added.length }} новых</span
					><span>↻ {{ editionPlan.changed.length }} изменённых</span
					><span>− {{ editionPlan.removed.length }} удалённых</span>
				</div>
				<p class="field-help">
					Лаунчер обновляет только файлы, которыми управляет сборка. Миры, скриншоты и личные
					настройки сохраняются.
				</p>
				<div v-if="editionPlan.conflicts.length" class="compatibility-warning">
					Локальные изменения требуют внимания:
					<ul>
						<li v-for="path in editionPlan.conflicts" :key="path">{{ path }}</li>
					</ul>
				</div>
				<details v-if="editionHasChanges" class="update-plan-files">
					<summary>Изменения файлов</summary>
					<ul>
						<li v-for="path in editionPlan.added" :key="'add' + path">+ {{ path }}</li>
						<li v-for="path in editionPlan.changed" :key="'change' + path">↻ {{ path }}</li>
						<li v-for="path in editionPlan.removed" :key="'remove' + path">− {{ path }}</li>
					</ul>
				</details>
				<p v-if="editionPlan.changelog" class="update-changelog">{{ editionPlan.changelog }}</p>
				<div v-if="error" class="banner error" role="alert">{{ error }}</div>
				<div class="dialog-actions">
					<button class="button subtle" :disabled="busy" @click="editionUpdateDialog?.close()">
						Отмена</button
					><button
						class="button primary"
						:disabled="busy || editionPlan.conflicts.length > 0 || !editionHasChanges"
						@click="applyOfficialUpdate"
					>
						Обновить сборку
					</button>
				</div></template
			>
		</dialog>
		<dialog
			ref="createDialog"
			class="modal create-instance-modal"
			aria-labelledby="create-instance-title"
		>
			<button
				class="modal-close icon-button"
				aria-label="Закрыть создание сборки"
				:disabled="busy"
				@click="createDialog?.close()"
			>
				<AppIcon name="close" />
			</button>
			<div class="modal-symbol"><AppIcon name="game" :size="27" /></div>
			<p class="eyebrow">НОВОЕ ПРИКЛЮЧЕНИЕ</p>
			<h2 id="create-instance-title">Создать сборку</h2>
			<p class="modal-intro">
				Выбери Minecraft и загрузчик.<br />Файлы установятся из официальных источников.
			</p>
			<form @submit.prevent="createInstance">
				<label for="instance-name">Название</label
				><input
					id="instance-name"
					ref="nameInput"
					v-model="createName"
					name="instance-name"
					autocomplete="off"
					maxlength="80"
					placeholder="Например, Мой новый мир"
					required
					:disabled="busy"
				/>
				<div class="form-columns">
					<div>
						<label for="instance-loader">Загрузчик</label
						><select id="instance-loader" v-model="loader" :disabled="busy || metadataLoading">
							<option v-for="(name, id) in loaderNames" :key="id" :value="id">{{ name }}</option>
						</select>
					</div>
					<div>
						<label for="instance-version">Minecraft</label
						><select
							id="instance-version"
							v-model="gameVersion"
							:disabled="busy || metadataLoading"
							required
						>
							<option v-if="!visibleVersions.length" value="">
								{{ metadataLoading ? 'Загружаем…' : 'Нет доступных версий' }}
							</option>
							<option v-for="version in visibleVersions" :key="version.id" :value="version.id">
								{{ version.id }}
							</option>
						</select>
					</div>
				</div>
				<label v-if="loader !== 'vanilla'" for="loader-version"
					>Версия {{ loaderNames[loader] }}</label
				><select
					v-if="loader !== 'vanilla'"
					id="loader-version"
					v-model="loaderVersion"
					:disabled="busy || metadataLoading"
					required
				>
					<option v-if="!loaderVersions.length" value="">
						{{ metadataLoading ? 'Загружаем…' : 'Нет совместимой версии' }}
					</option>
					<option v-for="version in loaderVersions" :key="version.id" :value="version.id">
						{{ version.id }}{{ version.stable ? '' : ' · preview' }}
					</option></select
				><label class="checkbox-row"
					><input v-model="includeSnapshots" type="checkbox" />Показывать snapshots Minecraft</label
				>
				<div v-if="metadataError" class="banner error" role="alert">
					{{ metadataError
					}}<button
						type="button"
						class="icon-button"
						aria-label="Повторить загрузку версий"
						@click="loadMetadata"
					>
						<AppIcon name="refresh" :size="16" />
					</button>
				</div>
				<p class="field-help">
					RAM: {{ (settings.memory_mb / 1024).toFixed(1) }} ГБ · Свой профиль, отдельно от
					официальных сборок сообщества.
				</p>
				<button class="button primary full-width" :disabled="busy || metadataLoading">
					{{ busy ? 'Создаём…' : 'Создать сборку' }}<AppIcon name="plus" :size="18" />
				</button>
			</form>
		</dialog>
		<dialog
			ref="manageDialog"
			class="modal"
			:class="{ 'figma-pack-settings-dialog': manageAction === 'configure' }"
			aria-labelledby="manage-title"
		>
			<div
				v-if="manageAction === 'configure' && managed"
				class="figma-pack-settings"
				data-figma-node="3:11"
			>
				<header class="figma-pack-header">
					<img class="figma-pack-mark" :src="managed.icon || '/figma/8aecb.png'" alt="" />
					<span>{{ managed.name }}</span
					><b id="manage-title">Настройки</b>
					<button aria-label="Закрыть окно" :disabled="busy" @click="manageDialog?.close()">
						<img src="/figma/20469.svg" alt="" />
					</button>
				</header>
				<nav class="figma-pack-nav" aria-label="Разделы настроек сборки">
					<button
						:class="{ selected: packSettingsTab === 'main' }"
						@click="packSettingsTab = 'main'"
					>
						<img src="/figma/4b87e.svg" alt="" />Основные
					</button>
					<button
						:class="{ selected: packSettingsTab === 'installation' }"
						@click="packSettingsTab = 'installation'"
					>
						<img src="/figma/2de22.svg" alt="" />Установка
					</button>
					<button
						:class="{ selected: packSettingsTab === 'overrides' }"
						@click="packSettingsTab = 'overrides'"
					>
						<img src="/figma/f2524.svg" alt="" />Переопределения
					</button>
				</nav>
				<div
					class="figma-pack-body"
					:class="{ 'figma-pack-body-overrides': packSettingsTab === 'overrides' }"
				>
					<div v-if="packSettingsTab === 'main'" class="figma-pack-main" data-figma-node="3:11">
						<label class="figma-pack-field"
							><strong>Название</strong><input v-model="manageName" maxlength="80" :disabled="busy"
						/></label>
						<button
							v-if="manageName.trim() !== managed.name"
							class="figma-pack-button red"
							:disabled="busy || !manageName.trim()"
							@click="saveManagedName"
						>
							Сохранить название
						</button>
						<div class="figma-pack-section">
							<strong>Создание копии сборки</strong
							><button
								class="figma-pack-button"
								:disabled="busy"
								@click="showManage(managed, 'duplicate')"
							>
								<img src="/figma/165b7.svg" alt="" />Создать копию
							</button>
							<p>Создаёт копию сборки, включая миры, настройки, моды и другой контент.</p>
						</div>
						<div class="figma-pack-section">
							<strong>Удаление сборки</strong
							><button
								class="figma-pack-button red"
								:disabled="busy"
								@click="showManage(managed, 'delete')"
							>
								<img src="/figma/94db2.svg" alt="" />Удалить сборку
							</button>
							<p>Файлы сборки будут перемещены в локальную корзину лаунчера.</p>
						</div>
						<div class="figma-pack-icon">
							<strong>Иконка</strong>
							<div><img v-if="managed.icon" :src="managed.icon" alt="" /></div>
						</div>
					</div>
					<div
						v-else-if="packSettingsTab === 'installation'"
						class="figma-pack-install"
						data-figma-node="3:422"
					>
						<strong>Сведения об установке</strong>
						<div class="figma-pack-install-box">
							<span
								>Платформа<strong>{{ loaderNames[managed.loader] }}</strong></span
							><span
								>Версия игры<strong>{{ managed.game_version }}</strong></span
							><span
								>Версия {{ loaderNames[managed.loader]
								}}<strong>{{ managed.loader_version || '—' }}</strong></span
							>
						</div>
						<button
							class="figma-pack-button red"
							disabled
							title="Смена установки с сохранением контента пока не поддерживается"
						>
							Изменить — недоступно
						</button>
						<p>
							Не рекомендуется изменять установку после добавления контента. Это может привести к
							ошибкам.
						</p>
						<strong>Исправление сборки</strong>
						<button
							class="figma-pack-button"
							disabled
							title="Безопасное исправление установленной сборки пока не поддерживается"
						>
							Исправить — недоступно
						</button>
						<p>
							Переустановка зависимостей без затрагивания пользовательского контента требует
							отдельной проверки.
						</p>
					</div>
					<div v-else class="figma-pack-overrides">
						<section class="figma-pack-window-section" data-figma-node="3:463">
							<div class="figma-pack-window-intro">
								<div>
									<strong>Переопределение настроек окна</strong>
									<p>Настройка полноэкранного режима и разрешения при запуске для этой сборки.</p>
								</div>
								<button
									class="figma-settings-switch"
									:class="{ on: manageWindowEnabled }"
									:aria-pressed="manageWindowEnabled"
									aria-label="Переопределить размер окна для сборки"
									:disabled="busy"
									@click="manageWindowEnabled = !manageWindowEnabled"
								/>
							</div>
							<div class="figma-settings-window-row">
								<span
									><strong>Полноэкранный режим</strong
									><small>Запускать игру в полноэкранном режиме (через options.txt).</small></span
								><button
									class="figma-settings-switch"
									disabled
									title="Полноэкранный режим пока недоступен"
									aria-label="Полноэкранный режим пока недоступен"
								/>
							</div>
							<label class="figma-settings-window-row"
								><span><strong>Ширина</strong><small>Ширина окна игры при запуске.</small></span
								><input
									v-model.number="manageWindowWidth"
									type="number"
									min="320"
									max="7680"
									:disabled="busy || !manageWindowEnabled"
									aria-label="Ширина окна этой сборки"
							/></label>
							<label class="figma-settings-window-row"
								><span><strong>Высота</strong><small>Высота окна игры при запуске.</small></span
								><input
									v-model.number="manageWindowHeight"
									type="number"
									min="240"
									max="4320"
									:disabled="busy || !manageWindowEnabled"
									aria-label="Высота окна этой сборки"
							/></label>
							<div class="figma-pack-window-intro">
								<div>
									<strong>Переопределение выделенной памяти</strong>
									<p>Настройка выделения памяти отдельно для этой сборки.</p>
								</div>
								<span
									class="figma-settings-switch on"
									aria-label="Для этой сборки задана отдельная память"
								/>
							</div>
							<label class="figma-pack-memory"
								><small>512 MB</small
								><input
									v-model.number="manageMemory"
									type="range"
									min="512"
									max="32768"
									step="256"
									:disabled="busy"
								/><small>32.0 GB</small><output>{{ manageMemory }}</output></label
							>
						</section>
						<section class="figma-pack-java-override" data-figma-node="3:530">
							<strong>Переопределение расположения Java</strong
							><label class="figma-pack-field"
								><span>Путь Java для этой сборки</span
								><input
									v-model="manageJava"
									:disabled="busy"
									placeholder="Автоматический выбор Java" /></label
							><button class="figma-pack-button" :disabled="busy" @click="pickManagedJava">
								Выбрать Java вручную
							</button>
						</section>
						<div class="figma-pack-unsupported" data-figma-node="3:588,3:629">
							<strong>Аргументы Java и команды запуска</strong><span>Пока недоступно</span>
						</div>
						<button class="figma-pack-button red" :disabled="busy" @click="manageInstance">
							Сохранить
						</button>
					</div>
				</div>
				<div v-if="error" class="banner error figma-pack-error" role="alert">{{ error }}</div>
			</div>
			<template v-else>
				<button
					class="modal-close icon-button"
					aria-label="Закрыть окно"
					:disabled="busy"
					@click="manageDialog?.close()"
				>
					<AppIcon name="close" />
				</button>
				<h2 id="manage-title">
					{{
						manageAction === 'delete'
							? 'Удалить сборку?'
							: manageAction === 'duplicate'
								? 'Создать копию'
								: manageAction === 'configure'
									? 'Параметры сборки'
									: 'Название сборки'
					}}
				</h2>
				<p class="modal-intro">
					{{
						manageAction === 'delete'
							? `«${managed?.name}» исчезнет из библиотеки. Файлы сборки будут перемещены в локальную корзину лаунчера.`
							: manageAction === 'configure'
								? 'RAM и Java используются только этой сборкой. Версия Minecraft и загрузчик сохраняются.'
								: 'Имя помогает найти свой мир в библиотеке.'
					}}
				</p>
				<form @submit.prevent="manageInstance">
					<template v-if="manageAction === 'configure'"
						><label for="instance-memory"
							>Оперативная память · {{ (manageMemory / 1024).toFixed(1) }} ГБ</label
						><input
							id="instance-memory"
							v-model.number="manageMemory"
							type="range"
							min="1024"
							max="32768"
							step="512"
							:disabled="busy"
						/><label for="instance-java" class="field-label-spaced"
							>Путь к исполняемому файлу Java</label
						><input
							id="instance-java"
							v-model="manageJava"
							name="instance-java"
							autocomplete="off"
							spellcheck="false"
							placeholder="Автоматический выбор установленной Java"
							:disabled="busy"
						/>
						<button type="button" class="button secondary" @click="pickManagedJava">
							Выбрать Java вручную
						</button>
						<p class="field-help">
							Укажи путь к java, например /usr/lib/jvm/java-21-openjdk/bin/java. Лаунчер проверит
							версию перед установкой и запуском.
						</p></template
					>
					<template v-else-if="manageAction !== 'delete'"
						><label for="manage-name">Название</label
						><input
							id="manage-name"
							v-model="manageName"
							name="manage-name"
							maxlength="80"
							autocomplete="off"
							required
					/></template>
					<div v-if="error" class="banner error" role="alert">{{ error }}</div>
					<div class="dialog-actions">
						<button
							type="button"
							class="button subtle"
							:disabled="busy"
							@click="manageDialog?.close()"
						>
							Отмена</button
						><button
							class="button"
							:class="manageAction === 'delete' ? 'danger' : 'primary'"
							:disabled="busy"
						>
							{{ busy ? 'Сохраняем…' : manageAction === 'delete' ? 'Удалить сборку' : 'Сохранить' }}
						</button>
					</div>
				</form>
			</template>
		</dialog>
		<dialog ref="updateDialog" class="modal update-dialog" aria-labelledby="update-title">
			<button
				class="modal-close icon-button"
				aria-label="Закрыть обновление модов"
				:disabled="busy"
				@click="updateDialog?.close()"
			>
				<AppIcon name="close" />
			</button>
			<p class="eyebrow">СОВМЕСТИМЫЕ ОБНОВЛЕНИЯ</p>
			<h2 id="update-title">Обновить моды</h2>
			<p class="modal-intro">
				Подбираем версии для Minecraft {{ current?.game_version }} и
				{{ current ? loaderNames[current.loader] : '' }}. Предыдущее состояние можно восстановить.
			</p>
			<ul class="update-list">
				<li
					v-for="update in updates.filter((item) => selectedUpdateIds.includes(item.content_id))"
					:key="update.content_id"
				>
					<strong>{{ update.name }}</strong
					><span>{{ update.current_version }} → {{ update.next_version }}</span>
				</li>
			</ul>
			<div v-if="error" class="banner error" role="alert">{{ error }}</div>
			<div class="dialog-actions">
				<button class="button subtle" :disabled="busy" @click="updateDialog?.close()">Отмена</button
				><button
					class="button primary"
					:disabled="busy || !selectedUpdateIds.length"
					@click="applyUpdates"
				>
					Обновить {{ selectedUpdateIds.length }}
				</button>
			</div>
		</dialog>
		<dialog ref="removeModDialog" class="modal" aria-labelledby="remove-mod-title">
			<h2 id="remove-mod-title">Удалить мод?</h2>
			<p class="modal-intro">
				{{ removingMod?.name }} будет удалён из этой сборки. Чтобы временно отключить мод, используй
				переключатель в списке.
			</p>
			<div v-if="error" class="banner error" role="alert">{{ error }}</div>
			<div class="dialog-actions">
				<button class="button subtle" :disabled="busy" @click="removeModDialog?.close()">
					Отмена</button
				><button class="button danger" :disabled="busy" @click="removeMod">Удалить мод</button>
			</div>
		</dialog>
		<dialog ref="rollbackDialog" class="modal" aria-labelledby="rollback-title">
			<h2 id="rollback-title">Восстановить состояние?</h2>
			<p class="modal-intro">
				Вернём управляемые файлы к сохранённому состоянию до последнего обновления. Миры и личные
				файлы не заменяются.
			</p>
			<div v-if="error" class="banner error" role="alert">{{ error }}</div>
			<div class="dialog-actions">
				<button class="button subtle" :disabled="busy" @click="rollbackDialog?.close()">
					Отмена</button
				><button class="button primary" :disabled="busy" @click="rollback">Восстановить</button>
			</div>
		</dialog>
	</section>
</template>
