<script>
    import SadLogo from "$lib/assets/images/sad.svg";
    import { NavBrand, Navbar } from 'flowbite-svelte';
</script>


<!-- https://flowbite-svelte.com/docs/components/navbar#Navbar -->
<Navbar
    rounded color="form"
	navDivClass="flex justify-center mx-0" 
	class="bg-transparent"
>
	<NavBrand >
		<img src={SadLogo} class="w-10 h-10" alt="Simple Android Debloater Logo" />
		<span class="self-center whitespace-nowrap text-l font-semibold dark:text-white pl-4">Simple Android Debloater</span>
	</NavBrand>
</Navbar>
