import { sadErrorStore } from "./stores";

export function setErrorModal(e: any) {
	sadErrorStore.setError(JSON.stringify(e));
}
