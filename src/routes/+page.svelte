<script lang="ts">
	import Logo from '$lib/components/Logo.svelte';
	import Button from '$lib/components/Button.svelte';
	import Terminal from '$lib/components/Terminal.svelte';
	import FeatureCard from '$lib/components/FeatureCard.svelte';

	let installSection: HTMLElement;
	let copyFeedback = $state('');

	async function copyToClipboard(text: string, label: string) {
		try {
			await navigator.clipboard.writeText(text);
			copyFeedback = label;
			setTimeout(() => { copyFeedback = ''; }, 2000);
		} catch {
			copyFeedback = 'Failed to copy';
			setTimeout(() => { copyFeedback = ''; }, 2000);
		}
	}

	function scrollToInstall() {
		installSection?.scrollIntoView({ behavior: 'smooth' });
	}

	const features = [
		{
			icon: 'chat',
			title: 'AI-Powered Chat',
			description: 'Chat with AI directly in your terminal. Supports OpenAI, Google, Azure, and Vertex AI.'
		},
		{
			icon: 'logs',
			title: 'Log Analysis',
			description: 'Query and analyze OTEL logs from Elasticsearch, OpenObserve, and Kibana with natural language.'
		},
		{
			icon: 'terminal',
			title: 'Beautiful TUI',
			description: 'Stunning terminal interface with syntax highlighting, rich formatting, and intuitive navigation.'
		},
		{
			icon: 'bolt',
			title: 'Lightning Fast',
			description: 'Built with Rust for blazing performance. Zero latency, instant responses.'
		},
		{
			icon: 'connect',
			title: 'Multi-Backend',
			description: 'Connect to multiple log backends simultaneously. Switch between them seamlessly.'
		},
		{
			icon: 'export',
			title: 'Session Export',
			description: 'Export your conversations to JSON or CSV. Share insights with your team.'
		}
	];

	const providers = [
		{ name: 'OpenAI' },
		{ name: 'Google AI' },
		{ name: 'Azure' },
		{ name: 'Vertex AI' }
	];
</script>

<svelte:head>
	<title>Zeteo – AI Assistant for Your Terminal</title>
	<meta name="description" content="AI-powered OTEL log explorer and chat assistant. Query logs, chat with AI, and explore data directly from your terminal." />
</svelte:head>

