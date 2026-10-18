<script setup lang="ts">
import { computed, onMounted, ref, shallowRef, watch } from 'vue'
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
const editSlim = shallowRef(false)
const pendingTexture = shallowRef<string | null>(null)
const addDialog = ref<HTMLDialogElement | null>(null)
const editDialog = ref<HTMLDialogElement | null>(null)
const addOpen = shallowRef(false)
const editOpen = shallowRef(false)
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
		pendingTexture.value = data
		if (!name.value.trim()) name.value = file.name.replace(/\.png$/i, '')
		addOpen.value = true
		addDialog.value?.showModal()
	}
	reader.onerror = () => {
		error.value = 'Не удалось прочитать файл'
	}
	reader.readAsDataURL(file)
	input.value = ''
}
function prepareAdd() {
	error.value = ''
	name.value = ''
	slim.value = false
	pendingTexture.value = null
}
async function saveNew() {
	if (!pendingTexture.value) return
	await run(
		'skin_library_save',
		{ name: name.value.trim(), slim: slim.value, data: pendingTexture.value },
		'Скин сохранён в библиотеке',
	)
	if (error.value) return
	selectedId.value = items.value[0]?.id || ''
	addDialog.value?.close()
	addOpen.value = false
}
function showEdit() {
	if (!selected.value) return
	error.value = ''
	editName.value = selected.value.name
	editSlim.value = selected.value.slim
	editOpen.value = true
	editDialog.value?.showModal()
}
function selectSkin(item: SavedSkin) {
	selectedId.value = item.id
	editName.value = item.name
	editSlim.value = item.slim
}
function remove() {
	if (!selected.value || !window.confirm(`Удалить «${selected.value.name}» из библиотеки?`)) return
	const id = selected.value.id
	void run('skin_library_remove', { id }, 'Скин удалён').then(() => {
		selectedId.value = items.value[0]?.id || ''
	})
}
async function rename() {
	if (!selected.value || !editName.value.trim()) return
	await run(
		'skin_library_rename',
		{ id: selected.value.id, name: editName.value, slim: editSlim.value },
		'Скин изменён',
	)
	if (!error.value) {
		editDialog.value?.close()
		editOpen.value = false
	}
}
async function promptRename() {
	if (!selected.value) return
	const next = window.prompt('Название скина', selected.value.name)
	if (next === null || next.trim() === selected.value.name) return
	await run(
		'skin_library_rename',
		{ id: selected.value.id, name: next, slim: null },
		'Скин переименован',
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
			<h1>Выбор скина</h1>
			<h2>⌃&nbsp; Сохраненные скины</h2>
		</div>
		<p v-if="error && !addOpen && !editOpen" role="alert" class="skin-page-error">{{ error }}</p>
		<div class="skin-layout">
			<aside class="skin-list" aria-label="Сохранённые скины">
				<label class="skin-add-tile" :aria-disabled="busy" @click="prepareAdd">
					<img src="/figma/55119.svg" width="25" height="25" alt="" />
					<strong>Добавить скин</strong>
					<small>Выберите PNG</small>
					<input
						type="file"
						accept="image/png"
						aria-label="Добавить скин PNG"
						:disabled="busy"
						@change="upload"
					/>
				</label>
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
			</aside>
			<div class="skin-detail">
				<strong v-if="selected" class="skin-selected-name">{{ selected.name }}</strong>
				<SkinPreview
					:texture="selected?.preview?.texture || null"
					:slim="selected?.slim || false"
					:reduced-motion="reducedMotion"
				/>
				<div v-if="selected" class="skin-controls">
					<button class="button secondary skin-edit-button" :disabled="busy" @click="showEdit">
						Изменить
					</button>
				</div>
			</div>
			<div v-if="selected" class="skin-actions">
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
				<button class="button secondary" :disabled="busy || !selected" @click="remove">
					Удалить выбранный скин
				</button>
				<button class="button secondary" :disabled="busy || !selected" @click="promptRename">
					Переименовать
				</button>
				<p v-if="message" role="status" aria-live="polite" class="skin-success">{{ message }}</p>
			</div>
		</div>
		<dialog
			ref="addDialog"
			class="skin-dialog"
			aria-labelledby="skin-add-title"
			tabindex="-1"
			@close="addOpen = false"
		>
			<header>
				<h2 id="skin-add-title">Добавить скин</h2>
				<button aria-label="Закрыть" @click="addDialog?.close()">×</button>
			</header>
			<div class="skin-dialog-body">
				<SkinPreview
					v-if="addOpen"
					:texture="pendingTexture"
					:slim="slim"
					:reduced-motion="reducedMotion"
				/>
				<p class="skin-rotate-hint">⟷ Перетащите, чтобы повернуть</p>
				<div class="skin-dialog-fields">
					<fieldset>
						<legend>Стиль рук</legend>
						<label><input v-model="slim" type="radio" :value="false" /> Стив</label>
						<label><input v-model="slim" type="radio" :value="true" /> Алекс</label>
					</fieldset>
					<label
						>Плащ<button disabled title="Загрузка плащей пока недоступна">
							<span>×</span><span>НЕТ</span>
						</button></label
					>
				</div>
			</div>
			<footer>
				<button class="button secondary" @click="addDialog?.close()">× Отменить</button>
				<button
					class="button primary"
					:disabled="busy || !pendingTexture || !name.trim()"
					@click="saveNew"
				>
					Сохранить скин
				</button>
			</footer>
			<p v-if="error" role="alert" class="skin-error">{{ error }}</p>
		</dialog>
		<dialog
			ref="editDialog"
			class="skin-dialog"
			aria-labelledby="skin-edit-title"
			tabindex="-1"
			@close="editOpen = false"
		>
			<header>
				<h2 id="skin-edit-title">Редактирование скина</h2>
				<button aria-label="Закрыть" @click="editDialog?.close()">×</button>
			</header>
			<div class="skin-dialog-body">
				<SkinPreview
					v-if="editOpen"
					:texture="selected?.preview?.texture || null"
					:slim="editSlim"
					:reduced-motion="reducedMotion"
				/>
				<p class="skin-rotate-hint">⟷ Перетащите, чтобы повернуть</p>
				<div class="skin-dialog-fields">
					<label
						>Текстура<button disabled title="Замена текстуры существующего скина пока недоступна">
							↑ Заменить текстуру
						</button></label
					>
					<fieldset>
						<legend>Стиль рук</legend>
						<label><input v-model="editSlim" type="radio" :value="false" /> Стив</label>
						<label><input v-model="editSlim" type="radio" :value="true" /> Алекс</label>
					</fieldset>
					<label
						>Плащ<button disabled title="Загрузка плащей пока недоступна">
							<span>×</span><span>НЕТ</span>
						</button></label
					>
				</div>
			</div>
			<footer>
				<button class="button secondary" @click="editDialog?.close()">× Отменить</button>
				<button class="button primary" :disabled="busy || !editName.trim()" @click="rename">
					Сохранить скин
				</button>
			</footer>
			<p v-if="error" role="alert" class="skin-error">{{ error }}</p>
		</dialog>
	</section>
</template>
<style scoped>
.skins-page {
	position: relative;
	padding: 28px 36px 48px;
}
.skin-page-error {
	position: absolute;
	left: 38px;
	top: 65px;
	z-index: 2;
	margin: 0;
	color: var(--figma-accent);
	font-size: 12px;
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
.skins-page {
	padding: 31px 25px 50px 38px;
}
.skins-heading {
	justify-content: flex-start;
	gap: 80px;
	margin: 0 0 22px;
}
.skins-heading h1,
.skins-heading h2 {
	margin: 0;
	font:
		500 20px/26px Geologica,
		sans-serif;
}
.skins-heading h2 {
	color: var(--figma-accent);
}
.skin-layout {
	display: grid;
	grid-template-columns: 169px minmax(0, 1fr);
	grid-template-rows: auto auto;
	gap: 0 14px;
}
.skin-list {
	grid-column: 2;
	grid-row: 1;
	display: grid;
	grid-template-columns: repeat(4, minmax(0, 136px));
	grid-auto-rows: 176px;
	gap: 14px;
	align-content: start;
	min-height: 0;
	max-height: none;
	padding: 0;
	border: 0;
	border-radius: 0;
	background: transparent;
	overflow: visible;
}
.skin-add-tile,
.skin-item {
	width: 136px;
	height: 176px;
	border: 1.5px solid var(--figma-border);
	border-radius: 22px;
	background: var(--figma-control);
	color: var(--figma-text);
	cursor: pointer;
}
.skin-add-tile {
	position: relative;
	display: flex;
	flex-direction: column;
	align-items: center;
	justify-content: center;
	gap: 5px;
	border-style: dashed;
	font-size: 10px;
}
.skin-add-tile input {
	position: absolute;
	inset: 0;
	width: 100%;
	height: 100%;
	opacity: 0;
	cursor: pointer;
}
.skin-add-tile:focus-within {
	outline: 2px solid var(--figma-accent);
	outline-offset: 2px;
}
.skin-add-tile img {
	margin-bottom: 12px;
}
.skin-add-tile strong {
	font-weight: 500;
}
.skin-add-tile small {
	color: var(--figma-muted);
	font-size: 9px;
}
.skin-item {
	position: relative;
	flex-direction: column;
	justify-content: center;
	gap: 6px;
	padding: 8px;
	text-align: center;
}
.skin-item:hover,
.skin-item.selected {
	background: var(--figma-control);
	border-color: var(--figma-accent);
}
.skin-item img {
	width: 112px;
	height: 112px;
	object-fit: contain;
	image-rendering: pixelated;
}
.skin-item span {
	max-width: 116px;
	font-size: 11px;
}
.skin-item b {
	position: absolute;
	right: 9px;
	top: 6px;
	font-size: 12px;
}
.skin-detail {
	grid-column: 1;
	grid-row: 1;
	width: 183px;
	margin-left: -13px;
}
.skin-selected-name {
	display: inline-block;
	max-width: 170px;
	margin: 28px 0 0 9px;
	padding: 9px 19px;
	border-radius: 15px;
	background: var(--figma-control);
	font:
		400 16px Geologica,
		sans-serif;
	white-space: nowrap;
	overflow: hidden;
	text-overflow: ellipsis;
}
.skin-detail .skin-preview {
	width: 183px;
	min-height: 300px;
	height: 300px;
	overflow: hidden;
}
.skin-detail .skin-preview canvas {
	max-width: none;
	width: 183px;
	height: 300px;
}
.skin-detail .skin-preview p {
	font-size: 11px;
	text-align: center;
}
.skin-controls {
	justify-content: center;
	margin: 8px 0 0;
}
.skin-edit-button {
	min-height: 24px;
	padding: 2px 12px;
	border: 0;
	border-radius: 9px;
	background: var(--figma-control);
	font-size: 12px;
}
.skin-actions {
	grid-column: 1 / -1;
	grid-row: 2;
	margin-top: 70px;
	background: var(--figma-control);
	border-color: var(--figma-border);
}
.skin-dialog {
	width: min(500px, calc(100vw - 32px));
	height: min(489px, calc(100vh - 32px));
	max-width: none;
	max-height: none;
	padding: 0;
	border: 1.5px solid var(--figma-border);
	border-radius: 27px;
	background: var(--figma-control);
	color: var(--figma-text);
	font-family: Geologica, sans-serif;
	overflow: hidden;
	outline: none;
}
.skin-dialog::backdrop {
	background: #000b;
}
.skin-dialog > header {
	display: flex;
	align-items: center;
	justify-content: space-between;
	height: 92px;
	padding: 0 31px;
	border-bottom: 1.5px solid var(--figma-border);
}
.skin-dialog > header h2 {
	margin: 0;
	font-size: 20px;
	font-weight: 600;
}
.skin-dialog > header button {
	padding: 0;
	border: 0;
	background: transparent;
	color: var(--figma-text);
	font:
		30px/1 Inter,
		sans-serif;
	cursor: pointer;
}
.skin-dialog-body {
	position: relative;
	display: grid;
	grid-template-columns: 218px minmax(0, 1fr);
	gap: 0;
	height: 315px;
	padding: 26px 30px 0;
}
.skin-dialog-body :deep(.skin-preview) {
	width: 180px;
	height: 270px;
	min-height: 0;
	overflow: hidden;
}
.skin-dialog-body :deep(.skin-preview canvas) {
	width: 180px;
	height: 270px;
	transform: translateY(-77px) scale(0.9);
}
.skin-rotate-hint {
	position: absolute;
	left: 39px;
	top: 268px;
	margin: 0;
	color: var(--figma-muted);
	font-size: 9px;
}
.skin-dialog-fields {
	display: flex;
	flex-direction: column;
	align-items: flex-start;
	gap: 13px;
	font-size: 16px;
	font-weight: 500;
}
.skin-dialog-fields > label {
	display: flex;
	flex-direction: column;
	align-items: flex-start;
	gap: 11px;
}
.skin-dialog-fields input[type='file'],
.skin-dialog-fields > label > button {
	width: 154px;
	font-size: 10px;
}
.skin-dialog-fields input[type='file']::file-selector-button,
.skin-dialog-fields > label > button {
	padding: 9px 10px;
	border: 0;
	border-radius: 12px;
	background: var(--figma-border);
	color: var(--figma-text);
	font-family: Geologica, sans-serif;
}
.skin-dialog-fields input[type='text'] {
	width: 190px;
}
.skin-dialog-fields fieldset {
	display: flex;
	gap: 16px;
	margin: 0;
	padding: 0;
	border: 0;
}
.skin-dialog-fields legend {
	margin-bottom: 10px;
	font-size: 16px;
}
.skin-dialog-fields fieldset label {
	display: inline-flex;
	align-items: center;
	gap: 6px;
	font-size: 10px;
}
.skin-dialog-fields input[type='radio'] {
	width: 13px;
	height: 13px;
	margin: 0;
	accent-color: var(--figma-accent);
}
.skin-dialog-fields > label:last-child button {
	display: flex;
	flex-direction: column;
	align-items: center;
	justify-content: center;
	width: 52px;
	height: 84px;
	gap: 0;
}
.skin-dialog-fields > label:last-child {
	margin-top: 10px;
}
.skin-dialog-fields > label:last-child button span:first-child {
	font-size: 20px;
}
.skin-dialog-fields > label:last-child button span:last-child {
	font-size: 10px;
}
.skin-dialog > footer {
	display: flex;
	justify-content: flex-end;
	gap: 21px;
	padding: 25px 31px 0;
}
.skin-dialog > footer .button {
	min-height: 31px;
	border-radius: 10px;
	font-size: 16px;
}
.skin-dialog > footer .secondary {
	width: 123px;
}
.skin-dialog > footer .primary {
	width: 175px;
	background: var(--figma-accent);
}
.skin-dialog > .skin-error {
	position: absolute;
	left: 30px;
	bottom: 5px;
	font-size: 10px;
}
@media (max-width: 1080px) {
	.skin-list {
		grid-template-columns: repeat(auto-fill, minmax(120px, 1fr));
	}
	.skin-add-tile,
	.skin-item {
		width: 100%;
	}
}
@media (max-width: 720px) {
	.skin-layout {
		grid-template-columns: 1fr;
	}
	.skin-detail,
	.skin-list,
	.skin-actions {
		grid-column: 1;
		grid-row: auto;
	}
	.skin-list {
		max-height: none;
	}
	.skin-dialog-body {
		grid-template-columns: 1fr;
		overflow: auto;
	}
}
</style>
