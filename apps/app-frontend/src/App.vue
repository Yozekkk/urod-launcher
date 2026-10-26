<script setup lang="ts">
import {
	computed,
	defineAsyncComponent,
	nextTick,
	onMounted,
	onUnmounted,
	reactive,
	ref,
	watch,
} from 'vue'
import { operationTitle, operationMessage } from './operation-labels'
import { invoke } from '@tauri-apps/api/core'
import { getVersion } from '@tauri-apps/api/app'
import { listen, type UnlistenFn } from '@tauri-apps/api/event'
import { enable, disable, isEnabled } from '@tauri-apps/plugin-autostart'
import AppIcon from './components/AppIcon.vue'
import AppNavbar from './components/AppNavbar.vue'
import JavaRuntimeDialog from './components/JavaRuntimeDialog.vue'
import FigmaGlobalSettings from './components/FigmaGlobalSettings.vue'
import SettingsSidebar from './components/SettingsSidebar.vue'
import ElyLoginDialog from './components/ElyLoginDialog.vue'
import LibraryPage from './components/LibraryPage.vue'
const SkinsPage = defineAsyncComponent(() => import('./components/SkinsPage.vue'))
import ContentPage from './components/ContentPage.vue'
import CustomizationPanel from './components/CustomizationPanel.vue'
import { applyCustomization, normalizeCustomization } from './customization'
import OperationPanel from './components/OperationPanel.vue'
import { operationPercent, useGameOperations } from './game-operations'
import { parseLauncherRoute, type Page } from './routes'
import { type Account, type Settings, type Skin, type Snapshot } from './models'
import { gameApi } from './game-api'
import { configureEditionInstaller } from './edition-installer'
import { formatUpdateBytes, useLauncherUpdates } from './launcher-updates'
import { renderProjectMarkdown } from './project-markdown'
import { messages } from './i18n'
import { parseJavaFailure } from './java-runtime'

const navigation = computed<{ id: Page; label: string; icon: string }[]>(() =>
	draft.value?.customization.language === 'en'
		? [
				{ id: 'library', label: 'Library', icon: 'folder' },
				{ id: 'content', label: 'Catalog', icon: 'game' },
				{ id: 'accounts', label: 'Accounts', icon: 'accounts' },
				{ id: 'skins', label: 'Skins', icon: 'accounts' },
				{ id: 'settings', label: 'Settings', icon: 'settings' },
			]
		: [
				{ id: 'library', label: 'Библиотека', icon: 'folder' },
				{ id: 'content', label: 'Каталог', icon: 'game' },
				{ id: 'accounts', label: messages.navigation.accounts, icon: 'accounts' },
				{ id: 'skins', label: 'Скины', icon: 'accounts' },
				{ id: 'settings', label: messages.navigation.settings, icon: 'settings' },
			],
)
const page = ref<Page>('library')
async function createFromSidebar() {
	setPage('library')
	await nextTick()
	window.document.getElementById('create-instance-button')?.click()
}
const operations = useGameOperations()
const launcherUpdates = useLauncherUpdates()
const launcherUpdateNotes = computed(() =>
	renderProjectMarkdown(launcherUpdates.candidate.value?.notes || ''),
)
const routeDetail = ref<string | null>(null)
const packLogSearch = ref('')
type PackLogLevel = 'errors' | 'warnings' | 'information'
const packLogFilter = ref<'all' | PackLogLevel>('all')
const packLogLines = ref<string[]>([])
const packLogError = ref('')
const packLogOpen = ref(false)
let packLogRequest = 0
let packLogTimer: ReturnType<typeof setInterval> | undefined
async function refreshPackLog() {
	const instanceId = page.value === 'library' ? routeDetail.value : null
	if (!instanceId) return
	const request = ++packLogRequest
	try {
		const text = await gameApi.gameLogTail(instanceId)
		if (request !== packLogRequest || routeDetail.value !== instanceId) return
		packLogLines.value = text.split(/\r?\n/).filter(Boolean).slice(-200)
		packLogError.value = ''
	} catch (reason) {
		if (request !== packLogRequest || routeDetail.value !== instanceId) return
		packLogError.value = String(reason)
	}
}
watch([page, routeDetail], () => {
	packLogRequest++
	packLogLines.value = []
	packLogError.value = ''
	packLogOpen.value = false
	if (page.value === 'library' && routeDetail.value) void refreshPackLog()
})
const visiblePackLogLines = computed(() => {
	const operationLines = operations.active.value
		.filter((job) => job.instance_id === routeDetail.value)
		.map((job) => ({
			text: operationMessage(job),
			level: (job.error ? 'errors' : 'information') as PackLogLevel,
		}))
	const gameLines = packLogLines.value.map((line) => ({
		text: line,
		level: (/\b(?:ERROR|FATAL)\b/i.test(line)
			? 'errors'
			: /\bWARN(?:ING)?\b/i.test(line)
				? 'warnings'
				: 'information') as PackLogLevel,
	}))
	const query = packLogSearch.value.toLocaleLowerCase()
	return [...operationLines, ...gameLines].filter(
		(line) =>
			(packLogFilter.value === 'all' || line.level === packLogFilter.value) &&
			line.text.toLocaleLowerCase().includes(query),
	)
})
const snapshot = ref<Snapshot | null>(null)
const draft = ref<Settings | null>(null)
let customizationTimer: ReturnType<typeof setTimeout> | undefined
watch(
	() => draft.value?.customization,
	(input) => {
		if (!input) return
		const value = normalizeCustomization(input)
		applyCustomization(value)
		if (
			!snapshot.value ||
			JSON.stringify(value) ===
				JSON.stringify(normalizeCustomization(snapshot.value.settings.customization))
		)
			return
		clearTimeout(customizationTimer)
		customizationTimer = setTimeout(() => {
			void invoke<Snapshot>('save_customization', { customization: value })
				.then((result) => {
					if (snapshot.value) snapshot.value.settings.customization = result.settings.customization
				})
				.catch((reason) => {
					error.value = String(reason)
				})
		}, 450)
	},
	{ deep: true },
)
const loading = ref(true)
const busy = ref(false)
const authenticating = ref(false)
const authProvider = ref<'microsoft' | 'ely_by'>('microsoft')
configureEditionInstaller(async ({ editionId }) => {
	await operations.track(
		await gameApi.installEdition(editionId, snapshot.value?.settings.release_channel || 'stable'),
	)
	notice.value = 'Установка сборки сообщества началась'
	return true
})
const error = ref('')
const notice = ref('')
let noticeTimer: ReturnType<typeof setTimeout> | undefined
watch(notice, (value) => {
	clearTimeout(noticeTimer)
	if (value)
		noticeTimer = setTimeout(() => {
			notice.value = ''
		}, 5000)
})
const online = ref(navigator.onLine)
const elyLoginDialog = ref<InstanceType<typeof ElyLoginDialog> | null>(null)
const authStage = ref<'browser' | 'minecraft'>('browser')
const eventCleanup: UnlistenFn[] = []
let mounted = true
const autostart = ref(false)
const autostartAvailable = ref(false)
const version = ref('…')
const license = ref('')
const fontLicenses = ref('')
const skins = reactive<Record<string, Skin | undefined>>({})
const skinLoading = reactive<Record<string, boolean>>({})
const generations = new Map<string, number>()
const offlineDialog = ref<HTMLDialogElement | null>(null)
const accountDialog = ref<HTMLDialogElement | null>(null)
const aboutDialog = ref<HTMLDialogElement | null>(null)
const nicknameInput = ref<HTMLInputElement | null>(null)
const nickname = ref('')
const nicknameError = ref('')
const selectedUuid = ref('')
const editNickname = ref('')
const deleting = ref(false)
const activeAccount = computed(() => snapshot.value?.accounts.find((account) => account.active))
const selectedAccount = computed(() =>
	snapshot.value?.accounts.find((account) => account.uuid === selectedUuid.value),
)
function serializeSettings(value: Settings | null | undefined) {
	return JSON.stringify(value, (_key, item) =>
		item && typeof item === 'object' && !Array.isArray(item)
			? Object.fromEntries(
					Object.entries(item).sort(([left], [right]) => left.localeCompare(right)),
				)
			: item,
	)
}
const settingsChanged = computed(
	() => serializeSettings(draft.value) !== serializeSettings(snapshot.value?.settings),
)
const usernameValid = (value: string) => /^[A-Za-z0-9_]{3,16}$/.test(value)
const accountType = (account: Account) =>
	account.kind === 'microsoft'
		? 'Microsoft'
		: account.kind === 'ely_by'
			? 'Ely.by'
			: skins[account.uuid]?.provider === 'ely_by'
				? 'Offline / Ely.by skin'
				: 'Offline'
