<script lang="ts">
	let { size = 32 }: { size?: number } = $props();
</script>

<svg width={size} height={size} viewBox="0 0 32 32" fill="none" xmlns="http://www.w3.org/2000/svg">
	<circle cx="16" cy="16" r="14" stroke="currentColor" stroke-width="2.5" />
	<circle cx="16" cy="16" r="6" fill="currentColor" />
	<path d="M16 6V10" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" />
	<path d="M16 22V26" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" />
	<path d="M6 16H10" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" />
	<path d="M22 16H26" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" />
</svg>
