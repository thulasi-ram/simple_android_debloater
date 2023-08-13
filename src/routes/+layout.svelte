<script>
	import NavBar from '$lib/NavBar.svelte';
	import Sidebar from '$lib/Sidebar.svelte';
	import '../app.css';
	import 'flowbite';
</script>

<div class="relative p-4">
	<NavBar />

	<div class="flex flex-wrap">
		<div class="flex-none"><Sidebar /></div>
		<div class="flex-auto w-64">
			<main class="container px-04">
				<slot />
			</main>
		</div>
	</div>
</div>
