<script lang="ts">
	import { goto } from '$app/navigation';
	import { toast } from 'svelte-sonner';
	import { invoke } from '@tauri-apps/api/tauri';
	import { Footer } from '$lib/components/ui/card';
	import * as AlertDialog from '$lib/components/ui/alert-dialog/index.js';
	import { Button } from '$lib/components/ui/button';
	import { getFiles } from '../../../../../stores/files';
	import { files } from '../../../../../stores/files';

	export let filename = '';
	export let content = '';
	export let isEdited;
	export let restoreFile: () => void;

	function deleteFile() {
		invoke('delete_file', {
			filename
		})
			.then((res) => {
				toast(`Archivo ${filename} borrado con exito`);
				goto('/');
				getFiles()
					.then((res) => {
						console.log(res);
						files.set(res);
					})
					.catch((e) => {
						toast('Ha ocurrido un error buscando los archivos actualizados');
					});
				return;
			})
			.catch((e) => {
				if (typeof e == 'string') {
					toast(`Ha ocurrido un error ${e}`);
					return;
				}
				toast('Ha ocurrido un error intentelo nuevamente');
			});
		goto('/');
	}

	function saveFile() {
		invoke('create_file', {
			filename,
			content
		})
			.then((res) => {
				toast('Archivo guardado con exito!');
				getFiles()
					.then((res) => {
						console.log(res);
						files.set(res);
					})
					.catch((e) => {
						toast('Ha ocurrido un error buscando los archivos actualizados');
					});
			})
			.catch((e) => {
				if (typeof e === 'string') {
					toast(`Ha ocurrido un error: ${e}`);
					return;
				} else {
					toast('Ha ocurrido un error inesperado');
				}
			});
	}
</script>

<Footer class="justify-between">
	<div class="flex gap-2">
		<!-- Save file Alert -->
		<AlertDialog.Root>
			<AlertDialog.Trigger asChild let:builder>
				<Button disabled={!isEdited} builders={[builder]}>Save</Button>
			</AlertDialog.Trigger>
			<AlertDialog.Content>
				<AlertDialog.Header>
					<AlertDialog.Title>Save File?</AlertDialog.Title>
				</AlertDialog.Header>
				<AlertDialog.Footer>
					<AlertDialog.Cancel>Cancel</AlertDialog.Cancel>
					<AlertDialog.Action on:click={saveFile}>Continue</AlertDialog.Action>
				</AlertDialog.Footer>
			</AlertDialog.Content>
		</AlertDialog.Root>
		<!-- End Save File Alert -->

		<!-- Discard Changes Alert -->
		<AlertDialog.Root>
			<AlertDialog.Trigger asChild let:builder>
				<Button disabled={!isEdited} builders={[builder]} variant="outline">Discard</Button>
			</AlertDialog.Trigger>
			<AlertDialog.Content>
				<AlertDialog.Header>
					<AlertDialog.Title>File restauration</AlertDialog.Title>
					<AlertDialog.Description>
						Are you sure you want to discard your changes? This action cannot be undone
					</AlertDialog.Description>
				</AlertDialog.Header>
				<AlertDialog.Footer>
					<AlertDialog.Cancel>Cancel</AlertDialog.Cancel>
					<AlertDialog.Action on:click={restoreFile}>Continue</AlertDialog.Action>
				</AlertDialog.Footer>
			</AlertDialog.Content>
		</AlertDialog.Root>
		<!-- End Discard Changes Alert -->
	</div>

	<!-- Delete File Alert -->
	<AlertDialog.Root>
		<AlertDialog.Trigger asChild let:builder>
			<Button
				builders={[builder]}
				variant="outline"
				class="border-red-500 text-red-500 hover:bg-red-100 hover:text-red-500">Delete</Button
			>
		</AlertDialog.Trigger>
		<AlertDialog.Content>
			<AlertDialog.Header>
				<AlertDialog.Title>Are you absolutely sure?</AlertDialog.Title>
				<AlertDialog.Description>
					This action cannot be undone. This will permanently delete the file from the system.
				</AlertDialog.Description>
			</AlertDialog.Header>
			<AlertDialog.Footer>
				<AlertDialog.Cancel>Cancel</AlertDialog.Cancel>
				<AlertDialog.Action on:click={deleteFile}>Continue</AlertDialog.Action>
			</AlertDialog.Footer>
		</AlertDialog.Content>
	</AlertDialog.Root>
	<!-- End Delete File Alert -->
</Footer>
