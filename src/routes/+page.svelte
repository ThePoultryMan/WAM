<script lang="ts">
  import { onMount } from "svelte";

  import { invoke } from "@tauri-apps/api";
  import DynamicHR from "$components/DynamicHR.svelte";

  let gameVersion: string;

  let headerElement;

  onMount(async () => {
    gameVersion = await invoke("get_game_version");
  });
</script>

<h1 bind:this={headerElement} class="w-fit text-2xl">World of Warcraft</h1>
<p class="ml-1 text-sm leading-tight">v{gameVersion}</p>
<DynamicHR element={headerElement} />
