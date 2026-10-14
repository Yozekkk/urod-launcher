export type SocialUser = { id: string; displayName: string; minecraftUuid?: string }
export type Friend = { user: SocialUser; since: string; presence: 'offline' | 'online' | 'in_game' }
export type FriendRequest = {
	id: string
	from: SocialUser
	to: SocialUser
	createdAt: string
	state: 'pending' | 'accepted' | 'declined'
}
export type DirectMessage = {
	id: string
	fromId: string
	toId: string
	text: string
	sentAt: string
	state: 'pending' | 'sent' | 'failed'
}
export type SocialEvent =
	| { type: 'friend.online'; userId: string }
	| { type: 'friend.offline'; userId: string }
	| { type: 'request.received'; request: FriendRequest }
	| { type: 'message.received'; message: DirectMessage }
export interface SocialApi {
	friends(): Promise<Friend[]>
	requests(): Promise<FriendRequest[]>
	messages(friendId: string): Promise<DirectMessage[]>
	sendRequest(userId: string): Promise<FriendRequest>
	sendMessage(friendId: string, text: string): Promise<DirectMessage>
	subscribe(listener: (event: SocialEvent) => void): () => void
}
export const socialUnavailable: SocialApi = {
	friends: async () => [],
	requests: async () => [],
	messages: async () => [],
	sendRequest: async () => {
		throw Error('Социальные функции пока не подключены')
	},
	sendMessage: async () => {
		throw Error('Социальные функции пока не подключены')
	},
	subscribe: () => () => {},
}
