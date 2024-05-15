<script>
	import * as AlertDialog from '../ui/alert-dialog';
	import { goto } from '$app/navigation';
	import { invoke } from '@tauri-apps/api/tauri';
	import { Button } from '../ui/button';
	import { Trash, Eye } from 'lucide-svelte';
	import { getFiles, files } from '../../../stores/files';
	import { toast } from 'svelte-sonner';

	export let filename = '';

	const navigate = () => goto(`/file/${filename}`);
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
</script>

<div>
	<div class="flex justify-between mb-2 items-center">
		<Button variant="link" on:click={navigate}>
			{filename}
		</Button>
		<div class="flex gap-2">
			<Button size="icon" variant="outline" on:click={navigate}><Eye class="h-4 w-4" /></Button>
			<AlertDialog.Root>
				<AlertDialog.Trigger asChild let:builder>
					<Button
						builders={[builder]}
						variant="outline"
						size="icon"
						class="border-red-500 text-red-500 hover:bg-red-100 hover:text-red-500"
					>
						<Trash class="h-4 w-4" />
					</Button>
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
		</div>
	</div>
	<hr />
</div>
