import { sadErrorStore } from '$lib/error/stores';
import { invoke } from '@tauri-apps/api/tauri';
import { writable, type Writable } from 'svelte/store';
import { info } from 'tauri-plugin-log-api';
import type { Config } from './models';

export function createConfigStore() {
	let store: Writable<Config | null> = writable(null);

	const { subscribe, set: _set } = store;

	function set(c: Config) {
		info(`invoking update_config ${JSON.stringify(c)}`);

		invoke('update_config', { config: c })
			.then(() => {
				_set(c);
			})
			.catch((e) => {
				sadErrorStore.setError(`unable to update config: ${e}`);
			});
	}

	return {
		set,
		subscribe,
		init: async () => {
			try {
				let res: Config = await invoke('get_config');
				set(res);
			} catch (error) {
				console.error(error);
				sadErrorStore.setError(`unable to load config: ${error}`);
			}
		}
	};
}

export const configStore = createConfigStore();