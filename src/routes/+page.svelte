<script lang="ts">
  import { onMount } from "svelte";

  import { invoke } from "@tauri-apps/api";

  let gameVersion: string;

  let headerElement: any;
  let lineWidth = 0;

  onMount(async () => {
    gameVersion = await invoke("get_game_version");

    lineWidth = headerElement.offsetWidth;
  });
</script>

<h1 bind:this={headerElement} class="w-fit text-2xl">World of Warcraft</h1>
<p class="ml-1 text-sm leading-tight">v{gameVersion}</p>
<hr class="mt-1" style="width: {lineWidth * 1.25}px;" />
