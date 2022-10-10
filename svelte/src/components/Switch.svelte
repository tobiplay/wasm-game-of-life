<script lang="ts">
  import { showFps } from "$lib/stores";
  export let addClass: string;
</script>

<div class="flex flex-col w-max {addClass}">
  <div class="flex flex-col w-max {addClass}">
    <div
      id="fps-counter"
      on:click={() => ($showFps = !$showFps)}
      class="pointer-events-auto h-6 w-10 rounded-full p-1 ring-1 ring-inset transition duration-200 ease-in-out {!$showFps
        ? 'bg-slate-900/10 ring-slate-900/5'
        : ' bg-indigo-600 ring-black/20'} "
    >
      <div
        class="h-4 w-4 rounded-full bg-white shadow-sm ring-1 ring-slate-700/10 transition duration-200 ease-in-out {$showFps
          ? 'translate-x-4'
          : ''}"
      />
    </div>
  </div>
</div>
