<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';
	let name = '';
	let content = '';

	async function getFiles() {
		let response = await invoke('get_files');
		console.log(response);
	}

	async function createFile() {
		let response = await invoke('create_file', {
			filename: name,
			content
		});
		console.log(response);
		await getFiles();
	}
</script>

<h1>New file</h1>
<div class="max-w-[60%] mx-auto">
	<div class="grid">
		<label for="filename"> Filename </label>
		<input bind:value={name} class="text-black" />
	</div>
	<div class="grid">
		<label for="filename"> Content </label>

		<textarea bind:value={content} class="text-black" />
	</div>
	<button on:click={createFile}> Create file </button>
</div>

<a href="/file"> Go to</a>
