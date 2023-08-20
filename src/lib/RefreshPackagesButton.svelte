<script>
	import { adb_list_packages } from '$lib/adb';
	import { setErrorModal } from '$lib/utils';
	import { Button } from 'flowbite-svelte';
	import { Icon } from 'flowbite-svelte-icons';
	import { get } from 'svelte/store';
	import { notifications } from '../notificationStore';
	import { packagesStore } from '../packageStore';
	import { selectedUserStore } from '../stores';

	function list_packages() {
		let user = get(selectedUserStore);
		if (!user) {
			return setErrorModal('user is not selected');
		}

		adb_list_packages(user.device_id, user.id)
			.then((pkgs) => {
				notifications.info(`fetching packages for ${user?.name}`);
				packagesStore.setPackages(pkgs.device_id, pkgs.user_id, pkgs.packages);
			})
			.catch(setErrorModal);
	}
</script>

<Button color="alternative" on:click={list_packages}>
	Refresh Packages
	<Icon name="rotate-outline" class="w-4 h-4 ml-2 inline text-gray-600" />

</Button>
