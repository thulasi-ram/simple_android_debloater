<script>
	import { Label, Select } from 'flowbite-svelte';
	import { Icon } from 'flowbite-svelte-icons';
	import { onMount } from 'svelte';
	import {
		applicableUsersStore,
		devicesWithUsersStore,
		sadErrorStore,
		selectedDeviceIDStore,
		selectedUserIDStore
	} from '../stores';
	import SadError from './SadError.svelte';

	export let divClass = '';

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

<div class={divClass}>
	<SadError />
	<Label>
		User
		<Select
			class="mt-2"
			items={userMap}
			bind:value={$selectedUserIDStore}
			placeholder="Select user"
			size="lg"
		>
			<Icon name="envelope-solid" class="w-5 h-5 text-gray-500 dark:text-gray-400" />
		</Select>
	</Label>
</div>
