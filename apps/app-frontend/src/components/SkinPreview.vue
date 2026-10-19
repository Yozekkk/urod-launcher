<script setup lang="ts">
import { onMounted, onUnmounted, useTemplateRef, watch } from 'vue'
import { SkinViewer } from 'skinview3d'

const props = defineProps<{
	texture: string | null
	slim: boolean
	reducedMotion?: boolean
	width?: number
	height?: number
}>()
const canvas = useTemplateRef<HTMLCanvasElement>('canvas')
let viewer: SkinViewer | null = null
onMounted(() => {
	if (!canvas.value) return
	try {
		viewer = new SkinViewer({
			canvas: canvas.value,
			width: props.width ?? 260,
			height: props.height ?? 320,
		})
		viewer.autoRotate = false
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
onUnmounted(() => {
	viewer?.dispose()
	viewer = null
})
</script>
<template>
	<div class="skin-preview" aria-label="Трёхмерный просмотр скина">
		<canvas ref="canvas" :width="width ?? 260" :height="height ?? 320" aria-hidden="true" />
		<p v-if="!texture">Выберите скин из библиотеки</p>
	</div>
</template>
<style scoped>
.skin-preview {
	display: grid;
	place-items: center;
	min-height: 340px;
	background: transparent;
	border: 0;
	border-radius: 0;
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
