<script lang="ts">
	import { IconSlash } from '@tabler/icons-svelte';
	import { Breadcrumb, BreadcrumbItem } from 'flowbite-svelte';
	import { isLastItem } from './utils';

	export let crumbs: { href: string; name: string | undefined }[];

	let linkClass =
		'ml-1 text-sm text-gray-700 hover:text-gray-900 md:ml-2 dark:text-gray-400 dark:hover:text-white hover:underline';
	let spanClass = 'ml-1 text-sm text-gray-900 md:ml-2 dark:text-gray-400';
</script>

<div class="space-y-12">
	<Breadcrumb aria-label="Sad BreadCrumb">
		{#each crumbs as { href, name }, i}
			<BreadcrumbItem {href} {linkClass} {spanClass}>
				<svelte:fragment slot="icon">
					<IconSlash size={18} stroke={1.5} class="dark:text-gray-400" />
				</svelte:fragment>
				{@const innerSpanClass = isLastItem(crumbs, i) ? 'font-medium' : ''}
				<span class={innerSpanClass}>{name}</span>
			</BreadcrumbItem>
		{/each}
	</Breadcrumb>
</div>
