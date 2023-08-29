// https://svelte.dev/repl/2254c3b9b9ba4eeda05d81d2816f6276?version=4.2.0
import { derived, writable, type Readable, type Writable } from 'svelte/store';

const TIMEOUT = 1000;

export type Notif = {
	id: string;
	type: string;
	message: string;
	timeout: number;
};

function createNotificationStore() {
	const _notifications: Writable<Notif[]> = writable([]);

	function send(message: string, type = 'default', tm: number) {
		_notifications.update((state) => {
			return [...state, { id: id(), type, message, timeout: tm }];
		});
	}

	let timers = [];

	const notifications: Readable<Notif[]> = derived(_notifications, ($_notifications, set) => {
		set($_notifications);
		if ($_notifications.length > 0) {
			const timer = setTimeout(() => {
				_notifications.update((state) => {
					state.shift();
					return state;
				});
			}, $_notifications[0].timeout);
			return () => {
				clearTimeout(timer);
			};
		}
	});
	const { subscribe } = notifications;

	return {
		subscribe,
		send,
		default: (msg: string, timeout: number = TIMEOUT) => send(msg, 'default', timeout),
		error: (msg: string, timeout: number = TIMEOUT) => send(msg, 'error', timeout),
		warning: (msg: string, timeout: number = TIMEOUT) => send(msg, 'warning', timeout),
		info: (msg: string, timeout: number = TIMEOUT) => send(msg, 'info', timeout),
		success: (msg: string, timeout: number = TIMEOUT) => send(msg, 'success', timeout)
	};
}

function id() {
	return '_' + Math.random().toString(36).substr(2, 9);
}

export const notifications = createNotificationStore();
