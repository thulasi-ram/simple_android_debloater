<script>
    import { NavBrand, Navbar } from 'flowbite-svelte';
    import SadLogo from "$lib/assets/images/sad.svg";
</script>

<Navbar
	let:hidden
	let:toggle
    rounded color="form"
>
	<NavBrand href="/" class="self-center">
		<img src={SadLogo} alt="Simpl Android Debloater Logo" />
		<span class="self-center whitespace-nowrap text-xl font-semibold dark:text-white">Simpl Android Debloater</span>
	</NavBrand>
</Navbar>
