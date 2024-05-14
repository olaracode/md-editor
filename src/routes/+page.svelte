<script lang="ts">
	import { goto } from '$app/navigation';
	import { files } from '../stores/files';
	let systemFiles = $files;
</script>

<h1 class="text-center">Notionless files</h1>

<div class="m-5 p-5 bg-slate-900">
	{#each systemFiles as file}
		<button
			type="button"
			class="w-full text-start border p-2 border-slate-400 rounded-md"
			on:click={() => goto(`/file/${file.filename}`)}
		>
			<h2>
				{file.filename}
			</h2>
		</button>
	{/each}
</div>
