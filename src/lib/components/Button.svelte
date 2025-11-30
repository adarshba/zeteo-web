<script lang="ts">
	import type { Snippet } from 'svelte';

	let {
		variant = 'primary',
		disabled = false,
		onclick,
		children
	}: {
		variant?: 'primary' | 'secondary' | 'outline';
		disabled?: boolean;
		onclick?: () => void;
		children: Snippet;
	} = $props();
</script>

<button class="btn {variant}" {disabled} {onclick}>
	{@render children()}
</button>

<style>
	.btn {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		gap: 8px;
		height: 50px;
		padding: 0 24px;
		border: none;
		border-radius: var(--radius);
		font-size: 17px;
		font-weight: 400;
		letter-spacing: -0.022em;
		transition: var(--transition);
		width: 100%;
	}

	.primary {
		background: var(--fg);
		color: var(--bg);
	}

	.primary:hover:not(:disabled) {
		opacity: 0.85;
	}

	.secondary {
		background: var(--bg-secondary);
		color: var(--fg);
	}

	.secondary:hover:not(:disabled) {
		background: var(--border);
	}

	.outline {
		background: transparent;
		color: var(--fg);
		border: 1px solid var(--border);
	}

	.outline:hover:not(:disabled) {
		background: var(--bg-secondary);
	}

	.btn:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	.btn:active:not(:disabled) {
		transform: scale(0.98);
	}
</style>
