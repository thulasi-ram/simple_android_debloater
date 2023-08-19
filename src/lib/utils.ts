import { sadErrorStore } from '../stores';

export function refresh() {
	sadErrorStore.reset();
}

export function setErrorModal(e: any) {
	sadErrorStore.setError(JSON.stringify(e));
}
