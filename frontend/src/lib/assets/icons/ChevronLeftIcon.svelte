<script lang="ts">
	type Props = {
		class?: string;
	};

	let { class: className = '' }: Props = $props();
</script>

<svg class={className} viewBox="0 0 24 24" fill="currentColor" aria-hidden="true">
	<path d="M15.41 7.41L14 6l-6 6 6 6 1.41-1.41L10.83 12z" />
</svg>