const errorMessage = (reason: unknown) =>
	typeof reason === 'string'
		? parseJavaFailure(reason)?.message || reason
		: reason instanceof Error
			? reason.message
			: 'Не удалось выполнить действие. Попробуйте ещё раз.'

function showElyLogin() {
	authProvider.value = 'ely_by'
	void elyLoginDialog.value?.open()
}
function updateRoute() {
	const route = parseLauncherRoute(location.hash)
	const canonical = `#/${route.page}${route.detail ? `/${route.detail}` : ''}`
	if (location.hash !== canonical) window.history.replaceState(null, '', canonical)
	page.value = route.page
	routeDetail.value = route.detail
}
function focusMain() {
	document.getElementById('main-content')?.focus()
}
function setPage(value: Page) {
	location.hash = `/${value}`
	page.value = value
	error.value = ''
}
function connectivity() {
	online.value = navigator.onLine
	if (online.value)
		for (const account of snapshot.value?.accounts || [])
			if (skins[account.uuid]?.status === 'network_error') void loadSkin(account, true)
}
async function loadSkin(account: Account, force = false) {
	if (!force && (skins[account.uuid] || skinLoading[account.uuid])) return
	const generation = (generations.get(account.uuid) || 0) + 1
	generations.set(account.uuid, generation)
	skinLoading[account.uuid] = true
	try {
		const result = await invoke<Skin>('skin', { uuid: account.uuid, force })
		if (
			generations.get(account.uuid) === generation &&
			snapshot.value?.accounts.some((item) => item.uuid === account.uuid)
		)
			skins[account.uuid] = result
	} catch {
		if (generations.get(account.uuid) === generation)
			skins[account.uuid] = {
				provider: 'fallback',
				head: null,
				texture: null,
				status: 'network_error',
			}
	} finally {
		if (generations.get(account.uuid) === generation) skinLoading[account.uuid] = false
	}
}
function applySnapshot(result: Snapshot) {
	const pendingSettings = draft.value && settingsChanged.value ? draft.value : null
	snapshot.value = result
	if (!pendingSettings || serializeSettings(pendingSettings) === serializeSettings(result.settings))
		draft.value = {
			...result.settings,
			release_channel: result.settings.release_channel || 'stable',
			customization: normalizeCustomization(result.settings.customization),
		}
	for (const uuid of Object.keys(skins))
		if (!result.accounts.some((account) => account.uuid === uuid)) {
			delete skins[uuid]
			generations.set(uuid, (generations.get(uuid) || 0) + 1)
		}
	for (const account of result.accounts) void loadSkin(account)
}
async function initialize() {
	loading.value = true
	error.value = ''
	try {
		const result = await invoke<Snapshot>('snapshot')
		applySnapshot(result)
		if (result.settings.auto_updates)
			void launcherUpdates.check(result.settings.release_channel || 'stable')
	} catch (reason) {
		error.value = errorMessage(reason)
	} finally {
		loading.value = false
	}
}
async function mutate(command: string, args?: Record<string, unknown>, success?: string) {
	busy.value = true
	error.value = ''
	try {
		applySnapshot(await invoke<Snapshot>(command, args))
		if (success) notice.value = success
		return true
	} catch (reason) {
		error.value = errorMessage(reason)
		return false
	} finally {
		busy.value = false
	}
}
async function showOffline() {
	error.value = ''
	nickname.value = ''
	nicknameError.value = ''
	offlineDialog.value?.showModal()
	await nextTick()
	nicknameInput.value?.focus()
}
async function addOffline() {
	if (!usernameValid(nickname.value)) {
		nicknameError.value = 'От 3 до 16 символов: латинские буквы, цифры и _.'
		nicknameInput.value?.focus()
		return
	}
	if (await mutate('add_offline', { nickname: nickname.value }, 'Офлайн аккаунт добавлен'))
		offlineDialog.value?.close()
}
function showAccount(account: Account) {
	selectedUuid.value = account.uuid
	editNickname.value = account.nickname
	deleting.value = false
	error.value = ''
	accountDialog.value?.showModal()
}
async function renameAccount() {
	if (!selectedAccount.value || !usernameValid(editNickname.value)) {
		error.value = 'Никнейм: от 3 до 16 латинских букв, цифр и _.'
		return
	}
	const result = await mutate(
		'rename_offline',
		{ uuid: selectedUuid.value, nickname: editNickname.value },
		'Никнейм обновлён',
	)
	if (result) accountDialog.value?.close()
}
async function deleteAccount() {
	if (await mutate('remove_account', { uuid: selectedUuid.value }, 'Аккаунт удалён'))
		accountDialog.value?.close()
}
async function refreshAccount() {
	const uuid = selectedUuid.value
	if (await mutate('refresh_account', { uuid }, 'Данные профиля обновлены')) {
		const account = snapshot.value?.accounts.find((item) => item.uuid === uuid)
		if (account) void loadSkin(account, true)
	}
}
async function microsoftLogin() {
	authenticating.value = true
	authProvider.value = 'microsoft'
	authStage.value = 'browser'
	error.value = ''
	notice.value = ''
	try {
		const result = await invoke<Snapshot | null>('microsoft_login')
		if (result) {
			applySnapshot(result)
			notice.value = 'Microsoft аккаунт добавлен'
		}
	} catch (reason) {
		error.value = errorMessage(reason)
	} finally {
		authenticating.value = false
	}
}
async function cancelLogin() {
	try {
		await invoke('cancel_login')
	} catch (reason) {
		error.value = errorMessage(reason)
	}
}
async function toggleAutostart() {
	try {
		if (autostart.value) await disable()
		else await enable()
		autostart.value = await isEnabled()
	} catch (reason) {
		error.value = errorMessage(reason)
	}
}
async function pickDefaultJava() {
	try {
		const runtime = await gameApi.pickJava()
		if (runtime && draft.value) {
			draft.value.java_path = runtime.path
			notice.value = `Java ${runtime.major} · ${runtime.architecture} проверена`
		}
	} catch (reason) {
		error.value = errorMessage(reason)
	}
}
function updateDefaultJavaPath(major: number, path: string) {
	if (!draft.value) return
	draft.value.java_paths = { ...draft.value.java_paths, [major]: path }
}
function updateWindowSize(dimension: 'width' | 'height', value: string) {
	if (!draft.value) return
	if (!value) {
		draft.value.window_width = null
		draft.value.window_height = null
		return
	}
	if (dimension === 'width') {
		draft.value.window_width = Number(value)
		draft.value.window_height ??= 480
	} else {
		draft.value.window_height = Number(value)
		draft.value.window_width ??= 854
	}
}
async function saveSettings() {
	if (!draft.value) return
	const previous = snapshot.value?.settings
	if (await mutate('save_settings', { settings: { ...draft.value } }, 'Настройки сохранены')) {
		const settings = snapshot.value?.settings
		if (
			settings?.auto_updates &&
			(!previous?.auto_updates || previous.release_channel !== settings.release_channel)
		)
			void launcherUpdates.check(settings.release_channel)
	}
}
async function restart() {
	try {
		await invoke('restart_launcher')
	} catch (reason) {
		error.value = errorMessage(reason)
	}
}

