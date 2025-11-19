<script lang="ts">
	import { createEventDispatcher } from 'svelte';

	export let config: any;

	const dispatch = createEventDispatcher();

	let localConfig = { ...config };

	function save() {
		if (!localConfig.url) {
			alert('Please enter a URL');
			return;
		}
		dispatch('save', localConfig);
	}

	function close() {
		dispatch('close');
	}
</script>

<div class="overlay" on:click={close} on:keydown={(e) => e.key === 'Escape' && close()}>
	<div class="modal" on:click|stopPropagation on:keydown|stopPropagation>
		<div class="modal-header">
			<h2>⚙️ Settings</h2>
			<button class="close-btn" on:click={close}>✕</button>
		</div>

		<div class="modal-body">
			<div class="form-group">
				<label>
					Log Source
					<select bind:value={localConfig.source}>
						<option value="elasticsearch">Elasticsearch / Kibana</option>
						<option value="openobserve">OpenObserve</option>
					</select>
				</label>
			</div>

			<div class="form-group">
				<label>
					URL *
					<input
						type="text"
						bind:value={localConfig.url}
						placeholder={localConfig.source === 'elasticsearch'
							? 'http://localhost:9200'
							: 'http://localhost:5080'}
						required
					/>
				</label>
				<small>
					{#if localConfig.source === 'elasticsearch'}
						Example: http://localhost:9200 or https://your-cluster.es.io:9200
					{:else}
						Example: http://localhost:5080
					{/if}
				</small>
			</div>

			<div class="form-group">
				<label>
					Username
					<input
						type="text"
						bind:value={localConfig.username}
						placeholder={localConfig.source === 'elasticsearch'
							? 'elastic'
							: 'root@example.com'}
					/>
				</label>
			</div>

			<div class="form-group">
				<label>
					Password
					<input type="password" bind:value={localConfig.password} placeholder="••••••••" />
				</label>
			</div>

			{#if localConfig.source === 'openobserve'}
				<div class="form-group">
					<label>
						Organization
						<input
							type="text"
							bind:value={localConfig.organization}
							placeholder="default"
						/>
					</label>
				</div>
			{/if}

			<div class="info-box">
				<h4>🔒 Security Note</h4>
				<p>
					Credentials are stored locally in your browser and sent directly to your log source.
					They are never stored on any server.
				</p>
			</div>
		</div>

		<div class="modal-footer">
			<button class="btn-secondary" on:click={close}>Cancel</button>
			<button class="btn-primary" on:click={save}>Save Settings</button>
		</div>
	</div>
</div>

<style>
	.overlay {
		position: fixed;
		top: 0;
		left: 0;
		right: 0;
		bottom: 0;
		background: rgba(0, 0, 0, 0.7);
		display: flex;
		align-items: center;
		justify-content: center;
		z-index: 1000;
	}

	.modal {
		background: white;
		border-radius: 12px;
		width: 90%;
		max-width: 600px;
		max-height: 90vh;
		overflow: auto;
		box-shadow: 0 20px 60px rgba(0, 0, 0, 0.3);
	}

	.modal-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 20px 30px;
		border-bottom: 1px solid #eee;
	}

	.modal-header h2 {
		margin: 0;
		color: #333;
	}

	.close-btn {
		background: none;
		border: none;
		font-size: 1.5em;
		cursor: pointer;
		color: #999;
		padding: 0;
		width: 30px;
		height: 30px;
		display: flex;
		align-items: center;
		justify-content: center;
		border-radius: 4px;
	}

	.close-btn:hover {
		background: #f5f5f5;
		color: #333;
	}

	.modal-body {
		padding: 30px;
	}

	.form-group {
		margin-bottom: 20px;
	}

	.form-group label {
		display: block;
		font-weight: 600;
		margin-bottom: 8px;
		color: #333;
	}

	.form-group input,
	.form-group select {
		width: 100%;
		padding: 12px;
		border: 2px solid #e0e0e0;
		border-radius: 8px;
		font-size: 1em;
		transition: border-color 0.3s;
		box-sizing: border-box;
	}

	.form-group input:focus,
	.form-group select:focus {
		outline: none;
		border-color: #667eea;
	}

	.form-group small {
		display: block;
		margin-top: 5px;
		color: #666;
		font-size: 0.9em;
	}

	.info-box {
		background: #f0f7ff;
		border: 1px solid #b3d9ff;
		border-radius: 8px;
		padding: 15px;
		margin-top: 20px;
	}

	.info-box h4 {
		margin: 0 0 8px 0;
		color: #0066cc;
		font-size: 0.95em;
	}

	.info-box p {
		margin: 0;
		font-size: 0.9em;
		color: #0066cc;
	}

	.modal-footer {
		padding: 20px 30px;
		border-top: 1px solid #eee;
		display: flex;
		justify-content: flex-end;
		gap: 10px;
	}

	.btn-primary,
	.btn-secondary {
		padding: 12px 24px;
		border: none;
		border-radius: 8px;
		font-size: 1em;
		cursor: pointer;
		transition: all 0.3s;
	}

	.btn-primary {
		background: #667eea;
		color: white;
	}

	.btn-primary:hover {
		background: #5568d3;
		transform: translateY(-2px);
		box-shadow: 0 4px 8px rgba(0, 0, 0, 0.2);
	}

	.btn-secondary {
		background: #f5f5f5;
		color: #333;
	}

	.btn-secondary:hover {
		background: #e0e0e0;
	}
</style>
