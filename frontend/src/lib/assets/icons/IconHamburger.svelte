<script lang="ts">
	type Props = {
		class?: string;
	};

	let { class: className = '' }: Props = $props();
</script>

<svg
	class={className}
	viewBox="0 0 24 24"
	fill="none"
	stroke="currentColor"
	stroke-width="2"
	aria-hidden="true"
>
	<path d="M4 6h16M4 12h16M4 18h16" />
</svg>
