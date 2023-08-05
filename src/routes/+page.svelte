<script>
  import Greet from '../lib/Greet.svelte'
</script>

<h1>Welcome to SvelteKit</h1>
<Greet />

<style lang="postcss">
  :global(html) {
    background-color: theme(colors.gray.100);
  }
</style>