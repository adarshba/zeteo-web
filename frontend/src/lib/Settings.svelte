<script lang="ts">
	import { createEventDispatcher } from 'svelte';

	export let config: any;

	const dispatch = createEventDispatcher();

	let localConfig = { ...config };
	let activeTab = 'logs'; // 'logs' or 'ai'
	
	// AI Provider configuration (stored in localStorage)
	let aiProvider = localStorage.getItem('ai_provider') || 'server';
	let aiApiKey = localStorage.getItem('ai_api_key') || '';
	let aiModel = localStorage.getItem('ai_model') || 'gpt-4o-mini';
	
	// Azure OpenAI specific
	let azureEndpoint = localStorage.getItem('azure_endpoint') || '';
	let azureDeployment = localStorage.getItem('azure_deployment') || '';

	function save() {
		if (activeTab === 'logs') {
			if (!localConfig.url) {
				alert('Please enter a URL');
				return;
			}
			dispatch('save', localConfig);
		} else if (activeTab === 'ai') {
			saveAIConfig();
		}
	}

	function saveAIConfig() {
		// Validate required fields
		if (aiProvider !== 'server') {
			if (!aiApiKey.trim()) {
				alert('API Key is required');
				return;
			}
			
			if (aiProvider === 'azure-openai') {
				if (!azureEndpoint.trim() || !azureDeployment.trim()) {
					alert('Azure Endpoint and Deployment Name are required');
					return;
				}
			}
		}

		// Save to localStorage
		localStorage.setItem('ai_provider', aiProvider);
		localStorage.setItem('ai_api_key', aiApiKey);
		localStorage.setItem('ai_model', aiModel);
		localStorage.setItem('azure_endpoint', azureEndpoint);
		localStorage.setItem('azure_deployment', azureDeployment);
		
		alert('✓ AI configuration saved successfully!');
		close();
	}

	function clearAIConfig() {
		if (confirm('Clear all AI configuration? This will revert to server-side configuration.')) {
			localStorage.removeItem('ai_provider');
			localStorage.removeItem('ai_api_key');
			localStorage.removeItem('ai_model');
			localStorage.removeItem('azure_endpoint');
			localStorage.removeItem('azure_deployment');
			
			aiProvider = 'server';
			aiApiKey = '';
			aiModel = 'gpt-4o-mini';
			azureEndpoint = '';
			azureDeployment = '';
			
			alert('✓ Configuration cleared');
		}
	}

	function close() {
		dispatch('close');
	}
</script>

