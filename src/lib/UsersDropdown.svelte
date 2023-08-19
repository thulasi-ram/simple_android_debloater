<script>
	import { Label, Select } from 'flowbite-svelte';
	import { Icon } from 'flowbite-svelte-icons';
	import { selectedDeviceStore, selectedUserIDStore } from '../stores';

	export let divClass = '';

	$: userMap = $selectedDeviceStore?.users.map((u) => ({
		name: u.name,
		value: u.id
	}));
</script>

<div class={divClass}>
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
