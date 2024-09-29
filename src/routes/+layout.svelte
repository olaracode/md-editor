<script lang="ts">
	import '../main.css';
	import Navbar from '$lib/components/Navbar.svelte';
	import { Toaster } from '$lib/components/ui/sonner';
</script>

<Toaster />
<Navbar />
<div class="overflow-y-hidden py-5 lg:max-w-[70%] mx-auto dark">
	<slot />
</div>
