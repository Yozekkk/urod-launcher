<script setup lang="ts">
import { computed, onMounted, shallowRef, watch } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import type { Account, Skin } from '../models'
import SkinPreview from './SkinPreview.vue'

export type SavedSkin = {
	id: string
	name: string
	slim: boolean
	active_for: string[]
	preview: Skin | null
}
const props = defineProps<{
	accounts: Account[]
	activeAccount: Account | null | undefined
	reducedMotion: boolean
}>()
const emit = defineEmits<{ applied: [account: Account] }>()
const items = shallowRef<SavedSkin[]>([])
const selectedId = shallowRef('')
const accountId = shallowRef('')
const name = shallowRef('')
const slim = shallowRef(false)
const busy = shallowRef(false)
const error = shallowRef('')
const message = shallowRef('')
const editName = shallowRef('')
const selected = computed(() => items.value.find((item) => item.id === selectedId.value))
const account = computed(() => props.accounts.find((item) => item.uuid === accountId.value))
const active = computed(() => selected.value?.active_for.includes(accountId.value) ?? false)
watch(
	() => props.activeAccount?.uuid,
	(value) => {
		if (value && !accountId.value) accountId.value = value
	},
	{ immediate: true },
)
async function load() {
	try {
		items.value = await invoke<SavedSkin[]>('skin_library_list')
		if (!selectedId.value) {
			selectedId.value = items.value[0]?.id || ''
			editName.value = items.value[0]?.name || ''
		}
	} catch (reason) {
		error.value = String(reason)
	}
}
onMounted(() => {
	void load()
})
async function run(command: string, args: Record<string, unknown>, success: string) {
	busy.value = true
	error.value = ''
	message.value = ''
	try {
		items.value = await invoke<SavedSkin[]>(command, args)
		message.value = success
		if (account.value && (command === 'skin_library_apply' || command === 'skin_library_reset'))
			emit('applied', account.value)
	} catch (reason) {
		error.value = String(reason)
	} finally {
		busy.value = false
	}
}
async function upload(event: Event) {
	const input = event.target as HTMLInputElement
	const file = input.files?.[0]
	if (!file) return
	if (file.size > 1_048_576 || file.type !== 'image/png') {
		error.value = 'Выберите PNG размером до 1 МБ'
		return
	}
	const reader = new FileReader()
	reader.onload = () => {
		const data = reader.result
		if (typeof data !== 'string') return
		void run(
			'skin_library_save',
			{ name: name.value.trim() || file.name.replace(/\.png$/i, ''), slim: slim.value, data },
			'Скин сохранён в библиотеке',
		).then(() => {
			selectedId.value = items.value[0]?.id || ''
		})
	}
	reader.onerror = () => {
		error.value = 'Не удалось прочитать файл'
	}
	reader.readAsDataURL(file)
	input.value = ''
}
function selectSkin(item: SavedSkin) {
	selectedId.value = item.id
	editName.value = item.name
}
function remove() {
	if (!selected.value || !window.confirm(`Удалить «${selected.value.name}» из библиотеки?`)) return
	const id = selected.value.id
	void run('skin_library_remove', { id }, 'Скин удалён').then(() => {
		selectedId.value = items.value[0]?.id || ''
	})
}
function rename() {
	if (!selected.value || !editName.value.trim()) return
	void run(
		'skin_library_rename',
		{ id: selected.value.id, name: editName.value },
		'Название изменено',
	)
}
function apply() {
	if (selected.value && account.value)
		void run(
			'skin_library_apply',
			{ id: selected.value.id, account: account.value.uuid },
			account.value.kind === 'microsoft'
				? 'Скин проверен в профиле Minecraft'
				: 'Локальный скин выбран',
		)
}
function reset() {
	if (account.value)
		void run('skin_library_reset', { account: account.value.uuid }, 'Стандартный скин восстановлен')
}
</script>
<template>
	<section class="page skins-page">
		<div class="skins-heading">
			<div>
				<p class="eyebrow">БИБЛИОТЕКА СКИНОВ</p>
				<h1>Скины</h1>
				<p>Сохраняйте варианты и переключайте их для выбранного аккаунта.</p>
			</div>
			<label class="button primary skin-upload"
				>Загрузить PNG<input
					type="file"
					accept="image/png"
					aria-label="Загрузить PNG скин"
					:disabled="busy"
					@change="upload"
			/></label>
		</div>
		<div class="skin-layout">
			<aside class="skin-list" aria-label="Сохранённые скины">
				<button
					v-for="item in items"
					:key="item.id"
					class="skin-item"
					:class="{ selected: selectedId === item.id }"
					@click="selectSkin(item)"
				>
					<img
						v-if="item.preview?.head"
						:src="item.preview.head"
						width="42"
						height="42"
						alt=""
					/><span
						>{{ item.name }}<small>{{ item.slim ? 'Alex / Slim' : 'Steve / Classic' }}</small></span
					><b v-if="item.active_for.includes(accountId)" title="Активный">●</b>
				</button>
				<p v-if="!items.length" class="skin-empty">Загрузите первый PNG скин.</p>
			</aside>
			<div class="skin-detail">
				<SkinPreview
					:texture="selected?.preview?.texture || null"
					:slim="selected?.slim || false"
					:reduced-motion="reducedMotion"
				/>
				<div v-if="selected" class="skin-controls">
					<label>Название<input v-model="editName" maxlength="48" autocomplete="off" /></label
					><button class="button secondary" :disabled="busy || !editName.trim()" @click="rename">
						Переименовать</button
					><button class="button secondary" :disabled="busy" @click="remove">Удалить</button>
				</div>
			</div>
			<div class="skin-actions">
				<h2>Применить скин</h2>
				<label for="skin-account">Аккаунт</label
				><select id="skin-account" v-model="accountId">
					<option v-for="item in accounts" :key="item.uuid" :value="item.uuid">
						{{ item.nickname }} ·
						{{
							item.kind === 'microsoft'
								? 'Microsoft'
								: item.kind === 'ely_by'
									? 'Ely.by'
									: 'Offline'
						}}
					</option>
				</select>
				<p v-if="account?.kind === 'microsoft'">
					Скин будет загружен в официальный профиль Minecraft. Результат проверяется после отправки.
				</p>
				<p v-else-if="account?.kind === 'offline'">
					Выбор действует локально в лаунчере. Лицензионные серверы не меняются.
				</p>
				<p v-else-if="account?.kind === 'ely_by'">
					Для смены скина Ely.by используйте кабинет провайдера.
				</p>
				<p v-else>Добавьте аккаунт, чтобы применить скин.</p>
				<button
					class="button primary"
					:disabled="busy || !selected || !account || account.kind === 'ely_by' || active"
					@click="apply"
				>
					{{ busy ? 'Проверяем…' : active ? 'Активный скин' : 'Применить' }}</button
				><button
					class="button secondary"
					:disabled="busy || !account || account.kind === 'ely_by'"
					@click="reset"
				>
					Восстановить стандартный
				</button>
				<div class="skin-import">
					<label
						>Название нового скина<input
							v-model="name"
							maxlength="48"
							autocomplete="off"
							placeholder="Название…" /></label
					><label class="skin-model"><input v-model="slim" type="checkbox" /> Alex / Slim</label>
				</div>
				<p v-if="message" role="status" aria-live="polite" class="skin-success">{{ message }}</p>
				<p v-if="error" role="alert" class="skin-error">{{ error }}</p>
			</div>
		</div>
	</section>
