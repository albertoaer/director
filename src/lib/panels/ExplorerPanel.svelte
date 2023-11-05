<script lang="ts">
  import Icon from "@iconify/svelte";
  import DirectoryBreadcrumbs from "../DirectoryBreadcrumbs.svelte";
  import DirectoryContent from "../DirectoryContent.svelte";
  import { route$, childs$, navigate } from "../FSManager.svelte";
  import ItemButton from "../ItemButton.svelte";
  import CalculateFolderIcon from "@iconify/icons-mdi/scale-unbalanced";
  import ChartIcon from "@iconify/icons-mdi/chart-pie";
  import ListIcon from "@iconify/icons-mdi/list-box-outline";
  import DirectoryChart from "../DirectoryChart.svelte";

  function handleNavigate(ev: CustomEvent<{ route: string}>) {
    navigate(ev.detail.route);
  }

  let route = $route$;
  $: route = $route$;

  let chart = false;

  function toggleChart() {
    chart = !chart;
  }

  function calculateFolder() {
    if (!route) return;
    navigate(route.path, { calculate: true });
  }
</script>

{#if $route$}
  <DirectoryBreadcrumbs route={$route$} on:navigate={handleNavigate}>
    <ItemButton
      on:click={calculateFolder}
      tooltip={{ content: 'calculate folder', singleton: 'dir-bar' }}
    >
      <Icon icon={CalculateFolderIcon} inline />
    </ItemButton>
    <ItemButton
      on:click={toggleChart}
      tooltip={{ content: 'toggle chart', singleton: 'dir-bar' }}
    >
      <Icon icon={chart ? ListIcon : ChartIcon} inline />
    </ItemButton>
  </DirectoryBreadcrumbs>
  <div id="content">
    {#if $childs$}
      <svelte:component this={chart ? DirectoryChart : DirectoryContent} childs={$childs$} on:navigate={handleNavigate} />
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