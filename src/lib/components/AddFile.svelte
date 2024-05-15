<script lang="ts">
	import * as Dialog from '$lib/components/ui/dialog/index.js';
	import { Input } from '$lib/components/ui/input/index.js';
	import { Label } from '$lib/components/ui/label/index.js';
	import { invoke } from '@tauri-apps/api/tauri';
	import { Button, buttonVariants } from './ui/button';
	import { toast } from 'svelte-sonner';
	import { Plus } from 'lucide-svelte';
	import { getFiles, files } from '../../stores/files';
	import { goto } from '$app/navigation';

	let filename = '';
	$: fileSystem = $files;
	function addFile() {
		if (!filename.includes('.md')) {
			filename += '.md';
		}
		let fileInSystem = fileSystem.find(
			(item) => item.filename.toLowerCase() === filename.toLowerCase()
		);
		if (fileInSystem) {
			toast.error('Error', { description: 'There is a file with this name already!' });
			return;
		}
		invoke('create_file', {
			filename,
			content: ''
		})
			.then(() => {
				toast.success('File created successfully');
				getFiles()
					.then((res) => {
						files.set(res);
						goto(`/file/${filename}`);
					})
					.catch((err) => {
						console.log(err);
					});
			})
			.catch((err) => {
				toast.error('No se ha podido crear el archivo');
			});
	}
</script>

<Dialog.Root>
	<Dialog.Trigger class={buttonVariants({ variant: 'outline' })}><Plus />Add</Dialog.Trigger>
	<Dialog.Content class="sm:max-w-[425px]">
		<Dialog.Header>
			<Dialog.Title>Add a new file</Dialog.Title>
			<Dialog.Description>
				Your file will be created at ~/.notionless with a markdown extension
			</Dialog.Description>
		</Dialog.Header>
		<div class="grid gap-4 py-4">
			<div class="grid grid-cols-4 items-center gap-4">
				<Label class="text-right">Name</Label>
				<Input placeholder="file.md" bind:value={filename} class="col-span-3" />
			</div>
		</div>
		<Dialog.Footer>
			<Button type="submit" on:click={addFile}>Save file</Button>
		</Dialog.Footer>
	</Dialog.Content>
</Dialog.Root>
