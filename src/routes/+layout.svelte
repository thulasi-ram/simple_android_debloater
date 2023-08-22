<script>
	import NavBar from '$lib/NavBar.svelte';
	import Sidebar from '$lib/Sidebar.svelte';
	import SadError from '$lib/error/SadError.svelte';
	import SadToast from '$lib/notifications/SadToast.svelte';
	import 'flowbite';
	import '../app.css';
	import { onMount } from 'svelte';
	import { attachConsole } from 'tauri-plugin-log-api';

	onMount(async () => {
		const _detach = await attachConsole();
	});
</script>

<div class="w-3/4 mx-auto">
	<div class="fixed top-4 right-4">
		<SadToast />
	</div>

	<NavBar />
	<SadError />

	<div class="flex flex-nowrap mt-4">
		<div class="flex-none">
			<Sidebar />
		</div>
		<div class="flex-auto px-5">
			<main class="container px-04">
				<slot />
			</main>
		</div>
	</div>
</div>
