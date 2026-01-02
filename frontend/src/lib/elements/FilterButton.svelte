<script lang="ts" generics="OptionValue extends number">
	import FilterIcon from '$lib/assets/icons/FilterIcon.svelte';

	interface Option {
		value: OptionValue;
		label: string;
	}

	interface Props {
		value: OptionValue;
		options: Option[];
		onChange: (value: OptionValue) => void;
	}

	let { value, options, onChange }: Props = $props();

	function handleChange(e: Event) {
		const newValue = Number((e.target as HTMLSelectElement).value) as OptionValue;
		onChange(newValue);
	}
</script>

<div class="filter-container">
	<FilterIcon class="filter-icon" />
	<select class="filter-select" {value} onchange={handleChange}>
		{#each options as option}
			<option value={option.value}>{option.label}</option>
		{/each}
	</select>
</div>

<style>
	.filter-container {
		position: relative;
		display: flex;
		align-items: center;
		justify-content: center;
		width: 40px;
		height: 40px;
	}

	.filter-container :global(.filter-icon) {
		position: absolute;
		width: 20px;
		height: 20px;
		pointer-events: none;
		color: var(--color-text-muted);
	}

	.filter-select {
		width: 100%;
		height: 100%;
		padding: 0;
		background-color: var(--color-surface);
		border: 1px solid var(--color-border);
		border-radius: 50%;
		color: transparent;
		font-size: 0;
		cursor: pointer;
		appearance: none;
		-webkit-appearance: none;
		font-family: inherit;
	}

	.filter-select option {
		color: var(--color-text);
		background-color: var(--color-surface);
		font-size: 0.875rem;
		font-family: inherit;
	}

	.filter-select:focus {
		outline: none;
	}

	.filter-select:active {
		background-color: var(--color-border);
	}
</style>
