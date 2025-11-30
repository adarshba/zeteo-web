<script lang="ts">
	import { goto } from '$app/navigation';
	import { auth } from '$lib/stores/auth';
	import Logo from '$lib/components/Logo.svelte';
	import Button from '$lib/components/Button.svelte';

	let searchQuery = $state('');

	function handleSearch(e: Event) {
		e.preventDefault();
		if (searchQuery.trim()) {
			console.log('Search:', searchQuery);
		}
	}

	function handleLogout() {
		auth.logout();
		goto('/login');
	}
</script>

<svelte:head>
	<title>Zeteo</title>
</svelte:head>

{#if $auth.loading}
	<div class="loading">
		<Logo size={40} />
	</div>
{:else if !$auth.user}
	<main class="hero">
		<nav class="nav">
			<div class="nav-logo">
				<Logo size={28} />
				<span class="nav-brand">Zeteo</span>
			</div>
			<a href="/login" class="nav-link">Sign in</a>
		</nav>

		<div class="hero-content">
			<h1 class="hero-title">Discover what matters</h1>
			<p class="hero-subtitle">A minimal search experience designed for clarity and focus</p>
			<div class="hero-cta">
				<Button onclick={() => goto('/login')}>Get started</Button>
			</div>
		</div>

		<footer class="footer">
			<span>© 2024 Zeteo</span>
		</footer>
	</main>
{:else}
	<main class="app">
		<nav class="nav">
			<div class="nav-logo">
				<Logo size={28} />
				<span class="nav-brand">Zeteo</span>
			</div>
			<div class="nav-user">
				<span class="user-email">{$auth.user.email}</span>
				<button class="signout" onclick={handleLogout}>Sign out</button>
			</div>
		</nav>

		<div class="content">
			<div class="search-container">
				<form class="search-form" onsubmit={handleSearch}>
					<input
						type="text"
						class="search-input"
						placeholder="Search..."
						bind:value={searchQuery}
					/>
				</form>
			</div>
		</div>

		<footer class="footer">
			<span>© 2024 Zeteo</span>
		</footer>
	</main>
{/if}

<style>
	.loading {
		min-height: 100vh;
		display: flex;
		align-items: center;
		justify-content: center;
		animation: fadeIn 0.3s var(--transition);
	}

	.hero, .app {
		min-height: 100vh;
		display: flex;
		flex-direction: column;
	}

	.nav {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 16px 24px;
		animation: fadeIn 0.6s var(--transition);
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

	.nav-link {
		font-size: 14px;
		color: var(--fg);
		padding: 8px 16px;
		border-radius: var(--radius);
		background: var(--bg-secondary);
		transition: var(--transition);
	}

	.nav-link:hover {
		text-decoration: none;
		background: var(--border);
	}

	.nav-user {
		display: flex;
		align-items: center;
		gap: 16px;
	}

	.user-email {
		font-size: 14px;
		color: var(--fg-secondary);
	}

	.signout {
		font-size: 14px;
		color: var(--fg-secondary);
		background: none;
		border: none;
		padding: 0;
		transition: var(--transition);
	}

	.signout:hover {
		color: var(--fg);
	}

	.hero-content {
		flex: 1;
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		text-align: center;
		padding: 24px;
		animation: slideUp 0.8s var(--transition);
	}

	.hero-title {
		font-size: clamp(40px, 8vw, 72px);
		font-weight: 600;
		letter-spacing: -0.03em;
		margin-bottom: 16px;
		line-height: 1.1;
	}

	.hero-subtitle {
		font-size: clamp(17px, 3vw, 21px);
		color: var(--fg-secondary);
		max-width: 440px;
		margin-bottom: 40px;
	}

	.hero-cta {
		width: 200px;
	}

	.content {
		flex: 1;
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 24px;
		animation: scaleIn 0.6s var(--transition);
	}

	.search-container {
		width: 100%;
		max-width: 600px;
	}

	.search-form {
		width: 100%;
	}

	.search-input {
		width: 100%;
		height: 56px;
		padding: 0 24px;
		font-size: 17px;
		color: var(--fg);
		background: var(--bg-secondary);
		border: 1px solid var(--border);
		border-radius: var(--radius-lg);
		outline: none;
		transition: var(--transition);
	}

	.search-input::placeholder {
		color: var(--fg-secondary);
	}

	.search-input:focus {
		border-color: var(--accent);
		box-shadow: 0 0 0 4px rgba(0, 113, 227, 0.1);
	}

	.footer {
		padding: 16px 24px;
		text-align: center;
		font-size: 12px;
		color: var(--fg-secondary);
		animation: fadeIn 0.6s var(--transition);
	}
</style>
