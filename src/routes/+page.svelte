<script lang="ts">
	export let data;
</script>

<h1 class="text-center">Notionless files</h1>

<div class="m-5 p-5 bg-slate-900">
	<ul>
		{#each data?.files as file}
			<li>
				{file.filename}
			</li>
		{/each}
	</ul>
</div>