<main class="page">
	<nav class="nav">
		<div class="nav-content">
			<div class="nav-logo">
				<Logo size={28} />
				<span class="nav-brand">Zeteo</span>
			</div>
			<div class="nav-links">
				<a href="#features" class="nav-link">Features</a>
				<a href="#demo" class="nav-link">Demo</a>
				<a href="#install" class="nav-link">Install</a>
				<a href="https://github.com/adarshba/zeteo-cli" target="_blank" rel="noopener" class="nav-link nav-link-github">
					<svg width="20" height="20" viewBox="0 0 24 24" fill="currentColor">
						<path d="M12 0c-6.626 0-12 5.373-12 12 0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23.957-.266 1.983-.399 3.003-.404 1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576 4.765-1.589 8.199-6.086 8.199-11.386 0-6.627-5.373-12-12-12z"/>
					</svg>
					GitHub
				</a>
			</div>
		</div>
	</nav>

	<section class="hero">
		<div class="hero-content">
			<h1 class="hero-title">
				AI assistant<br />for your terminal.
			</h1>
			<p class="hero-subtitle">
				Chat with AI, explore logs, and analyze data. 
				All from the comfort of your command line.
			</p>
			<div class="hero-cta">
				<Button onclick={scrollToInstall}>
					Get Started
				</Button>
				<a href="#demo" class="hero-link">See it in action →</a>
			</div>
			<div class="hero-providers">
				<span class="providers-label">Supports</span>
				<div class="providers-list">
					{#each providers as provider}
						<span class="provider-badge">{provider.name}</span>
					{/each}
				</div>
			</div>
		</div>
	</section>

	<section id="demo" class="demo-section">
		<div class="section-content">
			<Terminal />
		</div>
	</section>

	<section id="features" class="features-section">
		<div class="section-content">
			<div class="section-header">
				<h2 class="section-title">Everything you need.</h2>
				<p class="section-subtitle">Powerful features designed for developers who live in the terminal.</p>
			</div>
			<div class="features-grid">
				{#each features as feature}
					<FeatureCard {...feature} />
				{/each}
			</div>
		</div>
	</section>

	<section id="install" class="install-section" bind:this={installSection}>
		<div class="section-content">
			<div class="section-header">
				<h2 class="section-title">Ready to start?</h2>
				<p class="section-subtitle">Install Zeteo with a single command.</p>
			</div>
			{#if copyFeedback}
				<div class="copy-toast" role="status" aria-live="polite">{copyFeedback}</div>
			{/if}
			<div class="install-card">
				<div class="install-step">
					<span class="step-number">1</span>
					<div class="step-content">
						<h3>Install with Cargo</h3>
						<div class="code-block">
							<code>cargo install --git https://github.com/adarshba/zeteo-cli</code>
							<button class="copy-btn" aria-label="Copy install command" onclick={() => copyToClipboard('cargo install --git https://github.com/adarshba/zeteo-cli', 'Copied install command')}>
								<svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" aria-hidden="true">
									<rect x="9" y="9" width="13" height="13" rx="2" ry="2"></rect>
									<path d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1"></path>
								</svg>
							</button>
						</div>
					</div>
				</div>
				<div class="install-step">
					<span class="step-number">2</span>
					<div class="step-content">
						<h3>Set your API key</h3>
						<div class="code-block">
							<code>export OPENAI_API_KEY="sk-..."</code>
							<button class="copy-btn" aria-label="Copy API key command" onclick={() => copyToClipboard('export OPENAI_API_KEY="sk-..."', 'Copied API key command')}>
								<svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" aria-hidden="true">
									<rect x="9" y="9" width="13" height="13" rx="2" ry="2"></rect>
									<path d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1"></path>
								</svg>
							</button>
						</div>
					</div>
				</div>
				<div class="install-step">
					<span class="step-number">3</span>
					<div class="step-content">
						<h3>Launch Zeteo</h3>
						<div class="code-block">
							<code>zeteo</code>
							<button class="copy-btn" aria-label="Copy launch command" onclick={() => copyToClipboard('zeteo', 'Copied launch command')}>
								<svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" aria-hidden="true">
									<rect x="9" y="9" width="13" height="13" rx="2" ry="2"></rect>
									<path d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1"></path>
								</svg>
							</button>
						</div>
					</div>
				</div>
			</div>
		</div>
	</section>

	<section class="cta-section">
		<div class="section-content">
			<div class="cta-card">
				<h2 class="cta-title">Start exploring today.</h2>
				<p class="cta-subtitle">Open source. Free forever. Built with Rust.</p>
				<div class="cta-buttons">
					<a href="https://github.com/adarshba/zeteo-cli" target="_blank" rel="noopener" class="cta-button primary">
						<svg width="20" height="20" viewBox="0 0 24 24" fill="currentColor">
							<path d="M12 0c-6.626 0-12 5.373-12 12 0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23.957-.266 1.983-.399 3.003-.404 1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576 4.765-1.589 8.199-6.086 8.199-11.386 0-6.627-5.373-12-12-12z"/>
						</svg>
						View on GitHub
					</a>
					<a href="https://github.com/adarshba/zeteo-cli#readme" target="_blank" rel="noopener" class="cta-button secondary">
						Read the Docs
					</a>
				</div>
			</div>
		</div>
	</section>

	<footer class="footer">
		<div class="footer-content">
			<div class="footer-logo">
				<Logo size={24} />
				<span>Zeteo</span>
			</div>
			<div class="footer-links">
				<a href="https://github.com/adarshba/zeteo-cli" target="_blank" rel="noopener">GitHub</a>
				<a href="https://github.com/adarshba/zeteo-cli/blob/main/LICENSE" target="_blank" rel="noopener">MIT License</a>
			</div>
			<p class="footer-copy">© 2024 Zeteo. Open source under MIT.</p>
		</div>
	</footer>
</main>

<style>
	.page {
		min-height: 100vh;
	}

	.nav {
		position: fixed;
		top: 0;
		left: 0;
		right: 0;
		z-index: 100;
		background: rgba(255, 255, 255, 0.8);
		backdrop-filter: blur(20px);
		-webkit-backdrop-filter: blur(20px);
		border-bottom: 1px solid var(--border);
	}

	@media (prefers-color-scheme: dark) {
		.nav {
			background: rgba(0, 0, 0, 0.8);
		}
	}

	.nav-content {
		max-width: 1200px;
		margin: 0 auto;
		padding: 16px 24px;
		display: flex;
		align-items: center;
		justify-content: space-between;
	}

	.nav-logo {
		display: flex;
		align-items: center;
		gap: 10px;
	}

	.nav-brand {
		font-size: 21px;
		font-weight: 600;
		letter-spacing: -0.02em;
	}

	.nav-links {
		display: flex;
		align-items: center;
		gap: 8px;
	}

	.nav-link {
		font-size: 14px;
		color: var(--fg-secondary);
		padding: 8px 16px;
		border-radius: var(--radius);
		transition: var(--transition);
		text-decoration: none;
	}

	.nav-link:hover {
		color: var(--fg);
		background: var(--bg-secondary);
	}

	.nav-link-github {
		display: flex;
		align-items: center;
		gap: 6px;
		background: var(--fg);
		color: var(--bg);
		padding: 8px 16px;
	}

	.nav-link-github:hover {
		opacity: 0.9;
		color: var(--bg);
		background: var(--fg);
	}

	.hero {
		min-height: 100vh;
		display: flex;
		align-items: center;
		justify-content: center;
		padding: 120px 24px 80px;
	}

	.hero-content {
		max-width: 800px;
		text-align: center;
		animation: slideUp 0.8s var(--transition);
	}

	.hero-title {
		font-size: clamp(48px, 10vw, 96px);
		font-weight: 600;
		letter-spacing: -0.04em;
		line-height: 1.05;
		margin-bottom: 24px;
		background: linear-gradient(135deg, var(--fg) 0%, var(--fg-secondary) 100%);
		-webkit-background-clip: text;
		-webkit-text-fill-color: transparent;
		background-clip: text;
	}

	.hero-subtitle {
		font-size: clamp(18px, 3vw, 24px);
		color: var(--fg-secondary);
		max-width: 560px;
		margin: 0 auto 40px;
		line-height: 1.5;
	}

	.hero-cta {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 16px;
		margin-bottom: 48px;
	}

	.hero-cta :global(.btn) {
		width: auto;
		min-width: 200px;
	}

	.hero-link {
		font-size: 17px;
		color: var(--accent);
		transition: var(--transition);
	}

	.hero-link:hover {
		text-decoration: none;
		opacity: 0.8;
	}

	.hero-providers {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 12px;
	}

	.providers-label {
		font-size: 12px;
		text-transform: uppercase;
		letter-spacing: 0.1em;
		color: var(--fg-secondary);
	}

	.providers-list {
		display: flex;
		flex-wrap: wrap;
		justify-content: center;
		gap: 8px;
	}

	.provider-badge {
		font-size: 13px;
		padding: 6px 12px;
		background: var(--bg-secondary);
		border-radius: 20px;
		color: var(--fg-secondary);
	}

	.demo-section {
		padding: 80px 24px;
		background: var(--bg-secondary);
	}

	.section-content {
		max-width: 1200px;
		margin: 0 auto;
	}

	.section-header {
		text-align: center;
		margin-bottom: 64px;
	}

	.section-title {
		font-size: clamp(32px, 6vw, 56px);
		font-weight: 600;
		letter-spacing: -0.03em;
		margin-bottom: 16px;
	}

	.section-subtitle {
		font-size: clamp(17px, 2.5vw, 21px);
		color: var(--fg-secondary);
		max-width: 500px;
		margin: 0 auto;
	}

	.features-section {
		padding: 120px 24px;
	}

	.features-grid {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(320px, 1fr));
		gap: 24px;
	}

	.install-section {
		padding: 120px 24px;
		background: var(--bg-secondary);
		position: relative;
	}

	.copy-toast {
		position: fixed;
		bottom: 24px;
		left: 50%;
		transform: translateX(-50%);
		background: var(--fg);
		color: var(--bg);
		padding: 12px 24px;
		border-radius: var(--radius);
		font-size: 14px;
		font-weight: 500;
		z-index: 1000;
		animation: slideUp 0.3s ease;
	}

	.install-card {
		max-width: 640px;
		margin: 0 auto;
		display: flex;
		flex-direction: column;
		gap: 32px;
	}

	.install-step {
		display: flex;
		gap: 20px;
		align-items: flex-start;
	}

	.step-number {
		width: 32px;
		height: 32px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: var(--fg);
		color: var(--bg);
		border-radius: 50%;
		font-size: 14px;
		font-weight: 600;
		flex-shrink: 0;
	}

	.step-content {
		flex: 1;
	}

	.step-content h3 {
		font-size: 17px;
		font-weight: 600;
		margin-bottom: 12px;
	}

	.code-block {
		display: flex;
		align-items: center;
		background: var(--bg);
		border: 1px solid var(--border);
		border-radius: var(--radius);
		padding: 12px 16px;
		overflow-x: auto;
	}

	.code-block code {
		flex: 1;
		font-family: var(--font-mono);
		font-size: 14px;
		color: var(--fg);
	}

	.copy-btn {
		background: none;
		border: none;
		padding: 4px;
		color: var(--fg-secondary);
		transition: var(--transition);
		flex-shrink: 0;
	}

	.copy-btn:hover {
		color: var(--fg);
	}

	.cta-section {
		padding: 120px 24px;
	}

	.cta-card {
		text-align: center;
		max-width: 600px;
		margin: 0 auto;
	}

	.cta-title {
		font-size: clamp(32px, 6vw, 48px);
		font-weight: 600;
		letter-spacing: -0.03em;
		margin-bottom: 16px;
	}

	.cta-subtitle {
		font-size: 21px;
		color: var(--fg-secondary);
		margin-bottom: 40px;
	}

	.cta-buttons {
		display: flex;
		justify-content: center;
		gap: 16px;
		flex-wrap: wrap;
	}

	.cta-button {
		display: inline-flex;
		align-items: center;
		gap: 8px;
		padding: 14px 28px;
		border-radius: var(--radius);
		font-size: 17px;
		font-weight: 400;
		text-decoration: none;
		transition: var(--transition);
	}

	.cta-button.primary {
		background: var(--fg);
		color: var(--bg);
	}

	.cta-button.primary:hover {
		opacity: 0.85;
	}

	.cta-button.secondary {
		background: var(--bg-secondary);
		color: var(--fg);
		border: 1px solid var(--border);
	}

	.cta-button.secondary:hover {
		background: var(--border);
	}

	.footer {
		padding: 40px 24px;
		border-top: 1px solid var(--border);
	}

	.footer-content {
		max-width: 1200px;
		margin: 0 auto;
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 20px;
		text-align: center;
	}

	.footer-logo {
		display: flex;
		align-items: center;
		gap: 8px;
		font-size: 17px;
		font-weight: 600;
	}

	.footer-links {
		display: flex;
		gap: 24px;
	}

	.footer-links a {
		font-size: 14px;
		color: var(--fg-secondary);
	}

	.footer-links a:hover {
		color: var(--fg);
	}

	.footer-copy {
		font-size: 12px;
		color: var(--fg-secondary);
	}

	@media (max-width: 768px) {
		.nav-links {
			display: none;
		}

		.hero {
			padding: 100px 24px 60px;
		}

		.features-grid {
			grid-template-columns: 1fr;
		}

		.cta-buttons {
			flex-direction: column;
			width: 100%;
		}

		.cta-button {
			width: 100%;
			justify-content: center;
		}
	}
</style>