</template>
<style scoped>
.skins-page {
	padding: 28px 36px 48px;
}
.skins-heading {
	display: flex;
	justify-content: space-between;
	align-items: center;
	gap: 20px;
	margin-bottom: 24px;
}
.skins-heading h1 {
	font-size: 32px;
	margin: 4px 0;
}
.skins-heading p {
	color: var(--muted);
}
.eyebrow {
	font-size: 11px;
	letter-spacing: 0.16em;
	color: #ff706c !important;
}
.skin-upload {
	position: relative;
	overflow: hidden;
	white-space: nowrap;
}
.skin-upload input {
	position: absolute;
	inset: 0;
	opacity: 0;
	cursor: pointer;
}
.skin-layout {
	display: grid;
	grid-template-columns: minmax(180px, 230px) minmax(260px, 1fr) minmax(210px, 280px);
	gap: 18px;
	align-items: start;
}
.skin-list,
.skin-actions {
	background: var(--surface);
	border: 1px solid var(--line);
	border-radius: 18px;
	padding: 14px;
}
.skin-list {
	min-height: 340px;
	max-height: 580px;
	overflow: auto;
}
.skin-item {
	display: flex;
	align-items: center;
	gap: 12px;
	width: 100%;
	border: 1px solid transparent;
	border-radius: 12px;
	background: transparent;
	padding: 8px;
	text-align: left;
}
.skin-item:hover,
.skin-item.selected {
	background: #a62d3930;
	border-color: #e3475b55;
}
.skin-item img {
	image-rendering: pixelated;
	border-radius: 6px;
}
.skin-item span {
	min-width: 0;
	overflow: hidden;
	text-overflow: ellipsis;
	white-space: nowrap;
}
.skin-item small {
	display: block;
	color: var(--muted);
	font-size: 11px;
}
.skin-item b {
	margin-left: auto;
	color: #ff645d;
}
.skin-empty {
	color: var(--muted);
	padding: 12px;
}
.skin-detail {
	min-width: 0;
}
.skin-controls {
	display: flex;
	flex-wrap: wrap;
	gap: 8px;
	align-items: end;
	margin-top: 14px;
}
.skin-controls label {
	flex: 1;
	min-width: 160px;
}
.skin-actions {
	display: grid;
	gap: 12px;
}
.skin-actions h2 {
	font-size: 18px;
	margin: 0;
}
.skin-actions p {
	color: var(--muted);
	font-size: 12px;
	line-height: 1.5;
}
.skin-actions select,
.skin-actions input,
.skin-controls input {
	width: 100%;
	border: 1px solid var(--line);
	border-radius: 9px;
	background: var(--control);
	color: var(--foreground);
	padding: 9px;
}
.skin-model {
	display: flex;
	gap: 8px;
	align-items: center;
}
.skin-model input {
	width: auto;
}
.skin-import {
	border-top: 1px solid var(--line);
	margin-top: 8px;
	padding-top: 12px;
	display: grid;
	gap: 10px;
}
.skin-success {
	color: #9be5b0 !important;
}
.skin-error {
	color: #ff8a88 !important;
}
@media (max-width: 1080px) {
	.skin-layout {
		grid-template-columns: 190px 1fr;
	}
	.skin-actions {
		grid-column: 1/-1;
	}
}
@media (max-width: 720px) {
	.skins-page {
		padding: 20px;
	}
	.skin-layout {
		grid-template-columns: 1fr;
	}
	.skin-actions {
		grid-column: auto;
	}
	.skin-list {
		min-height: 0;
		max-height: 180px;
	}
}
</style>
