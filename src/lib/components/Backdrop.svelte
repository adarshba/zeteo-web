<script lang="ts">
</script>

<div class="backdrop">
	<div class="gradient"></div>
	<div class="noise"></div>
</div>

<style>
	.backdrop {
		position: fixed;
		inset: 0;
		z-index: -1;
		overflow: hidden;
	}

	.gradient {
		position: absolute;
		inset: 0;
		background: 
			radial-gradient(ellipse 80% 50% at 50% -20%, rgba(120, 119, 198, 0.15), transparent),
			radial-gradient(ellipse 60% 40% at 80% 60%, rgba(255, 100, 150, 0.1), transparent),
			radial-gradient(ellipse 50% 30% at 20% 80%, rgba(100, 200, 255, 0.1), transparent);
		animation: float 20s ease-in-out infinite;
	}

	.noise {
		position: absolute;
		inset: 0;
		background-image: url("data:image/svg+xml,%3Csvg viewBox='0 0 256 256' xmlns='http://www.w3.org/2000/svg'%3E%3Cfilter id='noise'%3E%3CfeTurbulence type='fractalNoise' baseFrequency='0.8' numOctaves='4' stitchTiles='stitch'/%3E%3C/filter%3E%3Crect width='100%25' height='100%25' filter='url(%23noise)'/%3E%3C/svg%3E");
		opacity: 0.03;
		pointer-events: none;
	}

	@keyframes float {
		0%, 100% { transform: translate(0, 0) scale(1); }
		33% { transform: translate(2%, -2%) scale(1.02); }
		66% { transform: translate(-2%, 2%) scale(0.98); }
	}

	@media (prefers-color-scheme: dark) {
		.gradient {
			background: 
				radial-gradient(ellipse 80% 50% at 50% -20%, rgba(120, 119, 198, 0.2), transparent),
				radial-gradient(ellipse 60% 40% at 80% 60%, rgba(255, 100, 150, 0.12), transparent),
				radial-gradient(ellipse 50% 30% at 20% 80%, rgba(100, 200, 255, 0.12), transparent);
		}
	}
</style>
