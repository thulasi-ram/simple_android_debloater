<script>
    import { NavBrand, Navbar } from 'flowbite-svelte';
    import SadLogo from "$lib/assets/images/sad.svg";
</script>


<!-- https://flowbite-svelte.com/docs/components/navbar#Navbar -->
<Navbar
	let:hidden
	let:toggle
    rounded color="form"
	navDivClass="mx-0" 
>
	<NavBrand href="/" >
		<img src={SadLogo} alt="Simpl Android Debloater Logo" />
		<span class="self-center whitespace-nowrap text-l font-semibold dark:text-white pl-4">Simpl Android Debloater</span>
	</NavBrand>
</Navbar>
