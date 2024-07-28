<script>
  import DynamicHR from "$components/DynamicHR.svelte";
  import { invoke } from "@tauri-apps/api";

  let page = 0;

  let headerElement;
</script>

<div class="flex w-full">
  <div class="flex-1">
    <h1 bind:this={headerElement} class="w-fit text-2xl">Addons</h1>
    <DynamicHR element={headerElement} />
    {#if page === 0}{:else}{/if}
  </div>
  <div class="flex flex-col items-center w-1/6">
    <button
      on:click={() => invoke("create_curseforge_window")}
      class="w-5/6 px-2 py-1 bg-neutral-900 border border-neutral-300"
    >
      Add
    </button>
  </div>
</div>
