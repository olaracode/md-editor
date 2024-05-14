<script lang="ts">
	// @ts-ignore
	import MarkdownIt from 'markdown-it';
	import hljs from 'highlight.js';
	import 'highlight.js/styles/default.css';
	export let data;
	let { file } = data;

	let isPreview = true;
	function setPreview() {
		isPreview = !isPreview;
	}

	$: isEdited = markdownInput != file?.content;

	let md = new MarkdownIt({
		highlight: function (str: string, lang: string) {
			if (lang && hljs.getLanguage(lang)) {
				try {
					return hljs.highlight(str, { language: lang }).value;
				} catch (__) {}
			}

			return ''; // use external default escaping
		}
	});

	let markdownInput = file ? file.content : '';
	let htmlOutput = file ? file.content : '';
	// @ts-ignore
	$: htmlOutput = md.render(markdownInput);
</script>

<div class="mx-5 h-[80vh]">
	{#if file}
		<div class="md:flex justify-between">
			<div>
				<h1 class="text-lg">
					File: <span class="font-bold">{file.filename}</span>
					{#if isEdited}
						<span class="text-red-500 text-sm"> (edited) </span>
					{/if}
				</h1>
			</div>
			<div class="flex">
				<button on:click={setPreview}>
					<span class={`${isPreview ? 'text-amber-600' : 'text-emerald-500'} hover:underline`}>
						Change to {!isPreview ? 'Preview Mode' : 'Edit Mode'}
					</span>
				</button>
			</div>
		</div>
		{#if isPreview}
			<div class="mt-4">
				{@html htmlOutput}
			</div>
		{:else}
			<textarea bind:value={markdownInput} class="w-full h-full bg-slate-900 p-5"></textarea>
		{/if}
	{/if}
</div>
