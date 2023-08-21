<script>
	import NavBar from '$lib/NavBar.svelte';
	import SadError from '$lib/SadError.svelte';
	import SadToast from '$lib/SadToast.svelte';
	import Sidebar from '$lib/Sidebar.svelte';
	import 'flowbite';
	import '../app.css';
</script>

<div class="relative p-4">
	<div class="fixed top-4 right-4">
		<SadToast />
	</div>

	<NavBar />
	<SadError />

	<div class="flex flex-wrap mt-4">
		<div class="flex-none"><Sidebar /></div>
		<div class="flex-auto px-5">
			<main class="container px-04">
				<slot />
			</main>
		</div>
	</div>
</div>
