<script lang="ts">
	import { text } from '@sveltejs/kit';
</script>

<div class="page">
	<div class="hero">
		<h1>Ryan Schildknecht</h1>
		<p class="tagline">iOS Software Engineer</p>
	</div>

	<section class="about prose">
		<h2>About</h2>
		<p>
			I'm an iOS Software Engineer based in St. Louis, specializing in Swift and SwiftUI
			development. Currently serving as iOS Tech Lead at O'Reilly Auto Parts.
		</p>
		<p>
			Beyond my professional work, I enjoy building tools that solve problems I find interesting or has a direct impact on myself or those around me.
		</p>
		<p>
			This site serves as a showcase for my personal projects, offering detailed breakdowns, source
			code, and downloads where available. Feel free to explore and reach out if anything catches
			your interest.
		</p>
	</section>
</div>

<style>
	.hero {
		margin-bottom: var(--space-2xl);
	}

	h1 {
		font-size: 2.5rem;
		font-weight: 700;
		margin: 0 0 var(--space-xs) 0;
	}

	.tagline {
		font-size: 1.25rem;
		color: var(--color-text-muted);
		margin: 0;
		text-align: center;
	}

	section {
		margin-bottom: var(--space-2xl);
	}

	h2 {
		font-size: 1.25rem;
		font-weight: 600;
		margin: 0 0 var(--space-md) 0;
		color: var(--color-text);
	}

	p {
		color: var(--color-text-muted);
		line-height: 1.7;
		margin: 0 0 var(--space-md) 0;
	}

	p:last-child {
		margin-bottom: 0;
	}

	@media (max-width: 768px) {
		h1 {
			font-size: 2rem;
		}

		.tagline {
			font-size: 1.125rem;
		}
	}
</style>
