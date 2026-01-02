<script lang="ts">
	type Props = {
		class?: string;
	};

	let { class: className = '' }: Props = $props();
</script>

<svg class={className} viewBox="0 0 24 24" fill="currentColor" aria-hidden="true">
	<path d="M10 6L8.59 7.41 13.17 12l-4.58 4.59L10 18l6-6z" />
</svg>
