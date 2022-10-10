<script lang="ts">
  import Switch from "./Switch.svelte";
  import { hidden } from "../lib/stores.js";

  export let handleGridSizeChange: any;
  export let handleUniverseOptionChange: any;

  import { ticksPerFrame, gridSize, universeTemplate } from "../lib/stores.js";
</script>

<div
  class="{$hidden
    ? 'hidden'
    : ''} absolute w-[calc(100vw-1rem)] mr-0 sm:w-56 mt-4 sm:mt-2 sm:mr-2 bg-gray-50 bg-opacity-75 backdrop-filter backdrop-blur-sm border border-gray-100 rounded-lg shadow-lg divide-y divide-gray-100 focus:outline-none right-0 top-0"
  on:mouseleave={() => {
    $hidden = true;
  }}
>
  <div class="py-2" role="none">
    <div class="flex flex-col px-4 py-3 w-full space-y-1">
      <label
        for="universe-template"
        class="font-mono uppercase text-xs text-slate-700 tracking-widest"
        >Universe template</label
      >
      <select
        id="universe-template"
        class="w-full bg-transparent p-1 border-slate-300 border-2 uppercase font-mono text-xs tracking-widest rounded-md focus:border-indigo-600 focus:outline-none"
        bind:value={$universeTemplate}
        on:change={handleUniverseOptionChange}
      >
        <option>Random</option>
        <option>Empty</option>
        <option>TwoSeven</option>
      </select>
    </div>
    <div class="flex flex-col px-4 py-3 w-full space-y-1">
      <label
        for="ticks-per-frame"
        class="font-mono uppercase text-xs text-slate-700 tracking-widest"
        >{$ticksPerFrame}
        {$ticksPerFrame > 1 ? "Ticks" : "Tick"} per frame</label
      >
      <input
        bind:value={$ticksPerFrame}
        type="range"
        id="ticks-per-frame"
        min="1"
        max="10"
        class="w-full accent-indigo-600"
      />
    </div>
    <div class="flex flex-col px-4 py-3 w-full space-y-1">
      <label
        for="grid-size"
        class="font-mono uppercase text-xs text-slate-700 tracking-widest"
        >{$gridSize}x{$gridSize} cells</label
      >
      <input
        on:change={handleGridSizeChange}
        bind:value={$gridSize}
        type="range"
        id="grid-size"
        min="16"
        max="128"
        step="16"
        class="w-full accent-indigo-600"
      />
    </div>

    <hr class="mt-2 pb-2 border-gray-200" />
    <div class="flex flex-col px-4 py-3 w-full space-y-1">
      <label
        for="fps-counter"
        class="font-mono uppercase text-xs text-slate-700 tracking-widest"
        >FPS counter</label
      >
      <Switch addClass="" />
    </div>
  </div>
</div>
