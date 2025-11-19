<script lang="ts">
	export let results: any;

	let expandedLogs = new Set<number>();

	function toggleLog(index: number) {
		if (expandedLogs.has(index)) {
			expandedLogs.delete(index);
		} else {
			expandedLogs.add(index);
		}
		expandedLogs = expandedLogs;
	}

	function getLevelColor(level: string): string {
		const upperLevel = level?.toUpperCase() || 'INFO';
		if (upperLevel.includes('ERROR') || upperLevel.includes('FATAL')) return '#d32f2f';
		if (upperLevel.includes('WARN')) return '#f57c00';
		if (upperLevel.includes('INFO')) return '#1976d2';
		if (upperLevel.includes('DEBUG')) return '#7b1fa2';
		return '#666';
	}

	function formatTimestamp(ts: string): string {
		try {
			return new Date(ts).toLocaleString();
		} catch {
			return ts;
		}
	}

	function copyToClipboard(text: string) {
		navigator.clipboard.writeText(text);
		alert('Copied to clipboard!');
	}
</script>

<div class="results-container">
	<div class="results-header">
		<h3>📊 Query Results</h3>
		<div class="stats">
			<span class="stat">
				<strong>{results.total}</strong> logs found
			</span>
		</div>
	</div>

	{#if results.summary}
		<div class="summary">
			<strong>AI Summary:</strong>
			{results.summary}
		</div>
	{/if}

	{#if results.total === 0}
		<div class="no-results">
			<p>🔍 No logs found matching your query</p>
			<p class="hint">Try adjusting your search terms or time range</p>
		</div>
	{:else}
		<div class="logs-list">
			{#each results.results as log, index}
				<div class="log-entry" class:expanded={expandedLogs.has(index)}>
					<div class="log-header" on:click={() => toggleLog(index)} on:keydown={(e) => e.key === 'Enter' && toggleLog(index)}>
						<span class="log-level" style="color: {getLevelColor(log.level)}">
							{log.level || 'INFO'}
						</span>
						<span class="log-timestamp">{formatTimestamp(log.timestamp)}</span>
						{#if log.service}
							<span class="log-service">{log.service}</span>
						{/if}
						<button class="expand-btn">
							{expandedLogs.has(index) ? '▼' : '▶'}
						</button>
					</div>

					<div class="log-message">{log.message}</div>

					{#if expandedLogs.has(index)}
						<div class="log-details">
							<div class="detail-actions">
								<button
									class="copy-btn"
									on:click={() => copyToClipboard(JSON.stringify(log, null, 2))}
								>
									📋 Copy JSON
								</button>
							</div>

							{#if log.trace_id}
								<div class="detail-row">
									<strong>Trace ID:</strong>
									<code>{log.trace_id}</code>
								</div>
							{/if}

							{#if log.fields && Object.keys(log.fields).length > 0}
								<div class="detail-row">
									<strong>Additional Fields:</strong>
									<pre>{JSON.stringify(log.fields, null, 2)}</pre>
								</div>
							{/if}
						</div>
					{/if}
				</div>
			{/each}
		</div>
	{/if}
</div>

<style>
	.results-container {
		background: white;
		border-radius: 12px;
		padding: 20px;
		margin: 20px 0;
		box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
	}

	.results-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 20px;
		padding-bottom: 15px;
		border-bottom: 2px solid #eee;
	}

	.results-header h3 {
		margin: 0;
		color: #333;
	}

	.stats {
		display: flex;
		gap: 20px;
	}

	.stat {
		color: #666;
	}

	.stat strong {
		color: #667eea;
		font-size: 1.2em;
	}

	.summary {
		background: #f0f7ff;
		border: 1px solid #b3d9ff;
		border-radius: 8px;
		padding: 15px;
		margin-bottom: 20px;
		color: #0066cc;
	}

	.no-results {
		text-align: center;
		padding: 60px 20px;
		color: #666;
	}

	.no-results .hint {
		font-size: 0.9em;
		color: #999;
	}

	.logs-list {
		display: flex;
		flex-direction: column;
		gap: 10px;
	}

	.log-entry {
		border: 1px solid #e0e0e0;
		border-radius: 8px;
		padding: 15px;
		transition: all 0.3s;
	}

	.log-entry:hover {
		border-color: #667eea;
		box-shadow: 0 2px 8px rgba(102, 126, 234, 0.1);
	}

	.log-entry.expanded {
		border-color: #667eea;
		background: #fafbff;
	}

	.log-header {
		display: flex;
		align-items: center;
		gap: 15px;
		cursor: pointer;
		user-select: none;
	}

	.log-level {
		font-weight: 700;
		font-size: 0.9em;
		text-transform: uppercase;
		min-width: 60px;
	}

	.log-timestamp {
		color: #666;
		font-size: 0.9em;
		font-family: 'Courier New', monospace;
	}

	.log-service {
		background: #e3f2fd;
		color: #1976d2;
		padding: 4px 12px;
		border-radius: 12px;
		font-size: 0.85em;
		font-weight: 600;
	}

	.expand-btn {
		margin-left: auto;
		background: none;
		border: none;
		cursor: pointer;
		font-size: 1.2em;
		color: #667eea;
		padding: 0;
		width: 24px;
		height: 24px;
	}

	.log-message {
		margin-top: 10px;
		color: #333;
		line-height: 1.5;
		word-break: break-word;
	}

	.log-details {
		margin-top: 15px;
		padding-top: 15px;
		border-top: 1px solid #e0e0e0;
	}

	.detail-actions {
		margin-bottom: 15px;
	}

	.copy-btn {
		padding: 6px 12px;
		background: #667eea;
		color: white;
		border: none;
		border-radius: 6px;
		cursor: pointer;
		font-size: 0.9em;
		transition: all 0.3s;
	}

	.copy-btn:hover {
		background: #5568d3;
	}

	.detail-row {
		margin-bottom: 10px;
	}

	.detail-row strong {
		display: block;
		margin-bottom: 5px;
		color: #666;
		font-size: 0.9em;
	}

	.detail-row code {
		background: #f5f5f5;
		padding: 4px 8px;
		border-radius: 4px;
		font-family: 'Courier New', monospace;
		font-size: 0.9em;
	}

	.detail-row pre {
		background: #f5f5f5;
		padding: 15px;
		border-radius: 8px;
		overflow-x: auto;
		font-family: 'Courier New', monospace;
		font-size: 0.85em;
		margin: 5px 0 0 0;
	}
</style>
