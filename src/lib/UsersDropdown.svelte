<script>
	import { error } from '@sveltejs/kit';
	import { Label, Select } from 'flowbite-svelte';
	import { Icon } from 'flowbite-svelte-icons';
	import { onMount } from 'svelte';
	import {
		sadErrorStore,
		selectedDeviceIDStore,
		selectedDeviceStore,
		selectedUserIDStore
	} from '../stores';
	import SadError from './SadError.svelte';

	export let divClass = '';

	/**
	 * @param {import('../models').DeviceWithUsers} selectedDevice
	 */
	function setSelectedUser(selectedDevice) {
		if (!selectedDevice.users || selectedDevice.users.length < 1) {
			return sadErrorStore.setError('UserDetails not found');
		}

		$selectedUserIDStore = selectedDevice.users[0].id;
	}

	onMount(() => {

		selectedDeviceStore.subscribe((sd) => {
			if (sd) {
				setSelectedUser(sd);
			}
		});

		if (!$selectedDeviceStore) {
			selectedDeviceIDStore.set('');
			sadErrorStore.setError("Invalid device");
		}
	});

	$: userMap = $selectedDeviceStore?.users.map((u) => ({
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
