<script lang="ts">
  import { onMount } from "svelte";

  import { invoke } from "@tauri-apps/api";
  import DynamicHR from "$components/DynamicHR.svelte";

  let gameVersion: string;

  let headerElement;

  let gamePath: string;

  onMount(async () => {
    gameVersion = await invoke("get_game_version");
  });

  async function setGamePath() {
    await invoke("set_game_path", { releaseType: "Retail", path: gamePath });
    await invoke("save_config");
    gameVersion = await invoke("get_game_version");
  }
</script>

<h1 bind:this={headerElement} class="w-fit text-2xl">World of Warcraft</h1>
<p class="ml-1 text-sm leading-tight">v{gameVersion}</p>
<DynamicHR element={headerElement} />
<form class="flex items-center gap-1 *:leading-none">
  <label for="game-path">Game Path:</label>
  <input id="game-path" type="text" bind:value={gamePath} class="flex-1 w-max p-1 bg-neutral-900" />
  <button on:click={setGamePath} class="ml-1 py-1 px-3 bg-neutral-900 border border-neutral-300">Set</button>
</form>
