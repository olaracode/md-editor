<script lang="ts">
     import { invoke } from '@tauri-apps/api/tauri'
    let name = '';
    let greetMsg = ''
     async function onClick(){
        greetMsg = await invoke('greet', {name}); 
     }
</script>

<h1>New tauri</h1>
<input type="text" bind:value={name}>
<button on:click={onClick}>
    Saluda
</button>

{#if greetMsg != ''}
    <p>
        {greetMsg}
    </p>
{/if}