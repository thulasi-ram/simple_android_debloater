import { writable, type Writable } from 'svelte/store';
import { defaultConfig, type Config } from './models';
import { info } from 'tauri-plugin-log-api';
import { invoke } from '@tauri-apps/api/tauri';
import { sadErrorStore } from '$lib/error/stores';

export function createConfigStore() {
	let store: Writable<Config | null> = writable(null);

	const { subscribe, set } = store;

	return {
		subscribe,
		init: async () => {
			try {
				let res: string = await invoke('get_config');
				console.info('res', res);
				set(JSON.parse(res));
			} catch (error) {
				if (error == 'config not found') {
					set(defaultConfig);
					return
				}
				console.error(error);
				sadErrorStore.setError(`unable to load config: ${error}`);
			}
		}
	};
}

export const configStore = createConfigStore();

configStore.subscribe(async (s) => {
	info(`writing store ${JSON.stringify(s)}`);
	if (s == null || Object.keys(s).length <= 0) {
		return;
	}
	try {
		await invoke('update_config', { config: JSON.stringify(s) });
	} catch (e) {
		sadErrorStore.setError('unable to update settings');
	}
});
