<script>
	import { goto } from '$app/navigation';
	import { Button } from '../ui/button';
	import { Trash, Eye, Edit } from 'lucide-svelte';
	export let filename = '';

	const navigate = () => goto(`/file/${filename}`);
</script>

<div class="flex justify-between mb-2 items-center">
	<Button variant="link" on:click={navigate}>
		{filename}
	</Button>
	<div class="flex gap-2">
		<Button size="icon" variant="outline" on:click={navigate}><Eye class="h-4 w-4" /></Button>
		<Button size="icon" variant="outline"><Edit class="h-4 w-4" /></Button>
		<Button size="icon" variant="outline" class="text-red-500 hover:bg-red-50 hover:text-red-500">
			<Trash class="h-4 w-4" />
		</Button>
	</div>
</div>
<hr />
