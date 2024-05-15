<script lang="ts">
	import { Header, Title } from '$lib/components/ui/card';
	import { Switch } from '$lib/components/ui/switch';
	import Label from '$lib/components/ui/label/label.svelte';
	import BreadCrumbGenerator from '../../BreadCrumbGenerator.svelte';
	export let filename;
	export let isEdited;
	export let isEditing;
	export let setPreview: () => void;
</script>

<Header>
	<BreadCrumbGenerator />
	<div class="flex justify-between">
		<Title class="Title">
			<span class="font-bold">{filename.replace('.md', '')}</span>
			{#if isEdited}
				<span class="text-red-500 text-sm"> (edited) </span>
			{/if}
		</Title>
		<div class="flex items-center space-x-2">
			<Switch id="airplane-mode" on:click={setPreview} checked={isEditing} />
			<Label for="airplane-mode">Edit Mode</Label>
		</div>
	</div>
</Header>
