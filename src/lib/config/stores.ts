import { sadErrorStore } from '$lib/error/stores';
import { invoke } from '@tauri-apps/api/tauri';
import { writable, type Writable } from 'svelte/store';
import { info } from 'tauri-plugin-log-api';
import type { Config } from './models';

export function createConfigStore() {
	let store: Writable<Config | null> = writable(null);

	const { subscribe, set } = store;

	return {
		set,
		subscribe,
		init: async () => {
			try {
				let res: Config = await invoke('get_config');
				console.info('res', res);
				set(res);
			} catch (error) {
				console.error(error);
				sadErrorStore.setError(`unable to load config: ${error}`);
			}
		}
	};
}

export const configStore = createConfigStore();

let updateTimeoutID: number;

configStore.subscribe(async (s) => {
	info(`writing store ${JSON.stringify(s)}`);
	if (s == null || Object.keys(s).length <= 0) {
		return;
	}

	if (updateTimeoutID) {
		clearTimeout(updateTimeoutID);
	}

	// debounce by 300ms
	updateTimeoutID = setTimeout(() => {
		invoke('update_config', { config: s })
			.then(() => {})
			.catch((e) => {
				sadErrorStore.setError(`unable to update settings: ${e}`);
			});
	}, 300);
});
