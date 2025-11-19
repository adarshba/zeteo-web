<script lang="ts">
	import { onMount } from 'svelte';
	import Settings from '../lib/Settings.svelte';
	import QueryInterface from '../lib/QueryInterface.svelte';
	import LogResults from '../lib/LogResults.svelte';

	let showSettings = false;
	let config = {
		source: 'elasticsearch',
		url: '',
		username: '',
		password: '',
		organization: 'default'
	};

	let queryResults: any = null;
	let isLoading = false;
	let error = '';

	// Load config from localStorage
	onMount(() => {
		const saved = localStorage.getItem('logsExplorerConfig');
		if (saved) {
			config = JSON.parse(saved);
		} else {
			showSettings = true;
		}
	});

	function saveConfig(newConfig: any) {
		config = newConfig;
		localStorage.setItem('logsExplorerConfig', JSON.stringify(config));
		showSettings = false;
	}

	async function handleQuery(event: CustomEvent<string>) {
		const query = event.detail;
		isLoading = true;
		error = '';
		queryResults = null;

		try {
			const response = await fetch('/api/query', {
				method: 'POST',
				headers: { 'Content-Type': 'application/json' },
				body: JSON.stringify({
					query,
					source: config.source,
					config: {
						url: config.url,
						username: config.username || undefined,
						password: config.password || undefined,
						organization: config.organization
					}
				})
			});

			if (!response.ok) {
				const errorText = await response.text();
				throw new Error(errorText || 'Query failed');
			}

			queryResults = await response.json();
		} catch (err: any) {
			error = err.message || 'An error occurred';
			console.error('Query error:', err);
		} finally {
			isLoading = false;
		}
	}
</script>

<svelte:head>
	<title>Logs Explorer - AI-Powered Log Analysis</title>
</svelte:head>

<div class="app">
	<header>
		<h1>🔍 Logs Explorer</h1>
		<p>AI-Powered Log Analysis & Debugging</p>
		<button class="settings-btn" on:click={() => (showSettings = !showSettings)}>
			⚙️ Settings
		</button>
	</header>

	{#if showSettings}
		<Settings {config} on:save={(e) => saveConfig(e.detail)} on:close={() => (showSettings = false)} />
	{/if}

	<main>
		<QueryInterface on:query={handleQuery} {isLoading} />

		{#if error}
			<div class="error">
				<h3>❌ Error</h3>
				<p>{error}</p>
			</div>
		{/if}

		{#if isLoading}
			<div class="loading">
				<div class="spinner"></div>
				<p>Analyzing logs with AI...</p>
			</div>
		{/if}

		{#if queryResults}
			<LogResults results={queryResults} />
		{/if}
	</main>

	<footer>
		<p>
			Powered by Rust + AI | 
			<a href="https://github.com/adarshba/logs-explorer" target="_blank">GitHub</a>
		</p>
	</footer>
</div>

<style>
	:global(body) {
		margin: 0;
		padding: 0;
		font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu,
			Cantarell, sans-serif;
		background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
		min-height: 100vh;
	}

	.app {
		max-width: 1400px;
		margin: 0 auto;
		padding: 20px;
	}

	header {
		background: white;
		border-radius: 12px;
		padding: 30px;
		margin-bottom: 20px;
		box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
		display: flex;
		align-items: center;
		justify-content: space-between;
	}

	header h1 {
		margin: 0;
		font-size: 2.5em;
		background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
		-webkit-background-clip: text;
		-webkit-text-fill-color: transparent;
		background-clip: text;
	}

	header p {
		margin: 5px 0 0 0;
		color: #666;
		font-size: 1.1em;
	}

	.settings-btn {
		background: #667eea;
		color: white;
		border: none;
		padding: 12px 24px;
		border-radius: 8px;
		cursor: pointer;
		font-size: 1em;
		transition: all 0.3s;
	}

	.settings-btn:hover {
		background: #5568d3;
		transform: translateY(-2px);
		box-shadow: 0 4px 8px rgba(0, 0, 0, 0.2);
	}

	main {
		min-height: 500px;
	}

	.error {
		background: #fee;
		border: 2px solid #fcc;
		border-radius: 8px;
		padding: 20px;
		margin: 20px 0;
	}

	.error h3 {
		margin: 0 0 10px 0;
		color: #c00;
	}

	.loading {
		text-align: center;
		padding: 60px 20px;
		background: white;
		border-radius: 12px;
		box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
	}

	.spinner {
		width: 50px;
		height: 50px;
		border: 4px solid #f3f3f3;
		border-top: 4px solid #667eea;
		border-radius: 50%;
		animation: spin 1s linear infinite;
		margin: 0 auto 20px;
	}

	@keyframes spin {
		0% {
			transform: rotate(0deg);
		}
		100% {
			transform: rotate(360deg);
		}
	}

	footer {
		margin-top: 40px;
		text-align: center;
		color: white;
		padding: 20px;
	}

	footer a {
		color: white;
		text-decoration: underline;
	}
</style>
