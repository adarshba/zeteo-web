<script lang="ts">
	import { createEventDispatcher } from 'svelte';

	export let isLoading = false;

	let query = '';
	const dispatch = createEventDispatcher();

	const exampleQueries = [
		'Show recent errors',
		'Database timeouts in last hour',
		'Payment service errors',
		'Authentication failures',
		'Slow requests over 2 seconds'
	];

	function submitQuery() {
		if (query.trim() && !isLoading) {
			dispatch('query', query);
		}
	}

	function useExample(example: string) {
		query = example;
		submitQuery();
	}
</script>

<div class="query-container">
	<div class="query-box">
		<h3>💬 Ask anything about your logs</h3>
		<form on:submit|preventDefault={submitQuery}>
			<div class="input-group">
				<input
					type="text"
					bind:value={query}
					placeholder="e.g., Show me all errors from the last hour"
					disabled={isLoading}
				/>
				<button type="submit" disabled={isLoading || !query.trim()}>
					{#if isLoading}
						<span class="spinner-small"></span>
					{:else}
						🔍 Query
					{/if}
				</button>
			</div>
		</form>

		<div class="examples">
			<p>Try these examples:</p>
			<div class="example-buttons">
				{#each exampleQueries as example}
					<button
						class="example-btn"
						on:click={() => useExample(example)}
						disabled={isLoading}
					>
						{example}
					</button>
				{/each}
			</div>
		</div>
	</div>
</div>

<style>
	.query-container {
		margin: 20px 0;
	}

	.query-box {
		background: white;
		border-radius: 12px;
		padding: 30px;
		box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
	}

	.query-box h3 {
		margin: 0 0 20px 0;
		color: #333;
		font-size: 1.4em;
	}

	.input-group {
		display: flex;
		gap: 10px;
		margin-bottom: 20px;
	}

	.input-group input {
		flex: 1;
		padding: 15px 20px;
		border: 2px solid #e0e0e0;
		border-radius: 8px;
		font-size: 1.1em;
		transition: border-color 0.3s;
	}

	.input-group input:focus {
		outline: none;
		border-color: #667eea;
	}

	.input-group input:disabled {
		background: #f5f5f5;
		cursor: not-allowed;
	}

	.input-group button {
		padding: 15px 30px;
		background: #667eea;
		color: white;
		border: none;
		border-radius: 8px;
		font-size: 1.1em;
		cursor: pointer;
		transition: all 0.3s;
		min-width: 120px;
	}

	.input-group button:hover:not(:disabled) {
		background: #5568d3;
		transform: translateY(-2px);
		box-shadow: 0 4px 8px rgba(0, 0, 0, 0.2);
	}

	.input-group button:disabled {
		background: #ccc;
		cursor: not-allowed;
		transform: none;
	}

	.spinner-small {
		display: inline-block;
		width: 16px;
		height: 16px;
		border: 2px solid #f3f3f3;
		border-top: 2px solid #667eea;
		border-radius: 50%;
		animation: spin 1s linear infinite;
	}

	@keyframes spin {
		0% {
			transform: rotate(0deg);
		}
		100% {
			transform: rotate(360deg);
		}
	}

	.examples {
		border-top: 1px solid #eee;
		padding-top: 20px;
	}

	.examples p {
		margin: 0 0 10px 0;
		color: #666;
		font-size: 0.9em;
	}

	.example-buttons {
		display: flex;
		flex-wrap: wrap;
		gap: 10px;
	}

	.example-btn {
		padding: 8px 16px;
		background: #f5f5f5;
		border: 1px solid #e0e0e0;
		border-radius: 6px;
		cursor: pointer;
		font-size: 0.9em;
		transition: all 0.3s;
	}

	.example-btn:hover:not(:disabled) {
		background: #667eea;
		color: white;
		border-color: #667eea;
		transform: translateY(-1px);
	}

	.example-btn:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}
</style>
