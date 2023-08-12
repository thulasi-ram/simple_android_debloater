<script>
	import { devicesWithUsersStore, selectedSidebarItemStore } from '../stores';
	import {
		Sidebar,
		SidebarWrapper,
		SidebarBrand,
		SidebarItem,
		SidebarGroup,
		SidebarDropdownWrapper,
		SidebarDropdownItem
	} from 'flowbite-svelte';
	import { Icon } from 'flowbite-svelte-icons';
	import { onMount } from 'svelte';
	import SadLogo from '$lib/assets/images/sad.svg';

	

	let logo = {
		name: '',
		href: '/',
		img: SadLogo
	};

	let activeUrl = '';
	onMount(() => {
		selectedSidebarItemStore.subscribe((val) => {
			activeUrl = val;
		});
	});

	$: deviceMap = $devicesWithUsersStore.map((d) => ({
		name: `${d.device.name} (${d.device.model})`,
		id: d.device.id
	}));

</script>

<Sidebar>
	<SidebarWrapper>
		<SidebarGroup>
			<SidebarBrand site={logo} />

			<SidebarItem label="Simple Android Debloater" href="/" />

			<SidebarDropdownWrapper label="Devices" isOpen={true} active={true}>
				<svelte:fragment slot="icon">
					<Icon
						name="grid-solid"
						class="w-5 h-5 text-gray-500 transition duration-75 dark:text-gray-400 group-hover:text-gray-900 dark:group-hover:text-white"
					/>
				</svelte:fragment>

				{#each deviceMap as { id, name }, i}
					<SidebarDropdownItem
						label="{name}"
						href="/devices/{id}"
						active={activeUrl === '/devices/{id}'}
					/>
				{/each}
			</SidebarDropdownWrapper>
		</SidebarGroup>

		<SidebarGroup border>
			<SidebarItem label="About">
				<svelte:fragment slot="icon">
					<Icon
						name="question-circle-solid"
						class="w-5 h-5 text-gray-500 transition duration-75 dark:text-gray-400 group-hover:text-gray-900 dark:group-hover:text-white"
					/>
				</svelte:fragment>
			</SidebarItem>
		</SidebarGroup>
	</SidebarWrapper>
</Sidebar>
