<script lang="ts">
	import '../app.css';
	import { auth } from '$lib/stores/auth';
	import { onMount } from 'svelte';
	import Backdrop from '$lib/components/Backdrop.svelte';

	let { children } = $props();

	onMount(() => {
		auth.init();
	});
</script>

<Backdrop />
{@render children()}
