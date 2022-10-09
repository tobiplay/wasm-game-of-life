<script lang="ts">
  export let options: any[];
  export let onChange: any;
  export let id: string;
  export let bindValue: any;
  export let addClass: string;
</script>

<form class={addClass}>
  <div>
    <label
      class="block text-sm font-bold text-slate-700 tracking-tight"
      for={id}>Universe template</label
    >
    <select
      name={id}
      {id}
      bind:value={bindValue}
      on:change={onChange}
      class="border"
    >
      {#each options as option}
        <option value={option} class="tracking-tight text-sm font-medium">
          {option.text}
        </option>
      {/each}</select
    >
  </div>
</form>
