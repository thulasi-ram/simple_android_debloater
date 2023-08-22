import { writable } from 'svelte/store';

type SadError = {
	message: string;
	isPermanent: boolean;
};

function createSadErrorStore() {
	const { set, update, subscribe } = writable<SadError>({
		message: '',
		isPermanent: false
	});

	function setError(message: string, isPermanent: boolean = false) {
		update((store) => ({
			...store,
			message: message,
			isPermanent: isPermanent
		}));
	}

	return {
		subscribe,
		setError,
		reset: () => setError('', false)
	};
}

export const sadErrorStore = createSadErrorStore();
