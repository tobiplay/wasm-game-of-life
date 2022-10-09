<script lang="ts">
  export let toggled: boolean = false;
  export let addClass: string;
</script>

<div class="flex flex-col w-max {addClass}">
  <div class="flex flex-col w-max {addClass}">
    <label
      for="fps-counter"
      class="block text-sm font-bold text-slate-700 tracking-tight"
      >FPS counter</label
    >
    <div
      id="fps-counter"
      on:click={() => (toggled = !toggled)}
      class="pointer-events-auto h-6 w-10 rounded-full p-1 ring-1 ring-inset transition duration-200 ease-in-out {!toggled
        ? 'bg-slate-900/10 ring-slate-900/5'
        : ' bg-indigo-600 ring-black/20'} "
    >
      <div
        class="h-4 w-4 rounded-full bg-white shadow-sm ring-1 ring-slate-700/10 transition duration-200 ease-in-out {toggled
          ? 'translate-x-4'
          : ''}"
      />
    </div>
  </div>
</div>
