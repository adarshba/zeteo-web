<script lang="ts">
	let { icon, title, description }: { icon: string; title: string; description: string } = $props();
</script>

<div class="feature-card">
	<span class="feature-icon">{icon}</span>
	<h3 class="feature-title">{title}</h3>
	<p class="feature-description">{description}</p>
</div>

<style>
	.feature-card {
		padding: 32px;
		background: var(--bg-secondary);
		border-radius: var(--radius-lg);
		transition: var(--transition);
	}

	.feature-card:hover {
		transform: translateY(-4px);
		box-shadow: var(--shadow-lg);
	}

	.feature-icon {
		font-size: 32px;
		display: block;
		margin-bottom: 16px;
	}

	.feature-title {
		font-size: 21px;
		font-weight: 600;
		letter-spacing: -0.02em;
		margin-bottom: 8px;
	}

	.feature-description {
		font-size: 15px;
		color: var(--fg-secondary);
		line-height: 1.5;
	}
</style>
