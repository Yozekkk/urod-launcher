<script setup lang="ts">
import { ref } from 'vue'
import type { Account, Skin } from '../models'
import type { Page } from '../routes'

const props = defineProps<{
	page: Page
	navigation: { id: Page; label: string; icon: string }[]
	accounts: Account[]
	activeAccount: Account | null | undefined
	skins: Record<string, Skin | undefined>
	version: string
}>()
const emit = defineEmits<{
	navigate: [page: Page]
	account: [account: Account]
	activate: [account: Account]
	about: []
	create: []
}>()
const menu = ref<HTMLDialogElement | null>(null)
const figmaIcons: Record<Page, string> = {
	home: '/figma/f1283.svg',
	library: '/figma/332c7.svg',
	accounts: '/figma/03486.svg',
	skins: '/figma/df15d.svg',
	content: '/figma/8aecb.png',
	settings: '/figma/fd48a.svg',
}
function activate(account: Account) {
	emit('activate', account)
	menu.value?.close()
}
function navigate(page: Page) {
	menu.value?.close()
	emit('navigate', page)
}
function goBack() {
	window.history.back()
}
function goForward() {
	window.history.forward()
}
</script>

<template>
	<header class="app-navbar figma-header">
		<button
			class="figma-account"
			aria-label="Переключить аккаунт"
			aria-haspopup="dialog"
			@click="menu?.showModal()"
		>
			<span class="figma-account-avatar">
				<img
					v-if="activeAccount && skins[activeAccount.uuid]?.head"
					:src="skins[activeAccount.uuid]?.head || ''"
					alt=""
				/>
				<img v-else class="figma-fallback-avatar" src="/figma/2aa90.png" alt="" />
			</span>
			<span class="figma-account-copy">
				<strong>{{ activeAccount?.nickname || 'Аккаунты' }}</strong>
				<small>{{
					activeAccount?.kind === 'microsoft'
						? 'Аккаунт Microsoft'
						: activeAccount?.kind === 'ely_by'
							? 'Аккаунт Ely.by'
							: activeAccount
								? 'Офлайн аккаунт'
								: 'Выбрать аккаунт'
				}}</small>
			</span>
		</button>
		<div class="figma-header-right">
			<div class="figma-history" aria-label="История навигации">
				<button aria-label="Назад" @click="goBack">‹</button>
				<button aria-label="Вперёд" @click="goForward">›</button>
			</div>
			<a href="#/home" class="figma-wordmark" @click.prevent="navigate('home')">urodlauncher</a>
			<button class="figma-about" aria-label="О приложении" @click="emit('about')">
				<img src="/figma/2927a.svg" width="32" height="32" alt="" />
			</button>
		</div>
		<nav class="figma-sidebar" aria-label="Основная навигация">
			<a
				v-for="item in navigation"
				:key="item.id"
				:href="`#/${item.id}`"
				:class="{ active: page === item.id, 'figma-settings-link': item.id === 'settings' }"
				:aria-current="page === item.id ? 'page' : undefined"
				:title="item.label"
				:aria-label="item.label"
				@click.prevent="navigate(item.id)"
			>
				<img :src="figmaIcons[item.id]" alt="" />
			</a>
			<button
				class="figma-create-link"
				aria-label="Новая сборка"
				title="Новая сборка"
				@click="emit('create')"
			>
				<img src="/figma/319d1.svg" alt="" />
			</button>
		</nav>
		<dialog
			ref="menu"
			class="account-switcher"
			aria-labelledby="switcher-title"
			@click="$event.target === menu && menu?.close()"
		>
			<div class="account-switcher-heading">
				<h2 id="switcher-title">Аккаунты</h2>
				<button class="icon-button" aria-label="Закрыть" @click="menu?.close()">×</button>
			</div>
			<button
				v-for="account in props.accounts"
				:key="account.uuid"
				class="switcher-account"
				:class="{ active: account.active }"
				@click="activate(account)"
			>
				<img
					v-if="skins[account.uuid]?.head"
					:src="skins[account.uuid]?.head || ''"
					width="32"
					height="32"
					alt=""
				/>
				<span
					>{{ account.nickname
					}}<small>{{
						account.kind === 'offline'
							? 'Офлайн'
							: account.kind === 'ely_by'
								? 'Ely.by'
								: 'Microsoft'
					}}</small></span
				>
			</button>
			<p v-if="!accounts.length" class="field-help">Добавьте профиль, чтобы начать играть.</p>
			<button class="button secondary full-width" @click="navigate('accounts')">
				Управление аккаунтами
			</button>
			<small class="switcher-version">UrodLauncher {{ version }}</small>
		</dialog>
	</header>
</template>
