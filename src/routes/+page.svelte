<script lang="ts">
  import { onMount } from "svelte";

  import { invoke } from "@tauri-apps/api";

  let gameVersion: string;

  onMount(async () => {
    gameVersion = await invoke("get_game_version");
  });
</script>

<h1 class="text-2xl">World of Warcraft</h1>
<p class="leading-none">v{gameVersion}</p>
