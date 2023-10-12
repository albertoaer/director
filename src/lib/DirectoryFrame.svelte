<script lang="ts">
  import { getMatches } from "@tauri-apps/api/cli";
  import { invoke } from "@tauri-apps/api/tauri";
  import DirectoryRoute from "./DirectoryRoute.svelte";

  let route: { name: string, path: string }[] | undefined;
  
  async function navigate(value: string | null | boolean | string[]) {
    route = await invoke('request_directory', { directory: value ? value.toString() : "" });
  }

  getMatches().then(async matches => {
    await navigate(matches.args['route'].value);
  });
</script>

{#if route}
  <DirectoryRoute {route} on:navigate={event => navigate(event.detail.route)}/>
{/if}