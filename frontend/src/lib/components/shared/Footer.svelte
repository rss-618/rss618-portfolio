<script lang="ts">
	import IconGitHub from '$lib/assets/icons/IconGitHub.svelte';
	import IconLinkedIn from '$lib/assets/icons/IconLinkedIn.svelte';

	const socialLinks = [
		{ href: 'https://github.com/rss-618', label: 'GitHub', icon: IconGitHub },
		{
			href: 'https://www.linkedin.com/in/ryan-schildknecht-242b0a1b3/',
			label: 'LinkedIn',
			icon: IconLinkedIn
		}
	];

	const currentYear = new Date().getFullYear();
</script>

<footer>
	<div class="footer-content">
		<p class="copyright">&copy; {currentYear} Ryan Schildknecht</p>

		<nav class="social-links" aria-label="Social links">
			{#each socialLinks as { href, label, icon: Icon }}
				<a {href} target="_blank" rel="noopener noreferrer" aria-label={label}>
					<Icon size={20} />
				</a>
			{/each}
		</nav>
	</div>
</footer>

<style>
	footer {
		border-top: 1px solid var(--color-border);
		padding: var(--space-lg) var(--space-xl);
		margin-top: auto;
	}

	.footer-content {
		display: flex;
		align-items: center;
		justify-content: space-between;
		max-width: 1200px;
		margin: 0 auto;
	}

	.copyright {
		color: var(--color-text-muted);
		font-size: 0.875rem;
		margin: 0;
	}

	.social-links {
		display: flex;
		gap: var(--space-md);
	}

	.social-links a {
		display: flex;
		align-items: center;
		justify-content: center;
		color: var(--color-text-muted);
		transition: color var(--transition-fast);
	}

	.social-links a:hover {
		color: var(--color-text);
	}

	@media (max-width: 768px) {
		footer {
			padding: var(--space-lg) var(--space-md);
		}

		.footer-content {
			flex-direction: column;
			gap: var(--space-md);
			text-align: center;
		}
	}
</style>
