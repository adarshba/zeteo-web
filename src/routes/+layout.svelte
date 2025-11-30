<script lang="ts">
	import '../app.css';
	import Backdrop from '$lib/components/Backdrop.svelte';

	let { children } = $props();
</script>

<Backdrop />
{@render children()}
