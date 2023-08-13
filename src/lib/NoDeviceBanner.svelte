<script>
	 /**
	 * @type {any}
	 */
	  export let refreshButton;
</script>

<div class="flex items-center justify-center w-full">
	<label
		for="dropzone-file"
		class="flex flex-col items-center justify-center w-full h-64 border-2 border-gray-300 border-dashed rounded-lg cursor-pointer bg-gray-50 dark:hover:bg-bray-800 dark:bg-gray-700 hover:bg-gray-100 dark:border-gray-600 dark:hover:border-gray-500 dark:hover:bg-gray-600"
	>
		<div class="flex flex-col items-center justify-center pt-5 pb-6">
			<svg
				class="w-6 h-6 text-gray-800 dark:text-white"
				aria-hidden="true"
				xmlns="http://www.w3.org/2000/svg"
				fill="none"
				viewBox="0 0 20 20"
			>
				<path
					stroke="currentColor"
					stroke-linecap="round"
					stroke-linejoin="round"
					stroke-width="2"
					d="M6 14H2a1 1 0 0 1-1-1V2a1 1 0 0 1 1-1h16a1 1 0 0 1 1 1v1M5 19h5m-9-9h5m4-4h8a1 1 0 0 1 1 1v12H9V7a1 1 0 0 1 1-1Zm6 8a2 2 0 1 1-4 0 2 2 0 0 1 4 0Z"
				/>
			</svg>
			<p class="mb-2 text-base text-gray-500 dark:text-gray-400">
				<span class="font-semibold"> No Device Selected </span>
			</p>

			<svelte:component this={refreshButton} />
		</div>
	</label>
</div>
