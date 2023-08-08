<script>
	import { invoke } from '@tauri-apps/api/tauri';
	import { Alert, Button, Label, Select } from 'flowbite-svelte';
	import { listen } from '@tauri-apps/api/event';
	import {
		Table,
		TableBody,
		TableBodyCell,
		TableBodyRow,
		TableHead,
		TableHeadCell,
		TableSearch
	} from 'flowbite-svelte';

	import {
		devicesWithUsersStore,
		selectedDeviceIDStore,
		applicableUsersStore,
		selectedUserIDStore,
		packagesStore
	} from '../stores';

	let cmdErr = '';
	async function adb_list_devices_with_users() {
		try {
			const cmdOutpt = await invoke('adb_list_devices_with_users');
			devicesWithUsersStore.set(cmdOutpt);
			if ($devicesWithUsersStore.length >= 1) {
				let dev = $devicesWithUsersStore[0];
				$selectedDeviceIDStore = dev.device.id;
				if (dev.users.length >= 1) {
					$selectedUserIDStore = dev.users[0].id;
				}
			}
		} catch (e) {
			cmdErr = String(e);
		}
	}

	async function adb_list_packages() {
		try {
			const cmdOutpt = await invoke('adb_list_packages', {
				deviceId: $selectedDeviceIDStore,
				userId: $selectedUserIDStore.toString()
			});
			packagesStore.set(cmdOutpt);
		} catch (e) {
			cmdErr = String(e);
		}
	}

	// let trackDevices = '';
	// await listen('rs2js', (event) => {
	// 	console.log('js: rs2js: ' + event);
	// 	let input = event.payload;
	// 	trackDevices = input;
	// });

	$: deviceMap = $devicesWithUsersStore.map((d) => ({
		name: `${d.device.name} (${d.device.model})`,
		value: d.device.id
	}));

	$: userMap = $applicableUsersStore.map((u) => ({
		name: u.name,
		value: u.id
	}));

	let searchTerm = '';
	$: filteredPackages = $packagesStore.filter(
		(pkg) => pkg.name.toLowerCase().indexOf(searchTerm.toLowerCase()) !== -1
	);
</script>

<div class="space-y-12">
	<Alert>
		{cmdErr}
	</Alert>

	<button on:click={adb_list_devices_with_users}>ADB List Devices and Users </button>
	<button on:click={adb_list_packages}>ADB List Packages</button>

	<Label
		>Select Device
		<Select class="mt-2" items={deviceMap} bind:value={$selectedDeviceIDStore} />
	</Label>

	<Label
		>Select User
		<Select class="mt-2" items={userMap} bind:value={$selectedUserIDStore} />
	</Label>

	<Table striped={true}>

	<TableSearch placeholder="Search by name" hoverable={true} bind:inputValue={searchTerm}>
		<TableHead>
			<TableHeadCell>name</TableHeadCell>
			<TableHeadCell>type</TableHeadCell>
			<TableHeadCell>state</TableHeadCell>
		</TableHead>
		<TableBody class="divide-y">
			{#each filteredPackages as pkg}
				<TableBodyRow>
					<TableBodyCell>{pkg.name}</TableBodyCell>
					<TableBodyCell>{pkg.ptype}</TableBodyCell>
					<TableBodyCell>{pkg.state}</TableBodyCell>
				</TableBodyRow>
			{/each}
		</TableBody>
	</TableSearch>
	</Table>
</div>
