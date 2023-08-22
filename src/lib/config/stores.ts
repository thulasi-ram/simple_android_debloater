import { writable, type Writable } from 'svelte/store';
import type { Config } from './models';

export const configStore: Writable<Config> = writable({
	prompt_disable_package: true,
	prompt_uninstall_package: true
});
