<script>
	import { adb_list_packages } from '$lib/adb';
	import { setErrorModal } from '$lib/utils';
	import { Button } from 'flowbite-svelte';
	import { get } from 'svelte/store';
	import { selectedUserStore } from '../stores';
	import { notifications } from '../notificationStore';
	import { packagesStore } from '../packageStore';

	function list_packages() {
		let user = get(selectedUserStore);
		if (!user) {
			return setErrorModal('user is not selected');
		}

		adb_list_packages(user.device_id, user.id)
			.then((pkgs) => {
				notifications.info(`fetching packages for ${user?.name}`);
				packagesStore.setPackages(pkgs.deviceId, pkgs.userId, pkgs.packages);
			})
			.catch(setErrorModal);
	}
</script>

<Button class="float-right" on:click={list_packages}>Refresh Packages</Button>
