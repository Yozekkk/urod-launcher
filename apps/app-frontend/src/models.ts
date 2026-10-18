export type Account = {
	uuid: string
	nickname: string
	kind: 'microsoft' | 'offline' | 'ely_by'
	account_provider?: 'microsoft' | 'offline' | 'ely_by'
	game_identity?: { uuid: string; nickname: string }
	credential_reference?: { provider: 'microsoft' | 'ely_by'; key: string } | null
	active: boolean
	skin_provider: 'mojang' | 'ely_by' | 'fallback'
}
import type { Customization } from './customization'
export type Settings = {
	locale: 'ru'
	theme: 'dark' | 'oled'
	animations: boolean
	blur: boolean
	reduced_motion: boolean
	memory_mb: number
	window_width: number | null
	window_height: number | null
	java_path: string
	java_paths: Record<string, string>
	game_directory: string
	auto_updates: boolean
	release_channel: 'stable' | 'beta'
	customization: Customization
}
export type Snapshot = { accounts: Account[]; settings: Settings; data_dir: string }
export type Skin = {
	provider: 'mojang' | 'ely_by' | 'fallback'
	head: string | null
	texture: string | null
	status: 'ready' | 'missing' | 'network_error'
}
export type EditionId = 'urod-server'
export type Edition = {
	id: EditionId
	name: string
	label: string
	description: string
	manifest: string | null
}
export const officialEdition: Edition = {
	id: 'urod-server',
	name: 'Город Уродов',
	label: 'Minecraft для всего сообщества',
	description:
		'Создавай свои сборки, играй на сервере и исследуй Minecraft вместе с Городом Уродов.',
	manifest: null,
}
