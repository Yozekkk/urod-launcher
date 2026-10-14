<script setup lang="ts">
import { computed, ref } from 'vue'
import AppIcon from './AppIcon.vue'
const props = defineProps<{ language: string }>()
const selected = ref(0)
const items = computed(() =>
	props.language === 'en'
		? [
				{ label: 'Launcher', icon: 'settings' },
				{ label: 'Updates', icon: 'refresh' },
				{ label: 'Appearance', icon: 'moon' },
				{ label: 'Minecraft & Java', icon: 'game' },
				{ label: 'Data & app', icon: 'folder' },
				{ label: 'Customization', icon: 'moon' },
			]
		: [
				{ label: 'Лаунчер', icon: 'settings' },
				{ label: 'Обновления', icon: 'refresh' },
				{ label: 'Внешний вид', icon: 'moon' },
				{ label: 'Minecraft и Java', icon: 'game' },
				{ label: 'Данные и приложение', icon: 'folder' },
				{ label: 'Кастомизация', icon: 'moon' },
			],
)
function select(index: number) {
	selected.value = index
	document
		.querySelectorAll<HTMLElement>('.settings-section')
		[index]?.scrollIntoView({ block: 'start' })
}
</script>
<template>
	<nav
		class="settings-sidebar"
		:aria-label="language === 'en' ? 'Settings sections' : 'Разделы настроек'"
	>
		<p>{{ language === 'en' ? 'SETTINGS' : 'НАСТРОЙКИ' }}</p>
		<button
			v-for="(item, index) in items"
			:key="item.label"
			:class="{ active: selected === index }"
			@click="select(index)"
		>
			<AppIcon :name="item.icon" :size="18" />{{ item.label }}
		</button>
	</nav>
</template>
