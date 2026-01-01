<script lang="ts">
	import IconGitHub from '$lib/assets/icons/IconGitHub.svelte';
	import IconLinkedIn from '$lib/assets/icons/IconLinkedIn.svelte';
	import IconEmail from '$lib/assets/icons/IconEmail.svelte';
</script>

<h1>More</h1>

<p class="prose intro">
	Feel free to connect with me on social media or reach out via email.
</p>

<section>
	<h2>Connect</h2>
	<div class="links">
		<a
			href="https://github.com/rss-618"
			target="_blank"
			rel="noopener noreferrer"
			class="connect-link"
		>
			<IconGitHub size={24} />
			<span>GitHub</span>
		</a>
		<a
			href="https://www.linkedin.com/in/ryan-schildknecht-242b0a1b3/"
			target="_blank"
			rel="noopener noreferrer"
			class="connect-link"
		>
			<IconLinkedIn size={24} />
			<span>LinkedIn</span>
		</a>
	</div>
</section>

<section>
	<h2>Email</h2>
	<a href="mailto:schildknecht618@gmail.com" class="connect-link">
		<IconEmail size={24} />
		<span>schildknecht618@gmail.com</span>
	</a>
</section>

<style>
	h1 {
		font-size: 2rem;
		font-weight: 700;
		margin: 0 0 var(--space-md) 0;
	}

	.intro {
		color: var(--color-text-muted);
		margin-bottom: var(--space-xl);
	}

	section {
		margin-bottom: var(--space-xl);
	}

	h2 {
		font-size: 1.25rem;
		font-weight: 600;
		margin: 0 0 var(--space-md) 0;
		color: var(--color-text);
	}

	.links {
		display: flex;
		gap: var(--space-md);
	}

	.connect-link {
		display: inline-flex;
		align-items: center;
		gap: var(--space-sm);
		padding: var(--space-sm) var(--space-md);
		border: 1px solid var(--color-border);
		border-radius: var(--radius-md);
		color: var(--color-text);
		text-decoration: none;
		font-weight: 500;
		transition:
			background-color var(--transition-fast),
			border-color var(--transition-fast);
	}

	.connect-link:hover {
		background-color: var(--color-surface);
		border-color: var(--color-text-muted);
	}

	@media (max-width: 768px) {
		.links {
			flex-direction: column;
		}

		.connect-link {
			justify-content: center;
		}
	}
</style>