onMounted(() => {
	window.history.replaceState(null, '', '#/library')
	updateRoute()
	packLogTimer = setInterval(() => {
		if (page.value === 'library' && routeDetail.value) void refreshPackLog()
	}, 3000)
	window.addEventListener('hashchange', updateRoute)
	window.addEventListener('online', connectivity)
	window.addEventListener('offline', connectivity)
	void listen<string>('urod-route', (event) => {
		const parsed = parseLauncherRoute(event.payload)
		location.hash = `/${parsed.page}${parsed.detail ? `/${parsed.detail}` : ''}`
		updateRoute()
	})
		.then((cleanup) => {
			if (mounted) eventCleanup.push(cleanup)
			else cleanup()
		})
		.catch(() => {})
	void listen<string>('auth-progress', (event) => {
		if (event.payload === 'browser' || event.payload === 'minecraft')
			authStage.value = event.payload
	})
		.then((cleanup) => {
			if (mounted) eventCleanup.push(cleanup)
			else cleanup()
		})
		.catch(() => {})
	void initialize()
	void fetch('/LICENSE.txt')
		.then((response) => response.text())
		.then((text) => {
			license.value = text
		})
		.catch(() => {
			license.value =
				'Не удалось загрузить текст. Лицензия находится в файле LICENSE исходного репозитория.'
		})
	void Promise.all(
		['/fonts/Poppins-OFL.txt', '/fonts/Manrope-OFL.txt', '/fonts/Unbounded-OFL.txt'].map((path) =>
			fetch(path).then((response) => response.text()),
		),
	)
		.then((texts) => {
			fontLicenses.value = texts.join('\n\n')
		})
		.catch(() => {})
	void getVersion()
		.then((value) => {
			version.value = value
		})
		.catch(() => {})
	void isEnabled()
		.then((value) => {
			autostart.value = value
			autostartAvailable.value = true
		})
		.catch(() => {})
})
onUnmounted(() => {
	clearTimeout(noticeTimer)
	clearTimeout(customizationTimer)
	clearInterval(packLogTimer)
	mounted = false
	for (const cleanup of eventCleanup) cleanup()
	window.removeEventListener('hashchange', updateRoute)
	window.removeEventListener('online', connectivity)
	window.removeEventListener('offline', connectivity)
})
</script>

