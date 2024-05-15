<script lang="ts">
	import * as Card from '$lib/components/ui/card';
	import { Button } from '$lib/components/ui/button/index.js';
	import { Textarea } from '$lib/components/ui/textarea';
	import { ScrollArea } from '$lib/components/ui/scroll-area/index.js';
	import { Header, MarkDown, EditorActions } from '$lib/components/blocks/editor/md';
	export let data;

	let { file } = data;

	let isEditing = false;
	$: isEdited = markdownInput != file?.content;
	function setPreview() {
		isEditing = !isEditing;
	}

	function restoreFile() {
		if (file) {
			markdownInput = file?.content;
		}
	}

	let markdownInput = file ? file.content : '';
	function parseTextArea(e: Event) {
		const target = e.target as HTMLTextAreaElement;
		if (target) {
			markdownInput = target.value.replaceAll(/[\u201C\u201D]/g, '"');
		}
	}
</script>

<Card.Root class="mx-5">
	{#if file}
		<Header {isEdited} {isEditing} {setPreview} filename={file.filename} />
		<Card.Content>
			<ScrollArea class="h-[65vh]">
				{#if !isEditing}
					<MarkDown {markdownInput} />
				{:else}
					<Textarea class="h-[75vh]" bind:value={markdownInput} on:input={parseTextArea} />
				{/if}
			</ScrollArea>
		</Card.Content>
		<EditorActions {isEdited} {restoreFile} filename={file.filename} />
	{/if}
</Card.Root>
