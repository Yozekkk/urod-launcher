<script setup lang="ts">
import { onMounted, onUnmounted, useTemplateRef, watch } from 'vue'
import { SkinViewer } from 'skinview3d'

const props = defineProps<{ texture: string | null; slim: boolean; reducedMotion?: boolean }>()
const canvas = useTemplateRef<HTMLCanvasElement>('canvas')
let viewer: SkinViewer | null = null
onMounted(() => {
	if (!canvas.value) return
	try {
		viewer = new SkinViewer({ canvas: canvas.value, width: 260, height: 320 })
		viewer.autoRotate = !props.reducedMotion
		viewer.autoRotateSpeed = 0.35
		viewer.controls.enableZoom = false
		viewer.controls.enablePan = false
		if (props.texture)
			void viewer.loadSkin(props.texture, { model: props.slim ? 'slim' : 'default' })
	} catch {
		viewer = null
	}
})
watch(
	() => [props.texture, props.slim] as const,
	([texture, slim]) => {
		if (viewer) {
			if (texture) void viewer.loadSkin(texture, { model: slim ? 'slim' : 'default' })
			else viewer.resetSkin()
		}
	},
)
watch(
	() => props.reducedMotion,
	(value) => {
		if (viewer) viewer.autoRotate = !value
	},
)
onUnmounted(() => {
	viewer?.dispose()
	viewer = null
})
</script>
<template>
	<div class="skin-preview" aria-label="Трёхмерный просмотр скина">
		<canvas ref="canvas" width="260" height="320" aria-hidden="true" />
		<p v-if="!texture">Выберите скин из библиотеки</p>
	</div>
</template>
<style scoped>
.skin-preview {
	display: grid;
	place-items: center;
	min-height: 340px;
	background: radial-gradient(circle at 50% 35%, #96323e44, transparent 65%), #15171b;
	border: 1px solid #ffffff14;
	border-radius: 20px;
}
.skin-preview canvas {
	max-width: 100%;
}
.skin-preview p {
	color: var(--muted);
	grid-area: 1 / 1;
	align-self: end;
}
</style>
