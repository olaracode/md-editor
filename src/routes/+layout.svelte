<script lang="ts">
	import '../main.css';
	import Navbar from '$lib/components/Navbar.svelte';
</script>

<Navbar />
<div class="py-20">
	<slot />
</div>
