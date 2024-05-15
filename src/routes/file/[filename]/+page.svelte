<script lang="ts">
	import { writable } from 'svelte/store';
	import * as Card from '$lib/components/ui/card';
	import { Textarea } from '$lib/components/ui/textarea';
	import { ScrollArea } from '$lib/components/ui/scroll-area/index.js';
	import { Header, MarkDown, EditorActions } from '$lib/components/blocks/editor/md';
	import { files } from '../../../stores/files.js';
	export let data;
	let { params } = data;
	$: filesValue = $files;
	$: file = params && filesValue.find((item) => item.filename === params.filename);

	const markdownInput = writable('');

	$: {
		if (file) {
			markdownInput.set(file.content);
		} else {
			markdownInput.set('');
		}
	}

	let isEditing = false;
	$: isEdited = $markdownInput != file?.content;
	function setPreview() {
		isEditing = !isEditing;
	}

	function restoreFile() {
		if (file) {
			markdownInput.set(file?.content);
		}
	}

	function parseTextArea(e: Event) {
		const target = e.target as HTMLTextAreaElement;
		if (target) {
			markdownInput.set(target.value.replaceAll(/[\u201C\u201D]/g, '"'));
		}
	}
</script>

<Card.Root>
	{#if file}
		<Header {isEdited} {isEditing} {setPreview} filename={file.filename} />
		<Card.Content>
			<ScrollArea class="h-[65vh]">
				{#if !isEditing}
					<MarkDown markdownInput={$markdownInput || ''} />
				{:else}
					<Textarea class="h-[75vh]" bind:value={$markdownInput} on:input={parseTextArea} />
				{/if}
			</ScrollArea>
		</Card.Content>
		<EditorActions {isEdited} {restoreFile} filename={file.filename} content={$markdownInput} />
	{/if}
</Card.Root>
