<script>
	import { Label, Select } from 'flowbite-svelte';
	import {
		applicableUsersStore,
		selectedUserIDStore,
		selectedDeviceIDStore,
		devicesWithUsersStore,
		sadErrorStore
	} from '../stores';
	import { onMount } from 'svelte';
	import SadError from './SadError.svelte';


    async function setSelectedUser() {
        let devices = $devicesWithUsersStore.filter((dev) => dev.device.id === $selectedDeviceIDStore);
		if (!devices || devices.length < 1) {
			return sadErrorStore.setError('DeviceDetails not found');
		}

		let dev = devices[0];

		if (!dev.users || dev.users.length < 1) {
			return sadErrorStore.setError('UserDetails not found');
		}

		$selectedUserIDStore = dev.users[0].id;
    }

	onMount(() => {
		setTimeout(() => {
            setSelectedUser();
        }, 300);
	});

	$: userMap = $applicableUsersStore.map((u) => ({
		name: u.name,
		value: u.id
	}));
</script>

<div>
	<SadError />
	<Label
		>Select User
		<Select class="mt-2" items={userMap} bind:value={$selectedUserIDStore} />
	</Label>
</div>
