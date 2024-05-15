<script lang="ts">
	import MarkdownIt from 'markdown-it/lib/index.mjs';
	import hljs from 'highlight.js';
	import 'highlight.js/styles/default.css';

	export let markdownInput = '';

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
	$: htmlOutput = md.render(markdownInput);
</script>

<div>
	{@html htmlOutput}
</div>
