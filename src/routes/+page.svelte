<script lang="ts">
	import { files } from '../stores/files';
	import * as Card from '$lib/components/ui/card';
	import { Button } from '$lib/components/ui/button/index.js';
	import { Plus } from 'lucide-svelte';
	import File from '$lib/components/blocks/File.svelte';
	import AddFile from '$lib/components/AddFile.svelte';
	$: systemFiles = $files;
</script>

<div class="">
	<Card.Root>
		<Card.Header>
			<div class="flex justify-between items-center">
				<Card.Title>Notionless files</Card.Title>
				<AddFile />
			</div>
			<Card.Description>Archivos {systemFiles.length}</Card.Description>
		</Card.Header>
		<Card.Content class="grid gap-2">
			{#each systemFiles as { filename, content }}
				<File {filename} />
			{/each}
			{#if systemFiles.length < 1}
				<p class="font-thin">
					There is nothing here yet, create a new file to start using notionless
				</p>
			{/if}
		</Card.Content>
	</Card.Root>
</div>
