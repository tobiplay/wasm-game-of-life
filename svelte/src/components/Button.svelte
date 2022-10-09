<script lang="ts">
  export let text: string;
  export let onClick: any;
  export let id: string;
  export let type: any;
</script>

{#if type == "primary"}
  <button
    class="inline-flex justify-center rounded-lg text-sm font-semibold py-3 px-4 bg-indigo-700 text-white hover:bg-indigo-900"
    on:click={onClick}
    {id}>{text}</button
  >
{:else}
  <button
    class="rounded-lg text-sm font-semibold py-3 px-4 bg-white/0 text-slate-900 ring-1 ring-slate-900/10 hover:bg-slate-400/10 hover:ring-slate-900/15"
    on:click={onClick}
    {id}>{text}</button
  >
{/if}