<div class="overlay" on:click={close} on:keydown={(e) => e.key === 'Escape' && close()}>
	<div class="modal" on:click|stopPropagation on:keydown|stopPropagation>
		<!-- Header -->
		<div class="modal-header">
			<h2>Settings</h2>
			<button class="btn-close" on:click={close} aria-label="Close">
				<svg width="16" height="16" viewBox="0 0 16 16" fill="currentColor">
					<path d="M3.72 3.72a.75.75 0 0 1 1.06 0L8 6.94l3.22-3.22a.749.749 0 0 1 1.275.326.749.749 0 0 1-.215.734L9.06 8l3.22 3.22a.749.749 0 0 1-.326 1.275.749.749 0 0 1-.734-.215L8 9.06l-3.22 3.22a.751.751 0 0 1-1.042-.018.751.751 0 0 1-.018-1.042L6.94 8 3.72 4.78a.75.75 0 0 1 0-1.06Z"></path>
				</svg>
			</button>
		</div>

		<!-- Navigation -->
		<nav class="nav-tabs">
			<button 
				class="nav-tab" 
				class:active={activeTab === 'logs'}
				on:click={() => activeTab = 'logs'}
			>
				Log Sources
			</button>
			<button 
				class="nav-tab" 
				class:active={activeTab === 'ai'}
				on:click={() => activeTab = 'ai'}
			>
				AI Providers
			</button>
		</nav>

		<!-- Content -->
		<div class="modal-body">
			<!-- Log Sources Tab -->
			{#if activeTab === 'logs'}
				<div class="form-section">
					<div class="section-header">
						<h3>Log source configuration</h3>
						<p>Connect to your Elasticsearch or OpenObserve instance</p>
					</div>

					<div class="form-group">
						<label class="form-label">Source type</label>
						<select class="form-control" bind:value={localConfig.source}>
							<option value="elasticsearch">Elasticsearch / Kibana</option>
							<option value="openobserve">OpenObserve</option>
						</select>
					</div>

					<div class="form-group">
						<label class="form-label">
							URL <span class="text-danger">*</span>
						</label>
						<input
							class="form-control"
							type="text"
							bind:value={localConfig.url}
							placeholder={localConfig.source === 'elasticsearch' ? 'http://localhost:9200' : 'http://localhost:5080'}
							required
						/>
						<p class="form-help">
							{#if localConfig.source === 'elasticsearch'}
								Example: http://localhost:9200
							{:else}
								Example: http://localhost:5080
							{/if}
						</p>
					</div>

					<div class="form-group">
						<label class="form-label">Username</label>
						<input
							class="form-control"
							type="text"
							bind:value={localConfig.username}
							placeholder={localConfig.source === 'elasticsearch' ? 'elastic' : 'root@example.com'}
						/>
					</div>

					<div class="form-group">
						<label class="form-label">Password</label>
						<input class="form-control" type="password" bind:value={localConfig.password} placeholder="Password" />
					</div>

					{#if localConfig.source === 'openobserve'}
						<div class="form-group">
							<label class="form-label">Organization</label>
							<input class="form-control" type="text" bind:value={localConfig.organization} placeholder="default" />
						</div>
					{/if}

					<div class="flash flash-info">
						<svg class="octicon" width="16" height="16" viewBox="0 0 16 16" fill="currentColor">
							<path d="M0 8a8 8 0 1 1 16 0A8 8 0 0 1 0 8Zm8-6.5a6.5 6.5 0 1 0 0 13 6.5 6.5 0 0 0 0-13ZM6.5 7.75A.75.75 0 0 1 7.25 7h1a.75.75 0 0 1 .75.75v2.75h.25a.75.75 0 0 1 0 1.5h-2a.75.75 0 0 1 0-1.5h.25v-2h-.25a.75.75 0 0 1-.75-.75ZM8 6a1 1 0 1 1 0-2 1 1 0 0 1 0 2Z"></path>
						</svg>
						<p>Credentials are stored locally in your browser and sent directly to your log source.</p>
					</div>
				</div>
			{/if}

			<!-- AI Providers Tab -->
			{#if activeTab === 'ai'}
				<div class="form-section">
					<div class="section-header">
						<h3>AI provider configuration</h3>
						<p>Configure which AI provider to use for log analysis</p>
					</div>

					<div class="form-group">
						<label class="form-label">Provider</label>
						<select class="form-control" bind:value={aiProvider}>
							<option value="server">Server configuration</option>
							<option value="openai">OpenAI</option>
							<option value="azure-openai">Azure OpenAI</option>
							<option value="google-ai">Google Gemini</option>
							<option value="anthropic">Anthropic Claude</option>
						</select>
					</div>

					{#if aiProvider !== 'server'}
						<!-- OpenAI -->
						{#if aiProvider === 'openai'}
							<div class="form-group">
								<label class="form-label">API Key <span class="text-danger">*</span></label>
								<input class="form-control" type="password" bind:value={aiApiKey} placeholder="sk-..." autocomplete="off" />
								<p class="form-help">
									Get from <a href="https://platform.openai.com/api-keys" target="_blank" rel="noopener">platform.openai.com</a>
								</p>
							</div>

							<div class="form-group">
								<label class="form-label">Model</label>
								<select class="form-control" bind:value={aiModel}>
									<option value="gpt-4o-mini">GPT-4o Mini</option>
									<option value="gpt-4o">GPT-4o</option>
									<option value="gpt-4-turbo">GPT-4 Turbo</option>
									<option value="gpt-3.5-turbo">GPT-3.5 Turbo</option>
								</select>
							</div>
						{/if}

						<!-- Azure OpenAI -->
						{#if aiProvider === 'azure-openai'}
							<div class="form-group">
								<label class="form-label">API Key <span class="text-danger">*</span></label>
								<input class="form-control" type="password" bind:value={aiApiKey} placeholder="Your Azure API key" autocomplete="off" />
							</div>

							<div class="form-group">
								<label class="form-label">Endpoint <span class="text-danger">*</span></label>
								<input class="form-control" type="text" bind:value={azureEndpoint} placeholder="https://your-resource.openai.azure.com" autocomplete="off" />
							</div>

							<div class="form-group">
								<label class="form-label">Deployment <span class="text-danger">*</span></label>
								<input class="form-control" type="text" bind:value={azureDeployment} placeholder="gpt-4o-mini" autocomplete="off" />
							</div>

							<div class="form-group">
								<label class="form-label">Model</label>
								<input class="form-control" type="text" bind:value={aiModel} placeholder="gpt-4o-mini" autocomplete="off" />
							</div>
						{/if}

						<!-- Google AI -->
						{#if aiProvider === 'google-ai'}
							<div class="form-group">
								<label class="form-label">API Key <span class="text-danger">*</span></label>
								<input class="form-control" type="password" bind:value={aiApiKey} placeholder="AIzaSy..." autocomplete="off" />
								<p class="form-help">
									Get from <a href="https://makersuite.google.com/app/apikey" target="_blank" rel="noopener">Google AI Studio</a>
								</p>
							</div>

							<div class="form-group">
								<label class="form-label">Model</label>
								<select class="form-control" bind:value={aiModel}>
									<option value="gemini-pro">Gemini Pro</option>
									<option value="gemini-pro-vision">Gemini Pro Vision</option>
								</select>
							</div>

							<div class="flash flash-warn">
								<svg class="octicon" width="16" height="16" viewBox="0 0 16 16" fill="currentColor">
									<path d="M6.457 1.047c.659-1.234 2.427-1.234 3.086 0l6.082 11.378A1.75 1.75 0 0 1 14.082 15H1.918a1.75 1.75 0 0 1-1.543-2.575Zm1.763.707a.25.25 0 0 0-.44 0L1.698 13.132a.25.25 0 0 0 .22.368h12.164a.25.25 0 0 0 .22-.368Zm.53 3.996v2.5a.75.75 0 0 1-1.5 0v-2.5a.75.75 0 0 1 1.5 0ZM9 11a1 1 0 1 1-2 0 1 1 0 0 1 2 0Z"></path>
								</svg>
								<p>Requires TypeScript API (Vercel deployment)</p>
							</div>
						{/if}

						<!-- Anthropic -->
						{#if aiProvider === 'anthropic'}
							<div class="form-group">
								<label class="form-label">API Key <span class="text-danger">*</span></label>
								<input class="form-control" type="password" bind:value={aiApiKey} placeholder="sk-ant-..." autocomplete="off" />
								<p class="form-help">
									Get from <a href="https://console.anthropic.com/" target="_blank" rel="noopener">Anthropic Console</a>
								</p>
							</div>

							<div class="form-group">
								<label class="form-label">Model</label>
								<select class="form-control" bind:value={aiModel}>
									<option value="claude-3-sonnet-20240229">Claude 3 Sonnet</option>
									<option value="claude-3-opus-20240229">Claude 3 Opus</option>
									<option value="claude-3-haiku-20240307">Claude 3 Haiku</option>
								</select>
							</div>

							<div class="flash flash-warn">
								<svg class="octicon" width="16" height="16" viewBox="0 0 16 16" fill="currentColor">
									<path d="M6.457 1.047c.659-1.234 2.427-1.234 3.086 0l6.082 11.378A1.75 1.75 0 0 1 14.082 15H1.918a1.75 1.75 0 0 1-1.543-2.575Zm1.763.707a.25.25 0 0 0-.44 0L1.698 13.132a.25.25 0 0 0 .22.368h12.164a.25.25 0 0 0 .22-.368Zm.53 3.996v2.5a.75.75 0 0 1-1.5 0v-2.5a.75.75 0 0 1 1.5 0ZM9 11a1 1 0 1 1-2 0 1 1 0 0 1 2 0Z"></path>
								</svg>
								<p>Requires TypeScript API (Vercel deployment)</p>
							</div>
						{/if}

						<div class="flash flash-success">
							<svg class="octicon" width="16" height="16" viewBox="0 0 16 16" fill="currentColor">
								<path d="M13.78 4.22a.75.75 0 0 1 0 1.06l-7.25 7.25a.75.75 0 0 1-1.06 0L2.22 9.28a.751.751 0 0 1 .018-1.042.751.751 0 0 1 1.042-.018L6 10.94l6.72-6.72a.75.75 0 0 1 1.06 0Z"></path>
							</svg>
							<p>Stored locally and sent directly to the provider</p>
						</div>

						<button type="button" class="btn-link" on:click={clearAIConfig}>Clear configuration</button>
					{/if}

					{#if aiProvider === 'server'}
						<div class="blankslate">
							<svg class="blankslate-icon" width="48" height="48" viewBox="0 0 16 16" fill="currentColor">
								<path d="M8 0a8 8 0 1 1 0 16A8 8 0 0 1 8 0ZM1.5 8a6.5 6.5 0 1 0 13 0 6.5 6.5 0 0 0-13 0Zm7-3.25v2.992l2.028.812a.75.75 0 0 1-.557 1.392l-2.5-1A.751.751 0 0 1 7 8.25v-3.5a.75.75 0 0 1 1.5 0Z"></path>
							</svg>
							<h3>Using server configuration</h3>
							<p>AI provider is configured via environment variables.</p>
						</div>
					{/if}
				</div>
			{/if}
		</div>

		<!-- Footer -->
		<div class="modal-footer">
			{#if activeTab === 'logs'}
				<button type="button" class="btn" on:click={close}>Cancel</button>
				<button type="button" class="btn btn-primary" on:click={save}>Save</button>
			{:else}
				{#if aiProvider !== 'server'}
					<button type="button" class="btn" on:click={close}>Cancel</button>
					<button type="button" class="btn btn-primary" on:click={save}>Save</button>
				{:else}
					<button type="button" class="btn" on:click={close}>Close</button>
				{/if}
			{/if}
		</div>
	</div>
</div>

<style>
	/* Automatically detects system color scheme preference */
	:root {
		color-scheme: light dark;
	}

	/* Light mode variables */
	@media (prefers-color-scheme: light) {
		.overlay {
			--bg-overlay: rgba(31, 35, 40, 0.5);
			--bg-canvas: #ffffff;
			--color-fg-default: #1f2328;
			--color-fg-muted: #636c76;
			--color-fg-subtle: #6e7781;
			--color-border-default: #d1d9e0;
			--color-border-muted: #d8dee4;
			--color-btn-text: #24292f;
			--color-btn-bg: #f6f8fa;
			--color-btn-border: #1f232826;
			--color-btn-hover-bg: #f3f4f6;
			--color-btn-hover-border: #1f23281f;
			--color-btn-primary-text: #ffffff;
			--color-btn-primary-bg: #1f883d;
			--color-btn-primary-hover-bg: #1a7f37;
			--color-input-bg: #ffffff;
			--color-input-border: #d1d9e0;
			--color-accent-emphasis: #0969da;
			--color-success-emphasis: #1a7f37;
			--color-attention-emphasis: #9a6700;
			--color-danger-emphasis: #d1242f;
			--color-neutral-muted: rgba(175, 184, 193, 0.2);
		}
	}

	/* Dark mode variables */
	@media (prefers-color-scheme: dark) {
		.overlay {
			--bg-overlay: rgba(13, 17, 23, 0.5);
			--bg-canvas: #0d1117;
			--color-fg-default: #e6edf3;
			--color-fg-muted: #8d96a0;
			--color-fg-subtle: #636c76;
			--color-border-default: #30363d;
			--color-border-muted: #21262d;
			--color-btn-text: #c9d1d9;
			--color-btn-bg: #21262d;
			--color-btn-border: #8b949e1f;
			--color-btn-hover-bg: #30363d;
			--color-btn-hover-border: #8b949e33;
			--color-btn-primary-text: #ffffff;
			--color-btn-primary-bg: #238636;
			--color-btn-primary-hover-bg: #2ea043;
			--color-input-bg: #0d1117;
			--color-input-border: #30363d;
			--color-accent-emphasis: #4493f8;
			--color-success-emphasis: #3fb950;
			--color-attention-emphasis: #d29922;
			--color-danger-emphasis: #f85149;
			--color-neutral-muted: rgba(110, 118, 129, 0.4);
		}
	}

	.overlay {
		position: fixed;
		inset: 0;
		background: var(--bg-overlay);
		display: flex;
		align-items: flex-start;
		justify-content: center;
		padding: 32px 16px;
		z-index: 999;
		overflow-y: auto;
	}

	.modal {
		background: var(--bg-canvas);
		border: 1px solid var(--color-border-default);
		border-radius: 12px;
		width: 100%;
		max-width: 600px;
		margin: auto;
		box-shadow: 0 16px 32px rgba(0, 0, 0, 0.15);
	}

	.modal-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 16px 24px;
		border-bottom: 1px solid var(--color-border-muted);
	}

	.modal-header h2 {
		margin: 0;
		font-size: 20px;
		font-weight: 600;
		color: var(--color-fg-default);
	}

	.btn-close {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 32px;
		height: 32px;
		padding: 0;
		background: transparent;
		border: 0;
		border-radius: 6px;
		color: var(--color-fg-muted);
		cursor: pointer;
		transition: background 0.2s;
	}

	.btn-close:hover {
		background: var(--color-neutral-muted);
		color: var(--color-fg-default);
	}

	.nav-tabs {
		display: flex;
		border-bottom: 1px solid var(--color-border-muted);
		background: var(--bg-canvas);
	}

	.nav-tab {
		flex: 1;
		padding: 12px 24px;
		background: none;
		border: 0;
		border-bottom: 2px solid transparent;
		color: var(--color-fg-muted);
		font-size: 14px;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.2s;
	}

	.nav-tab:hover {
		color: var(--color-fg-default);
		background: var(--color-neutral-muted);
	}

	.nav-tab.active {
		color: var(--color-fg-default);
		border-bottom-color: var(--color-accent-emphasis);
	}

	.modal-body {
		padding: 24px;
		max-height: 500px;
		overflow-y: auto;
	}

	.form-section {
		display: flex;
		flex-direction: column;
		gap: 20px;
	}

	.section-header h3 {
		margin: 0 0 4px 0;
		font-size: 16px;
		font-weight: 600;
		color: var(--color-fg-default);
	}

	.section-header p {
		margin: 0;
		font-size: 14px;
		color: var(--color-fg-muted);
	}

	.form-group {
		display: flex;
		flex-direction: column;
		gap: 6px;
	}

	.form-label {
		font-size: 14px;
		font-weight: 600;
		color: var(--color-fg-default);
		margin: 0;
	}

	.text-danger {
		color: var(--color-danger-emphasis);
	}

	.form-control {
		width: 100%;
		padding: 8px 12px;
		font-size: 14px;
		line-height: 20px;
		color: var(--color-fg-default);
		background-color: var(--color-input-bg);
		border: 1px solid var(--color-input-border);
		border-radius: 6px;
		box-sizing: border-box;
		transition: border 0.2s;
	}

	.form-control:focus {
		outline: none;
		border-color: var(--color-accent-emphasis);
		box-shadow: 0 0 0 3px rgba(9, 105, 218, 0.3);
	}

	@media (prefers-color-scheme: dark) {
		.form-control:focus {
			box-shadow: 0 0 0 3px rgba(68, 147, 248, 0.3);
		}
	}

	.form-help {
		margin: 0;
		font-size: 12px;
		color: var(--color-fg-muted);
	}

	.form-help a {
		color: var(--color-accent-emphasis);
		text-decoration: none;
	}

	.form-help a:hover {
		text-decoration: underline;
	}

	.flash {
		display: flex;
		gap: 12px;
		padding: 12px;
		border-radius: 6px;
		font-size: 13px;
		line-height: 1.5;
	}

	.flash p {
		margin: 0;
		flex: 1;
	}

	.octicon {
		flex-shrink: 0;
	}

	.flash-info {
		background: rgba(9, 105, 218, 0.1);
		border: 1px solid rgba(9, 105, 218, 0.2);
		color: var(--color-accent-emphasis);
	}

	@media (prefers-color-scheme: dark) {
		.flash-info {
			background: rgba(68, 147, 248, 0.1);
			border: 1px solid rgba(68, 147, 248, 0.2);
		}
	}

	.flash-success {
		background: rgba(26, 127, 55, 0.1);
		border: 1px solid rgba(26, 127, 55, 0.2);
		color: var(--color-success-emphasis);
	}

	@media (prefers-color-scheme: dark) {
		.flash-success {
			background: rgba(63, 185, 80, 0.1);
			border: 1px solid rgba(63, 185, 80, 0.2);
		}
	}

	.flash-warn {
		background: rgba(154, 103, 0, 0.1);
		border: 1px solid rgba(154, 103, 0, 0.2);
		color: var(--color-attention-emphasis);
	}

	@media (prefers-color-scheme: dark) {
		.flash-warn {
			background: rgba(210, 153, 34, 0.1);
			border: 1px solid rgba(210, 153, 34, 0.2);
		}
	}

	.blankslate {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 48px 24px;
		text-align: center;
	}

	.blankslate-icon {
		margin-bottom: 16px;
		color: var(--color-fg-muted);
	}

	.blankslate h3 {
		margin: 0 0 8px 0;
		font-size: 16px;
		font-weight: 600;
		color: var(--color-fg-default);
	}

	.blankslate p {
		margin: 0;
		font-size: 14px;
		color: var(--color-fg-muted);
	}

	.btn-link {
		padding: 0;
		background: none;
		border: 0;
		color: var(--color-accent-emphasis);
		font-size: 13px;
		cursor: pointer;
		text-decoration: none;
	}

	.btn-link:hover {
		text-decoration: underline;
	}

	.modal-footer {
		display: flex;
		justify-content: flex-end;
		gap: 8px;
		padding: 16px 24px;
		border-top: 1px solid var(--color-border-muted);
	}

	.btn {
		padding: 5px 16px;
		font-size: 14px;
		font-weight: 500;
		line-height: 20px;
		color: var(--color-btn-text);
		background-color: var(--color-btn-bg);
		border: 1px solid var(--color-btn-border);
		border-radius: 6px;
		cursor: pointer;
		transition: all 0.2s;
	}

	.btn:hover {
		background-color: var(--color-btn-hover-bg);
		border-color: var(--color-btn-hover-border);
	}

	.btn-primary {
		color: var(--color-btn-primary-text);
		background-color: var(--color-btn-primary-bg);
		border-color: var(--color-btn-primary-bg);
	}

	.btn-primary:hover {
		background-color: var(--color-btn-primary-hover-bg);
		border-color: var(--color-btn-primary-hover-bg);
	}
</style>
