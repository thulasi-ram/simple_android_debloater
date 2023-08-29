<script>
	import Breadcrumbs from '$lib/BreadCrumbs.svelte';
	

	let crumbs = [
		{ name: 'Home', href: '/' },
		{ name: 'Import', href: '' }
	];
</script>

<Breadcrumbs {crumbs} />

<div class="flex flex-col gap-y-5 mt-10 text-gray-700 dark:text-gray-200">
	<p>
		Import and Export
	</p>
	
</div>
