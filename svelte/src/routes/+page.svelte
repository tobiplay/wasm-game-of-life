<script lang="ts">
    import { onMount } from 'svelte';
    import init, {greet} from "wasm-game-of-life";
        
    onMount(async () => {
        init();
    })
</script>

<h1>Welcome to SvelteKit</h1>
<p>Visit <a href="https://kit.svelte.dev">kit.svelte.dev</a> to read the documentation</p>
<button on:click={() => greet()}>Click</button>
