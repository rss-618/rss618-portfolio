<script lang="ts">
	import { page } from '$app/state';
	import ManagePage from '$lib/components/manage/ManagePage.svelte';
</script>

{#if page.data.user}
	<ManagePage />
{/if}
