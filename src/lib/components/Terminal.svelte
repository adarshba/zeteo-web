<script lang="ts">
	import { onMount } from 'svelte';

	let phase = $state(0);
	let typingText = $state('');
	let showCursor = $state(true);
	let showSlashModal = $state(false);
	let slashSelected = $state(0);

	const slashCommands = [
		{ name: 'backend', shortcut: 'b', desc: 'Switch log backend' },
		{ name: 'clear', shortcut: 'c', desc: 'Clear chat history' },
		{ name: 'help', shortcut: 'h', desc: 'Show available commands' },
		{ name: 'quit', shortcut: 'q', desc: 'Exit the application' },
	];

	const userQuery = 'Show me the last 5 errors from the logs';
	const aiResponse = `I found 5 errors in the logs from the last hour:

**Error Summary:**
1. \`auth-service\` - Connection timeout (2 occurrences)
2. \`payment-api\` - Invalid request format
3. \`user-service\` - Database connection failed
4. \`gateway\` - Rate limit exceeded

The most critical issue appears to be the **database connection failure** in user-service, which may be causing cascading timeouts.

*Recommendation:* Check the database server health and connection pool settings.`;

	function formatMarkdown(text: string): string {
		return text
			.split('\n')
			.map(line => {
				// Code blocks (inline)
				line = line.replace(/`([^`]+)`/g, '<code>$1</code>');
				// Bold (must be processed before italic)
				line = line.replace(/\*\*([^*]+)\*\*/g, '<strong>$1</strong>');
				// Italic (only single asterisks not adjacent to other asterisks)
				line = line.replace(/(?<!\*)\*(?!\*)([^*]+)(?<!\*)\*(?!\*)/g, '<em>$1</em>');
				// List items
				if (line.match(/^\d+\.\s/)) {
					line = '<div class="list-item">' + line + '</div>';
				}
				return line;
			})
			.join('<br/>');
	}

	// Animation phases:
	// 0 = welcome screen
	// 1 = typing user query
	// 2 = show loading state
	// 3 = show AI response
	// 4 = type slash command
	// 5 = show slash modal with navigation
	// 6 = close modal, show final state

	onMount(() => {
		// Cursor blink
		const cursorTimer = setInterval(() => {
			showCursor = !showCursor;
		}, 530);

		// Animation timeline
		const timeline = [
			{ delay: 1500, action: () => { phase = 1; } }, // Start typing
			{ delay: 3500, action: () => { typingText = userQuery; phase = 2; } }, // Show full query + loading
			{ delay: 5500, action: () => { phase = 3; } }, // Show response
			{ delay: 9000, action: () => { phase = 4; typingText = '/'; showSlashModal = true; } }, // Type slash
			{ delay: 10000, action: () => { slashSelected = 1; } }, // Navigate down
			{ delay: 10700, action: () => { slashSelected = 2; } }, // Navigate to help
			{ delay: 11400, action: () => { showSlashModal = false; phase = 6; typingText = ''; } }, // Close modal
		];

		let timeoutIds: ReturnType<typeof setTimeout>[] = [];
		let elapsed = 0;
		timeline.forEach(({ delay, action }) => {
			elapsed += delay;
			timeoutIds.push(setTimeout(action, elapsed));
		});

		// Typing animation for phase 1
		let charIndex = 0;
		const typeTimer = setInterval(() => {
			if (phase === 1 && charIndex < userQuery.length) {
				typingText = userQuery.slice(0, charIndex + 1);
				charIndex++;
			}
		}, 50);

		return () => {
			clearInterval(cursorTimer);
			clearInterval(typeTimer);
			timeoutIds.forEach(clearTimeout);
		};
	});
</script>

<div class="terminal" role="img" aria-label="Terminal demonstration showing Zeteo CLI in action">
	<div class="terminal-header">
		<div class="terminal-buttons" aria-hidden="true">
			<span class="btn-close"></span>
			<span class="btn-minimize"></span>
			<span class="btn-maximize"></span>
		</div>
		<span class="terminal-title">zeteo — zsh</span>
	</div>
	<div class="terminal-body">
		<!-- Header bar -->
		<div class="tui-header">
			<span class="header-dot">●</span>
			<span class="header-title">zeteo</span>
			<span class="header-backend">[kibana]</span>
		</div>

		<!-- Content area -->
		<div class="tui-content">
			{#if phase === 0}
				<!-- Welcome screen -->
				<div class="welcome-screen" class:fade-out={phase > 0}>
					<div class="welcome-dot">●</div>
					<div class="welcome-text">How can I help you today?</div>
					<div class="welcome-status">Connected to kibana</div>
				</div>
			{:else}
				<!-- Chat messages -->
				<div class="chat-area">
					<!-- User message -->
					<div class="message user-message">
						<div class="message-role">You</div>
						<div class="message-content">{userQuery}</div>
					</div>

					{#if phase >= 2}
						<!-- AI response -->
						<div class="message ai-message">
							<div class="message-role ai">Zeteo</div>
							{#if phase === 2}
								<div class="message-content thinking">Querying logs...</div>
							{:else}
								<div class="message-content">
									<div class="tool-indicator">📊 [Log query executed]</div>
								{@html formatMarkdown(aiResponse)}
								</div>
							{/if}
						</div>
					{/if}
				</div>
			{/if}
		</div>

		<!-- Slash command modal -->
		{#if showSlashModal}
			<div class="slash-modal">
				<div class="slash-modal-title">Commands</div>
				{#each slashCommands as cmd, i}
					<div class="slash-item" class:selected={i === slashSelected}>
						<span class="slash-name">/{cmd.name}</span>
						<span class="slash-shortcut">({cmd.shortcut})</span>
						<span class="slash-desc">{cmd.desc}</span>
					</div>
				{/each}
			</div>
		{/if}

		<!-- Input area -->
		<div class="tui-input" class:active={phase >= 1 && typingText.length > 0} class:loading={phase === 2}>
			<span class="input-prompt" class:loading={phase === 2}>
				{phase === 2 ? '◉' : '›'}
			</span>
			{#if phase === 0}
				<span class="input-placeholder">Ask about logs, errors, or any question...</span>
			{:else if phase === 2}
				<span class="input-status">Querying logs (search_logs)...</span>
			{:else}
				<span class="input-text">{typingText}</span>
			{/if}
			{#if phase !== 2 && showCursor}
				<span class="input-cursor"></span>
			{/if}
		</div>
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
		padding: 0;
		min-height: 420px;
		font-size: 14px;
		line-height: 1.6;
		color: #e0e0e0;
		display: flex;
		flex-direction: column;
		position: relative;
	}

	/* TUI Header */
	.tui-header {
		padding: 12px 20px;
		text-align: center;
		border-bottom: 1px solid #333;
	}

	.header-dot {
		color: #007aff;
		margin-right: 8px;
	}

	.header-title {
		font-weight: 600;
		color: #fff;
	}

	.header-backend {
		color: #8e8e93;
		margin-left: 8px;
	}

	/* Content area */
	.tui-content {
		flex: 1;
		padding: 20px;
		overflow: hidden;
		position: relative;
	}

	/* Welcome screen */
	.welcome-screen {
		position: absolute;
		top: 50%;
		left: 50%;
		transform: translate(-50%, -50%);
		text-align: center;
		animation: fadeIn 0.5s ease;
	}

	.welcome-screen.fade-out {
		animation: fadeOut 0.3s ease forwards;
	}

	.welcome-dot {
		font-size: 32px;
		color: #007aff;
		margin-bottom: 16px;
	}

	.welcome-text {
		color: #8e8e93;
		font-size: 16px;
		margin-bottom: 8px;
	}

	.welcome-status {
		color: #5c5c5c;
		font-size: 13px;
	}

	/* Chat area */
	.chat-area {
		animation: fadeIn 0.3s ease;
	}

	.message {
		margin-bottom: 16px;
	}

	.message-role {
		font-weight: 600;
		color: #8e8e93;
		margin-bottom: 4px;
		font-size: 12px;
	}

	.message-role.ai {
		color: #007aff;
	}

	.message-content {
		color: #e5e5ea;
		line-height: 1.7;
	}

	.message-content.thinking {
		color: #8e8e93;
		font-style: italic;
	}

	.tool-indicator {
		color: #8e8e93;
		font-size: 13px;
		margin-bottom: 12px;
	}

	.message-content :global(code) {
		background: rgba(152, 195, 121, 0.15);
		color: #98c379;
		padding: 2px 6px;
		border-radius: 4px;
		font-size: 13px;
	}

	.message-content :global(strong) {
		color: #fff;
		font-weight: 600;
	}

	.message-content :global(em) {
		color: #abb2bf;
		font-style: italic;
	}

	.message-content :global(.list-item) {
		margin-left: 8px;
		padding: 2px 0;
	}

	/* Slash command modal */
	.slash-modal {
		position: absolute;
		bottom: 70px;
		left: 24px;
		background: #1e1e1e;
		border: 1px solid #3a3a3c;
		border-radius: 8px;
		padding: 8px 0;
		min-width: 280px;
		box-shadow: 0 8px 24px rgba(0, 0, 0, 0.4);
		animation: slideUp 0.2s ease;
	}

	.slash-modal-title {
		color: #8e8e93;
		font-size: 11px;
		padding: 4px 12px 8px;
		border-bottom: 1px solid #3a3a3c;
		margin-bottom: 4px;
	}

	.slash-item {
		padding: 6px 12px;
		display: flex;
		gap: 8px;
		cursor: pointer;
		transition: background 0.15s;
	}

	.slash-item.selected {
		background: #007aff;
	}

	.slash-item.selected .slash-name,
	.slash-item.selected .slash-shortcut,
	.slash-item.selected .slash-desc {
		color: #fff;
	}

	.slash-name {
		color: #fff;
		font-weight: 500;
	}

	.slash-shortcut {
		color: #8e8e93;
		font-size: 13px;
	}

	.slash-desc {
		color: #8e8e93;
		font-size: 13px;
		margin-left: auto;
	}

	/* Input area */
	.tui-input {
		margin: 0 16px 16px;
		padding: 10px 14px;
		border: 1px solid #3a3a3c;
		border-radius: 8px;
		display: flex;
		align-items: center;
		gap: 8px;
		transition: border-color 0.2s;
	}

	.tui-input.active {
		border-color: #007aff;
	}

	.tui-input.loading {
		border-color: #ff9f0a;
	}

	.input-prompt {
		color: #007aff;
		font-weight: 500;
	}

	.input-prompt.loading {
		color: #ff9f0a;
	}

	.input-placeholder {
		color: #8e8e93;
	}

	.input-status {
		color: #8e8e93;
	}

	.input-text {
		color: #fff;
	}

	.input-cursor {
		width: 8px;
		height: 18px;
		background: #fff;
		display: inline-block;
		vertical-align: middle;
	}

	@keyframes fadeIn {
		from { opacity: 0; transform: translateY(8px); }
		to { opacity: 1; transform: translateY(0); }
	}

	@keyframes fadeOut {
		from { opacity: 1; }
		to { opacity: 0; }
	}

	@keyframes slideUp {
		from { opacity: 0; transform: translateY(8px); }
		to { opacity: 1; transform: translateY(0); }
	}
</style>
