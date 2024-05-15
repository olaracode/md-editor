<script lang="ts">
	import { page } from '$app/stores';
	import * as Breadcrumb from '$lib/components/ui/breadcrumb/index.js';
	import type { BreadCrumbT } from '../../types';

	let breadcrumbs: BreadCrumbT[] = [];

	$: {
		const parts = $page.url.pathname.split('/').filter(Boolean);
		breadcrumbs = parts.map((part: string, index: number) => {
			return {
				name: part,
				path: '/' + parts.slice(0, index + 1).join('/')
			};
		});
	}
</script>

<Breadcrumb.Root>
	<Breadcrumb.List>
		<Breadcrumb.Item>
			<Breadcrumb.Link href="/">Home</Breadcrumb.Link>
		</Breadcrumb.Item>
		<Breadcrumb.Separator />
		{#each breadcrumbs as { name, path }, index}
			{#if index + 1 == breadcrumbs.length}
				<Breadcrumb.Item class="font-bold">{name}</Breadcrumb.Item>
			{:else}
				<Breadcrumb.Item><Breadcrumb.Link href={path}>{name}</Breadcrumb.Link></Breadcrumb.Item>
				<Breadcrumb.Separator />
			{/if}
		{/each}
	</Breadcrumb.List>
</Breadcrumb.Root>
