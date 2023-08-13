<script>
	import NavBar from '$lib/NavBar.svelte';
	import Sidebar from '$lib/Sidebar.svelte';
	import '../app.css';
	import 'flowbite';
</script>

<div class="relative p-4">
	<NavBar />

	<div class="flex flex-wrap mt-4">
		<div class="flex-none"><Sidebar /></div>
		<div class="flex-auto px-5">
			<main class="container px-04">
				<slot />
			</main>
		</div>
	</div>
</div>
