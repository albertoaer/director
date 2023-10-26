<script lang="ts">
  import DirectoryBreadcrumbs from "../DirectoryBreadcrumbs.svelte";
  import DirectoryContent from "../DirectoryContent.svelte";
  import { route$, childs$, navigate } from "../FSManager.svelte";

  function handleNavigate(ev: CustomEvent<{ route: string}>) {
    navigate(ev.detail.route);
  }

  function handleCalculate(ev: CustomEvent<{ route: string}>) {
    navigate(ev.detail.route, { calculate: true });
  }
</script>

{#if $route$}
  <DirectoryBreadcrumbs route={$route$} on:navigate={handleNavigate} on:calculate={handleCalculate} />
  <div id="content">
    {#if $childs$}
      <DirectoryContent childs={$childs$} on:navigate={handleNavigate} />
    {/if}
  </div>
{/if}

<style>
  #content {
    margin-top: 0.5em;
    width: 100%;
    overflow-y: auto;
    height: 100%;
  }
</style>