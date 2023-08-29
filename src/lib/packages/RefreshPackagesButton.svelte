<script>
	import { setErrorModal } from '$lib/error';
	import { notifications } from '$lib/notifications/stores';
	import { selectedUserStore } from '$lib/users/stores';
	import { IconRefresh } from '@tabler/icons-svelte';
	import { Button } from 'flowbite-svelte';
	import { get } from 'svelte/store';
	import { adb_list_packages } from './adb';
	import { packagesStore } from './stores';

	function list_packages() {
		let user = get(selectedUserStore);
		if (!user) {
			return setErrorModal('user is not selected');
		}

		adb_list_packages(user.device_id, user.id)
			.then((pkgs) => {
				notifications.info(`fetched packages for ${user?.name}`);
				packagesStore.setPackages(pkgs.device_id, pkgs.user_id, pkgs.packages);
			})
			.catch(setErrorModal);
	}
</script>

<Button color="alternative" class="gap-x-2 dark:hover:border-gray-400" on:click={list_packages}>
	Refresh Packages
	<IconRefresh size={18} stroke={1.5} />
</Button>
