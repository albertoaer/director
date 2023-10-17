<script lang="ts">
  import { getMatches } from "@tauri-apps/api/cli";
  import { invoke } from "@tauri-apps/api/tauri";
  import { listen } from "@tauri-apps/api/event";
  import { message } from "@tauri-apps/api/dialog";
  import type { FSEvent, Route } from "./model";
  import { DirectoryContext } from "./directory_context";

  let context = DirectoryContext.getOrSet();
  context.setNavigate(navigate);

  let route: Route | undefined;
  
  listen<FSEvent>("updated-entry", (event) => {
    console.log(event)
    if (event.payload.entry?.path == route?.path && route?.path) {
      context.pushChilds(event.payload.entry?.childs);
    }
  });
  
  async function navigateCore(directory: string) {
    try {
      route = await invoke<Route>('request_directory', { directory });
      context.pushRoute(route);
    } catch (err: any) {
      message(err.toString(), { type: 'error' });
    }
  }

  async function navigate(value: string | null | boolean | string[]) {
    await navigateCore(value ? value.toString() : "");
    // at this point the route is ensured
    history.pushState({route}, '', '');
  }

  function handlePopState(event: PopStateEvent) {
    if (event.state?.route?.path)
      navigateCore(event.state.route.path);
  }

  getMatches().then(async matches => {
    await navigate(matches.args['route'].value);
  });
</script>

<svelte:window on:popstate={handlePopState} ></svelte:window>

{#if route}
  <slot></slot>
{/if}