<template>
	<div
		class="launcher"
		:class="{
			oled: snapshot?.settings.theme === 'oled',
			'no-motion': snapshot?.settings.reduced_motion || snapshot?.settings.animations === false,
			'no-blur': snapshot?.settings.blur === false,
			'is-settings': page === 'settings',
			'is-pack-detail': page === 'library' && !!routeDetail,
		}"
	>
		<a href="#main-content" class="skip-link" @click.prevent="focusMain">Перейти к содержимому</a>
		<AppNavbar
			:page="page"
			:navigation="navigation"
			:accounts="snapshot?.accounts || []"
			:active-account="activeAccount"
			:skins="skins"
			:version="version"
			@navigate="setPage"
			@create="createFromSidebar"
			@account="showAccount"
			@activate="
				(account) => mutate('set_active', { uuid: account.uuid }, 'Активный аккаунт изменён')
			"
			@about="aboutDialog?.showModal()"
		/>
		<JavaRuntimeDialog
			:operations="operations"
			:channel="snapshot?.settings.release_channel || 'stable'"
		/>
		<main id="main-content" tabindex="-1" class="main-area">
			<header class="topbar">
				<span class="breadcrumb"
					>UrodLauncher <span>/</span>
					{{ navigation.find((item) => item.id === page)?.label }}</span
				><span class="connection" :class="{ disconnected: !online }"
					><AppIcon name="wifi" :size="15" />{{ online ? 'В сети' : 'Нет подключения' }}</span
				>
			</header>
			<div v-if="!online" class="banner network" role="status">
				Нет подключения к интернету. Офлайн аккаунты и настройки доступны; скины загрузятся после
				подключения.
			</div>
			<div
				v-if="launcherUpdates.phase.value === 'available' && page !== 'settings'"
				class="banner launcher-update-banner"
				role="status"
			>
				<span>Доступно обновление UrodLauncher {{ launcherUpdates.candidate.value?.version }}</span>
				<div class="launcher-update-banner-actions">
					<button class="button secondary" @click="setPage('settings')">Подробнее</button>
					<button class="icon-button" aria-label="Напомнить позже" @click="launcherUpdates.later()">
						<AppIcon name="close" :size="16" />
					</button>
				</div>
			</div>
			<div
				v-if="error && !offlineDialog?.open && !accountDialog?.open"
				class="banner error"
				role="alert"
			>
				<span>{{ error }}</span
				><button class="icon-button" aria-label="Закрыть сообщение" @click="error = ''">
					<AppIcon name="close" :size="16" />
				</button>
			</div>
			<div
				v-if="operations.error.value && page !== 'library' && page !== 'content'"
				class="banner error"
				role="alert"
			>
				<span>{{ operations.error.value }}</span
				><button
					class="icon-button"
					aria-label="Закрыть ошибку установки"
					@click="operations.clearError()"
				>
					<AppIcon name="close" :size="16" />
				</button>
			</div>
			<div v-if="notice" class="toast" role="status">
				<AppIcon name="check" :size="17" />{{ notice
				}}<button class="icon-button" aria-label="Закрыть уведомление" @click="notice = ''">
					<AppIcon name="close" :size="15" />
				</button>
			</div>
			<div v-if="loading" class="loading-screen" role="status">
				<img src="/brand/logo.svg" width="72" height="72" alt="" /><span class="spinner" />
				<h1>Готовим UrodLauncher…</h1>
				<p>Загружаем аккаунты и настройки</p>
			</div>
			<div v-else-if="!snapshot" class="empty-state">
				<AppIcon name="info" :size="44" />
				<h1>Не удалось открыть лаунчер</h1>
				<p>Проверьте сообщение об ошибке и попробуйте снова.</p>
				<button class="button primary" @click="initialize">
					<AppIcon name="refresh" />Повторить
				</button>
			</div>
			<template v-else>
				<OperationPanel
					v-for="job in page === 'library' || page === 'content' ? [] : operations.active.value"
					:key="job.id"
					class="global-operation"
					:title="operationTitle(job.operation)"
					:message="operationMessage(job)"
					:progress="operationPercent(job)"
					:bytes-per-second="job.phase === 'downloading' ? job.bytes_per_second : undefined"
					:cancellable="job.cancellable"
					:cancelling="operations.cancelling.value === job.id"
					@cancel="operations.cancel(job.id)"
				/>
				<LibraryPage
					v-if="page === 'library'"
					:detail="routeDetail"
					:settings="snapshot.settings"
					:account-name="activeAccount?.nickname || null"
					:operations="operations"
					@open-log="packLogOpen = true"
					@open-global-settings="setPage('settings')"
				/>
				<ContentPage
					v-else-if="page === 'content'"
					:detail="routeDetail"
					:operations="operations"
					:release-channel="snapshot.settings.release_channel || 'stable'"
				/>
				<SkinsPage
					v-else-if="page === 'skins'"
					:accounts="snapshot?.accounts || []"
					:active-account="activeAccount"
					:reduced-motion="!!snapshot?.settings.reduced_motion"
					@applied="(account) => loadSkin(account, true)"
				/>
				<section v-else-if="page === 'accounts'" class="page accounts-page">
					<div class="page-heading">
						<div>
							<p class="eyebrow">ТВОЯ ИГРОВАЯ ИДЕНТИЧНОСТЬ</p>
							<h1>
								Аккаунты<span class="heading-count">{{ snapshot.accounts.length }}</span>
							</h1>
							<p class="page-intro">Один лаунчер. Все твои Minecraft-профили.</p>
						</div>
						<button
							class="button secondary"
							:disabled="busy || authenticating"
							@click="showOffline"
						>
							<AppIcon name="plus" />Офлайн аккаунт
						</button>
					</div>
					<div class="auth-panel">
						<div class="microsoft-mark" aria-hidden="true"><i /><i /><i /><i /></div>
						<div>
							<h2>Твой Minecraft, официально</h2>
							<p>Войди через Microsoft, чтобы добавить лицензионный аккаунт.</p>
						</div>
						<button
							class="button primary"
							:disabled="authenticating || busy || !online"
							@click="microsoftLogin"
						>
							Войти через Microsoft<AppIcon name="arrow" :size="18" />
						</button>
					</div>
					<div class="auth-panel ely-provider-panel">
						<div class="ely-provider-icon"><AppIcon name="shield" :size="22" /></div>
						<div>
							<h2>Аккаунт Ely.by</h2>
							<p>Игровой профиль и скины официальной системы Ely.by.</p>
						</div>
						<button
							class="button secondary"
							:disabled="authenticating || busy || !online"
							@click="showElyLogin"
						>
							Войти в Ely.by<AppIcon name="arrow" :size="17" />
						</button>
					</div>
					<div
						v-if="authenticating && authProvider === 'microsoft'"
						class="auth-progress"
						role="status"
					>
						<span class="spinner" />
						<div>
							<strong>{{
								authStage === 'minecraft'
									? 'Проверяем профиль Minecraft…'
									: 'Ожидаем вход через Microsoft…'
							}}</strong>
							<p>
								{{
									authStage === 'minecraft'
										? 'Вход завершён. Проверяем игровой профиль и сохраняем аккаунт…'
										: 'Заверши авторизацию в открывшемся браузере. Пароль остаётся у Microsoft.'
								}}
							</p>
						</div>
						<button
							class="button subtle"
							:disabled="authStage === 'minecraft'"
							@click="cancelLogin"
						>
							Отменить
						</button>
					</div>
					<div v-if="snapshot.accounts.length === 0" class="empty-state account-empty">
						<div class="empty-portrait">
							<span class="pixel-avatar large" /><span class="floating-plus">+</span>
						</div>
						<h2>Здесь начинается твоя история</h2>
						<p>
							Добавь аккаунт Microsoft или создай локальный<br />профиль — для него нужен только
							никнейм.
						</p>
						<button
							class="button secondary"
							:disabled="busy || authenticating"
							@click="showOffline"
						>
							<AppIcon name="plus" />Добавить офлайн аккаунт</button
						><small>Скины офлайн профилей — из Ely.by, без пароля.</small>
					</div>
					<div v-else class="account-grid">
						<button
							v-for="account in snapshot.accounts"
							:key="account.uuid"
							class="account-card"
							:class="{ selected: account.active }"
							@click="showAccount(account)"
						>
							<span class="account-card-top"
								><span class="account-kind">{{ accountType(account) }}</span
								><span v-if="account.active" class="active-badge"><span />Активный</span></span
							><span class="account-portrait"
								><img
									v-if="skins[account.uuid]?.head"
									:src="skins[account.uuid]?.head || ''"
									width="72"
									height="72"
									:alt="`Аватар ${account.nickname}`" /><span
									v-else
									class="pixel-avatar large" /><span
									v-if="skinLoading[account.uuid]"
									class="avatar-loader spinner" /></span
							><strong>{{ account.nickname }}</strong
							><span class="account-uuid">{{ account.uuid }}</span
							><span class="account-card-bottom"
								><span>{{
									skinLoading[account.uuid]
										? 'Загружаем скин…'
										: skins[account.uuid]?.status === 'network_error'
											? 'Скин недоступен · резервный аватар'
											: skins[account.uuid]?.provider === 'ely_by'
												? 'Скин Ely.by'
												: account.kind !== 'offline'
													? 'Профиль Minecraft'
													: 'Локальный профиль'
								}}</span
								><AppIcon name="arrow" :size="16"
							/></span>
						</button>
					</div>
					<div class="privacy-note">
						<AppIcon name="shield" :size="18" />
						<p>
							Аккаунты хранятся на этом устройстве. Офлайн профиль не подтверждает владение
							Minecraft.
						</p>
					</div>
				</section>
				<section v-else-if="draft" class="page settings-page figma-settings-page">
					<FigmaGlobalSettings
						:settings="draft"
						:data-dir="snapshot.data_dir"
						:busy="busy"
						:changed="settingsChanged"
						@close="setPage('library')"
						@save="saveSettings"
						@update-memory="draft.memory_mb = $event"
						@update-window-size="updateWindowSize"
						@update-java-path="updateDefaultJavaPath"
						@update-download-parallelism="draft.download_parallelism = $event"
					/>
					<details class="figma-settings-legacy">
						<summary>Другие настройки лаунчера</summary>
						<div class="page-heading">
							<div>
								<p class="eyebrow">НАСТРОЙ ПОД СЕБЯ</p>
								<h1>Настройки</h1>
								<p class="page-intro">Комфорт начинается с деталей.</p>
							</div>
							<button
								class="button primary"
								:disabled="busy || !settingsChanged"
								@click="saveSettings"
							>
								{{ busy ? 'Сохраняем…' : 'Сохранить изменения' }}<AppIcon name="check" :size="17" />
							</button>
						</div>
						<SettingsSidebar :language="draft?.customization.language || 'ru'" />
						<div class="settings-content">
							<div class="settings-section">
								<h2><AppIcon name="settings" />Лаунчер</h2>
								<div class="setting-row">
									<div>
										<label for="language">Язык интерфейса</label>
										<p>Язык интерфейса UrodLauncher</p>
									</div>
									<select id="language" v-model="draft.locale">
										<option value="ru">Русский</option>
										<option disabled>English — скоро</option>
									</select>
								</div>
								<div class="setting-row">
									<div>
										<strong>Запуск вместе с системой</strong>
										<p>
											{{
												autostartAvailable
													? 'Открывать UrodLauncher при входе в систему'
													: 'Недоступно в текущей среде'
											}}
										</p>
									</div>
									<button
										class="switch"
										:class="{ on: autostart }"
										role="switch"
										:aria-checked="autostart"
										aria-label="Запуск вместе с системой"
										:disabled="!autostartAvailable"
										@click="toggleAutostart"
									>
										<span />
									</button>
								</div>
							</div>
							<div class="settings-section launcher-updates-section">
								<h2><AppIcon name="refresh" />Обновления лаунчера</h2>
								<div class="setting-row">
									<div>
										<strong>Автоматическая проверка</strong>
										<p>
											Искать обновления при запуске. Установка начнётся только после подтверждения.
										</p>
									</div>
									<button
										class="switch"
										:class="{ on: draft.auto_updates }"
										role="switch"
										:aria-checked="draft.auto_updates"
										aria-label="Автоматическая проверка обновлений"
										@click="draft.auto_updates = !draft.auto_updates"
									>
										<span />
									</button>
								</div>
								<div class="setting-row">
									<div>
										<label for="release-channel">Канал версий</label>
										<p>Stable не получает Beta автоматически. Сохраните выбор перед проверкой.</p>
									</div>
									<select id="release-channel" v-model="draft.release_channel">
										<option value="stable">Stable · стабильный</option>
										<option value="beta">Beta · предварительный</option>
									</select>
								</div>
								<div class="setting-row launcher-update-check-row">
									<div>
										<strong>Текущая версия — {{ version }}</strong>
										<p>
											Канал: {{ snapshot.settings.release_channel === 'beta' ? 'Beta' : 'Stable' }}
										</p>
									</div>
									<button
										class="button secondary"
										:disabled="
											!online ||
											settingsChanged ||
											['checking', 'downloading', 'installing', 'restarting'].includes(
												launcherUpdates.phase.value,
											)
										"
										@click="launcherUpdates.check(snapshot.settings.release_channel)"
									>
										<AppIcon name="refresh" :size="17" />{{
											launcherUpdates.phase.value === 'checking'
												? 'Проверяем…'
												: 'Проверить обновления'
										}}
									</button>
								</div>
								<div
									v-if="settingsChanged || launcherUpdates.phase.value !== 'idle'"
									class="launcher-update-state"
									aria-live="polite"
								>
									<p v-if="settingsChanged" class="field-help">
										Сохраните настройки, чтобы применить выбранный канал обновлений.
									</p>
									<p v-if="launcherUpdates.phase.value === 'checking'" role="status">
										<span class="spinner" />Проверка обновлений…
									</p>
									<p v-else-if="launcherUpdates.phase.value === 'current'" role="status">
										<AppIcon name="check" :size="17" /> Установлена последняя доступная версия.
									</p>
									<div
										v-else-if="launcherUpdates.phase.value === 'available'"
										class="launcher-update-available"
									>
										<strong
											>Доступно обновление {{ launcherUpdates.candidate.value?.version }}</strong
										>
										<p>
											Размер: {{ formatUpdateBytes(launcherUpdates.candidate.value?.size_bytes) }} ·
											Подпись будет проверена перед установкой.
										</p>
										<details v-if="launcherUpdates.candidate.value?.notes">
											<summary>Что нового</summary>
											<!-- Release notes pass through the existing tag allowlist and DOMPurify. -->
											<!-- eslint-disable vue/no-v-html -->
											<div
												class="launcher-update-notes project-markdown"
												v-html="launcherUpdateNotes"
											/>
											<!-- eslint-enable vue/no-v-html -->
										</details>
										<div class="launcher-update-actions">
											<button class="button primary" @click="launcherUpdates.install()">
												Обновить
											</button>
											<button class="button subtle" @click="launcherUpdates.later()">Позже</button>
										</div>
									</div>
									<div
										v-else-if="
											['downloading', 'installing', 'restarting'].includes(
												launcherUpdates.phase.value,
											)
										"
										class="launcher-update-progress"
										role="status"
									>
										<strong>{{
											launcherUpdates.phase.value === 'downloading'
												? 'Загрузка обновления'
												: launcherUpdates.phase.value === 'installing'
													? 'Установка обновления'
													: 'Перезапуск лаунчера'
										}}</strong>
										<progress
											v-if="launcherUpdates.progress.value?.total_bytes"
											:max="launcherUpdates.progress.value.total_bytes"
											:value="launcherUpdates.progress.value.downloaded_bytes"
										/>
										<p v-if="launcherUpdates.phase.value === 'downloading'">
											{{ formatUpdateBytes(launcherUpdates.progress.value?.downloaded_bytes) }} /
											{{ formatUpdateBytes(launcherUpdates.progress.value?.total_bytes) }}
											<span v-if="launcherUpdates.progress.value?.bytes_per_second">
												·
												{{
													formatUpdateBytes(launcherUpdates.progress.value.bytes_per_second)
												}}/с</span
											>
										</p>
										<p v-else>
											{{ launcherUpdates.progress.value?.message || 'Пожалуйста, подождите…' }}
										</p>
									</div>
									<div
										v-else-if="launcherUpdates.phase.value === 'error'"
										class="banner error"
										role="alert"
									>
										{{ launcherUpdates.error.value }}
									</div>
								</div>
							</div>
							<div class="settings-section">
								<h2><AppIcon name="moon" />Внешний вид</h2>
								<div class="setting-row">
									<div>
										<label for="theme">Тема</label>
										<p>Тёмная палитра с тёплым акцентом</p>
									</div>
									<select id="theme" v-model="draft.theme">
										<option value="dark">Тёмная</option>
										<option value="oled">Глубокий чёрный</option>
									</select>
								</div>
								<div
									v-for="setting in [
										{
											key: 'animations' as const,
											label: 'Плавные анимации',
											detail: 'Мягкие переходы между состояниями',
										},
										{
											key: 'blur' as const,
											label: 'Прозрачность и размытие',
											detail: 'Лёгкая глубина полупрозрачных поверхностей',
										},
										{
											key: 'reduced_motion' as const,
											label: 'Уменьшить движение',
											detail: 'Отключить необязательные анимации',
										},
									]"
									:key="setting.key"
									class="setting-row"
								>
									<div>
										<strong>{{ setting.label }}</strong>
										<p>{{ setting.detail }}</p>
									</div>
									<button
										class="switch"
										:class="{ on: draft[setting.key] }"
										role="switch"
										:aria-checked="draft[setting.key]"
										:aria-label="setting.label"
										@click="draft[setting.key] = !draft[setting.key]"
									>
										<span />
									</button>
								</div>
							</div>
							<div class="settings-section">
								<h2><AppIcon name="game" />Minecraft</h2>
								<div class="future-note">
									RAM и путь к Java используются при создании своих сборок. Настройки существующих
									профилей сохраняются отдельно.
								</div>
								<div class="setting-row memory-row">
									<div>
										<label for="memory">Оперативная память</label>
										<p>Объём RAM по умолчанию для новых сборок</p>
									</div>
									<div class="memory-control">
										<output for="memory">{{ (draft.memory_mb / 1024).toFixed(1) }} ГБ</output
										><input
											id="memory"
											v-model.number="draft.memory_mb"
											type="range"
											min="1024"
											max="32768"
											step="512"
										/>
									</div>
								</div>
								<div class="setting-row stacked">
									<label for="java">Путь к Java</label
									><input
										id="java"
										v-model="draft.java_path"
										name="java-path"
										autocomplete="off"
										spellcheck="false"
										placeholder="Автоматически после установки сборки"
									/>
									<button class="button secondary" @click="pickDefaultJava">
										Выбрать Java вручную
									</button>
								</div>
								<div class="setting-row stacked">
									<label for="game-directory">Папка игры · будущая настройка</label
									><input
										id="game-directory"
										:value="`${snapshot.data_dir}/instances`"
										readonly
										name="game-directory"
										autocomplete="off"
										spellcheck="false"
										aria-describedby="game-directory-help"
									/>
									<p id="game-directory-help" class="field-help">
										Сборки хранятся в отдельной папке UrodLauncher. Выбор другой папки появится
										вместе с безопасным переносом данных.
									</p>
								</div>
							</div>
							<div class="settings-section compact-section">
								<h2><AppIcon name="folder" />Данные приложения</h2>
								<p class="data-directory">{{ snapshot.data_dir }}</p>
								<div class="settings-bottom">
									<button class="button subtle" @click="aboutDialog?.showModal()">
										<AppIcon name="info" :size="17" />О приложении и лицензии</button
									><button
										class="button subtle"
										:disabled="busy || authenticating"
										@click="restart"
									>
										<AppIcon name="refresh" :size="17" />Перезапустить
									</button>
								</div>
							</div>
							<CustomizationPanel v-if="draft" v-model="draft.customization" />
						</div>
					</details>
				</section>
			</template>
		</main>
		<aside
			v-if="page === 'library' || page === 'content' || page === 'skins'"
			class="figma-news"
			:class="{
				'figma-pack-log': page === 'library' && !!routeDetail,
				'compact-open': packLogOpen,
			}"
			:aria-label="page === 'library' && routeDetail ? 'Журнал сборки' : 'Новости сообщества'"
		>
			<template v-if="page === 'library' && routeDetail">
				<div class="figma-pack-log-controls">
					<input
						v-model="packLogSearch"
						type="search"
						aria-label="Поиск по журналу"
						placeholder="Поиск"
					/><span title="Журнал игры и операций сборки">Игра</span>
					<button class="pack-log-close" aria-label="Закрыть журнал" @click="packLogOpen = false">
						Закрыть
					</button>
				</div>
				<div class="figma-pack-log-filters">
					<button :class="{ selected: packLogFilter === 'all' }" @click="packLogFilter = 'all'">
						Всё</button
					><button
						:class="{ selected: packLogFilter === 'errors' }"
						@click="packLogFilter = 'errors'"
					>
						Ошибки</button
					><button
						:class="{ selected: packLogFilter === 'warnings' }"
						@click="packLogFilter = 'warnings'"
					>
						Предупреждения</button
					><button
						:class="{ selected: packLogFilter === 'information' }"
						@click="packLogFilter = 'information'"
					>
						Информация
					</button>
				</div>
				<div class="figma-pack-log-lines" role="log" aria-label="Журнал сборки">
					<p v-if="packLogError" role="alert">{{ packLogError }}</p>
					<p v-for="(line, index) in visiblePackLogLines" :key="index">&gt; {{ line.text }}</p>
					<p v-if="!packLogError && !visiblePackLogLines.length">&gt;_консоль</p>
				</div>
			</template>
			<template v-else>
				<h2>НОВОСТИ</h2>
				<a href="https://t.me/YrodNews" target="_blank" rel="noopener noreferrer"
					>Открыть официальный канал новостей</a
				>
			</template>
		</aside>
		<dialog
			ref="offlineDialog"
			class="modal offline-modal"
			aria-labelledby="offline-title"
			@close="nicknameError = ''"
		>
			<button
				class="modal-close icon-button"
				aria-label="Закрыть окно"
				:disabled="busy"
				@click="offlineDialog?.close()"
			>
				<AppIcon name="close" />
			</button>
			<div class="modal-symbol"><AppIcon name="accounts" :size="28" /></div>
			<p class="eyebrow">ТОЛЬКО ТВОЙ НИКНЕЙМ</p>
			<h2 id="offline-title">Офлайн аккаунт</h2>
			<p class="modal-intro">
				Создай локальный Minecraft-профиль.<br />Никаких паролей — просто будь собой.
			</p>
			<form @submit.prevent="addOffline">
				<label for="nickname">Никнейм</label
				><input
					id="nickname"
					ref="nicknameInput"
					v-model="nickname"
					name="nickname"
					autocomplete="off"
					spellcheck="false"
					maxlength="16"
					placeholder="Например, UrodPlayer"
					:aria-invalid="!!nicknameError"
					aria-describedby="nickname-help"
					@input="nicknameError = ''"
				/>
				<p
					id="nickname-help"
					class="field-help"
					:class="{ 'field-error': nicknameError }"
					aria-live="polite"
				>
					{{ nicknameError || '3–16 символов: латинские буквы, цифры и _' }}
				</p>
				<div v-if="error" class="banner error" role="alert">{{ error }}</div>
				<div class="skin-note">
					<AppIcon name="shield" :size="20" />
					<p>Если у этого никнейма есть скин Ely.by,<br />мы покажем его автоматически.</p>
				</div>
				<button class="button primary full-width" type="submit" :disabled="busy">
					{{ busy ? 'Создаём аккаунт…' : 'Добавить аккаунт' }}<AppIcon name="plus" :size="18" />
				</button>
			</form>
		</dialog>
		<dialog
			ref="accountDialog"
			class="modal account-modal"
			aria-labelledby="account-title"
			@close="deleting = false"
		>
			<template v-if="selectedAccount">
				<button
					class="modal-close icon-button"
					aria-label="Закрыть окно"
					:disabled="busy"
					@click="accountDialog?.close()"
				>
					<AppIcon name="close" />
				</button>
				<div class="profile-hero">
					<img
						v-if="skins[selectedUuid]?.head"
						:src="skins[selectedUuid]?.head || ''"
						width="88"
						height="88"
						alt="Аватар аккаунта"
					/><span v-else class="pixel-avatar large" /><span
						v-if="selectedAccount.active"
						class="active-badge"
						><span />Активный аккаунт</span
					>
				</div>
				<h2 id="account-title">{{ selectedAccount.nickname }}</h2>
				<p class="profile-type">{{ accountType(selectedAccount) }}</p>
				<div class="uuid-field">
					<span>UUID</span><code>{{ selectedAccount.uuid }}</code>
				</div>
				<div v-if="skins[selectedUuid]?.texture" class="skin-preview">
					<img
						:src="skins[selectedUuid]?.texture || ''"
						width="192"
						height="192"
						alt="Развёртка скина Minecraft"
					/>
					<div>
						<strong>{{
							skins[selectedUuid]?.provider === 'fallback' ? 'Стандартный скин' : 'Твой скин'
						}}</strong>
						<p>
							{{
								skins[selectedUuid]?.provider === 'ely_by'
									? 'Ely.by Skin System'
									: skins[selectedUuid]?.provider === 'fallback'
										? 'Steve · локальный резервный скин'
										: 'Minecraft / Mojang'
							}}
						</p>
					</div>
				</div>
				<p v-else class="skin-fallback">
					{{
						skinLoading[selectedUuid]
							? 'Загружаем скин…'
							: 'Скин не найден или сеть недоступна. Используем резервный аватар.'
					}}
				</p>
				<form
					v-if="selectedAccount.kind === 'offline'"
					class="rename-form"
					@submit.prevent="renameAccount"
				>
					<label for="rename">Никнейм</label>
					<div class="rename-row">
						<input
							id="rename"
							v-model="editNickname"
							name="rename"
							autocomplete="off"
							spellcheck="false"
							maxlength="16"
						/><button
							class="button secondary"
							:disabled="busy || editNickname === selectedAccount.nickname"
						>
							Сохранить
						</button>
					</div>
					<p class="field-help">При смене никнейма изменится и офлайн UUID.</p>
				</form>
				<div v-if="error" class="banner error" role="alert">{{ error }}</div>
				<div class="profile-actions">
					<button
						v-if="!selectedAccount.active"
						class="button primary full-width"
						:disabled="busy"
						@click="mutate('set_active', { uuid: selectedUuid }, 'Активный аккаунт изменён')"
					>
						<AppIcon name="check" :size="17" />Сделать активным</button
					><button
						v-if="selectedAccount.kind !== 'offline'"
						class="button secondary full-width"
						:disabled="busy || !online"
						@click="refreshAccount"
					>
						<AppIcon name="refresh" :size="17" />Обновить данные профиля</button
					><button
						v-else
						class="button subtle full-width"
						:disabled="skinLoading[selectedUuid] || !online"
						@click="loadSkin(selectedAccount, true)"
					>
						<AppIcon name="refresh" :size="17" />Обновить скин
					</button>
					<div v-if="deleting" class="delete-confirm">
						<p>
							Удалить аккаунт из лаунчера?<br /><small>Это не удалит сам Minecraft-аккаунт.</small>
						</p>
						<div>
							<button class="button subtle" :disabled="busy" @click="deleting = false">
								Отмена</button
							><button class="button danger" :disabled="busy" @click="deleteAccount">
								{{ busy ? 'Удаляем…' : 'Удалить' }}
							</button>
						</div>
					</div>
					<button
						v-else
						class="button delete-button full-width"
						:disabled="busy"
						@click="deleting = true"
					>
						<AppIcon name="trash" :size="16" />Удалить аккаунт
					</button>
				</div>
			</template>
		</dialog>
		<dialog ref="aboutDialog" class="modal about-modal" aria-labelledby="about-title">
			<button
				class="modal-close icon-button"
				aria-label="Закрыть окно"
				@click="aboutDialog?.close()"
			>
				<AppIcon name="close" /></button
			><img
				class="about-logo"
				src="/brand/logo.svg"
				width="72"
				height="72"
				alt="Значок UrodLauncher"
			/>
			<h2 id="about-title">UrodLauncher</h2>
			<p class="profile-type">Версия {{ version }} · Предварительная</p>
			<p>Minecraft-лаунчер Города Уродов.</p>
			<p>
				Основан на открытом desktop-коде Modrinth App (Theseus). Copyright © Modrinth и участники
				проекта. Изменения © 2026 UrodLauncher. Распространяется по GNU GPL v3.
			</p>
			<details>
				<summary>Текст лицензии GNU GPL v3</summary>
				<pre class="license-frame">{{ license || 'Загружаем лицензию…' }}</pre>
			</details>
			<details>
				<summary>Лицензии шрифтов Poppins, Manrope и Unbounded</summary>
				<pre class="license-frame">{{ fontLicenses || 'Загружаем лицензии…' }}</pre>
			</details>
			<p class="legal-note">
				Не является официальным продуктом Minecraft. Не одобрено и не связано с Mojang или
				Microsoft.
			</p>
		</dialog>
		<ElyLoginDialog ref="elyLoginDialog" @saved="applySnapshot" @busy="authenticating = $event" />
	</div>
</template>
