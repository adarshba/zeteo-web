<script lang="ts">
	import { onMount } from 'svelte';

	let currentLine = $state(0);
	let typedText = $state('');
	let isTyping = $state(false);

	const lines = [
		{ type: 'prompt', text: 'zeteo' },
		{ type: 'banner', text: `
  ███████╗███████╗████████╗███████╗ ██████╗
  ╚══███╔╝██╔════╝╚══██╔══╝██╔════╝██╔═══██╗
    ███╔╝ █████╗     ██║   █████╗  ██║   ██║
   ███╔╝  ██╔══╝     ██║   ██╔══╝  ██║   ██║
  ███████╗███████╗   ██║   ███████╗╚██████╔╝
  ╚══════╝╚══════╝   ╚═╝   ╚══════╝ ╚═════╝` },
		{ type: 'info', text: '  AI-Powered OTEL Log Explorer & Chat Assistant' },
		{ type: 'divider', text: '' },
		{ type: 'status', text: 'Provider: openai' },
		{ type: 'status', text: 'Log Explorer: Connected' },
		{ type: 'divider', text: '' },
		{ type: 'prompt', text: 'zeteo [0]> ' },
		{ type: 'user', text: 'What is OpenTelemetry?' },
		{ type: 'thinking', text: 'Thinking...' },
		{ type: 'response', text: `OpenTelemetry is an observability framework for 
cloud-native software that provides a standardized 
way to collect and export telemetry data including:

  - Distributed traces
  - Metrics  
  - Logs

It's vendor-neutral and supports multiple languages
and platforms, making it ideal for modern microservices
architectures.` },
		{ type: 'time', text: 'Response time: 1.23s' },
		{ type: 'prompt', text: 'zeteo [1]> ' },
		{ type: 'cursor', text: '' }
	];

	onMount(() => {
		const timer = setInterval(() => {
			if (currentLine < lines.length - 1) {
				currentLine++;
			}
		}, 400);

		return () => clearInterval(timer);
	});
</script>

<div class="terminal">
	<div class="terminal-header">
		<div class="terminal-buttons">
			<span class="btn-close"></span>
			<span class="btn-minimize"></span>
			<span class="btn-maximize"></span>
		</div>
		<span class="terminal-title">zeteo — bash</span>
	</div>
	<div class="terminal-body">
		{#each lines.slice(0, currentLine + 1) as line, i}
			<div class="line {line.type}" class:animate={i === currentLine}>
				{#if line.type === 'prompt'}
					<span class="prompt-text">{line.text}</span>
				{:else if line.type === 'banner'}
					<pre class="banner-text">{line.text}</pre>
				{:else if line.type === 'divider'}
					<span class="divider-line"></span>
				{:else if line.type === 'status'}
					<span class="status-icon">●</span>
					<span class="status-text">{line.text}</span>
				{:else if line.type === 'user'}
					<span class="user-text">{line.text}</span>
				{:else if line.type === 'thinking'}
					<span class="thinking-text">{line.text}</span>
				{:else if line.type === 'response'}
					<pre class="response-text">{line.text}</pre>
				{:else if line.type === 'time'}
					<span class="time-text">{line.text}</span>
				{:else if line.type === 'info'}
					<span class="info-text">{line.text}</span>
				{:else if line.type === 'cursor'}
					<span class="cursor"></span>
				{:else}
					{line.text}
				{/if}
			</div>
		{/each}
	</div>
</div>

<style>
	.terminal {
		max-width: 800px;
		margin: 0 auto;
		background: #1a1a1a;
		border-radius: 12px;
		overflow: hidden;
		box-shadow: 0 25px 50px -12px rgba(0, 0, 0, 0.5);
		font-family: var(--font-mono);
	}

	.terminal-header {
		background: #2d2d2d;
		padding: 12px 16px;
		display: flex;
		align-items: center;
		gap: 16px;
	}

	.terminal-buttons {
		display: flex;
		gap: 8px;
	}

	.terminal-buttons span {
		width: 12px;
		height: 12px;
		border-radius: 50%;
	}

	.btn-close {
		background: #ff5f56;
	}

	.btn-minimize {
		background: #ffbd2e;
	}

	.btn-maximize {
		background: #27c93f;
	}

	.terminal-title {
		color: #999;
		font-size: 13px;
	}

	.terminal-body {
		padding: 20px;
		min-height: 400px;
		font-size: 14px;
		line-height: 1.6;
		color: #e0e0e0;
	}

	.line {
		opacity: 0;
		animation: fadeInLine 0.3s forwards;
	}

	.line.animate {
		animation: fadeInLine 0.3s forwards;
	}

	@keyframes fadeInLine {
		from { opacity: 0; transform: translateY(4px); }
		to { opacity: 1; transform: translateY(0); }
	}

	.prompt-text {
		color: #27c93f;
	}

	.banner-text {
		color: #61afef;
		margin: 0;
		font-size: 11px;
		line-height: 1.2;
	}

	.info-text {
		color: #98c379;
		display: block;
		margin-top: 8px;
	}

	.divider-line {
		display: block;
		height: 1px;
		background: #333;
		margin: 12px 0;
	}

	.status-icon {
		color: #27c93f;
		margin-right: 8px;
	}

	.status-text {
		color: #abb2bf;
	}

	.user-text {
		color: #e0e0e0;
	}

	.thinking-text {
		color: #61afef;
		font-style: italic;
	}

	.response-text {
		color: #98c379;
		margin: 12px 0;
		padding: 16px;
		background: rgba(152, 195, 121, 0.1);
		border-left: 2px solid #98c379;
		white-space: pre-wrap;
	}

	.time-text {
		color: #5c6370;
		font-size: 12px;
	}

	.cursor {
		display: inline-block;
		width: 8px;
		height: 16px;
		background: #27c93f;
		animation: blink 1s infinite;
		vertical-align: middle;
	}

	@keyframes blink {
		0%, 50% { opacity: 1; }
		51%, 100% { opacity: 0; }
	}
</style>
