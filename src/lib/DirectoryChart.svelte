<script lang="ts">
  import { createEventDispatcher } from "svelte";
  import type { FSChild } from "./model/fs";
  import type { Chart as ChartJS } from "chart.js";
  import Chart from "./Chart.svelte";
  import type { ChartData, ChartOptions } from "chart.js";
    import { tooltip } from "./Tooltip.svelte";

  const dispatch = createEventDispatcher<{ navigate: { route: string } }>();

  export let childs: FSChild[];
  let chart: ChartJS | undefined;
  let mutatedVisibility = false;

  let [chartData, validChilds, remain] = updateChartData(childs);
  $: [chartData, validChilds, remain] = updateChartData(childs);

  function updateChartData(childs: FSChild[]): [ChartData, FSChild[], number] {
    let validChilds = childs.filter(x => x.size.status === 'Known' || x.size.status === 'Calculated');
    restoreVisibility();
    return [
      {
        datasets: [{ data: validChilds.map(x => x.size.value as number) }],
        labels: validChilds.map(x => x.name)
      },
      validChilds,
      childs.length - validChilds.length
    ]
  }

  const options: ChartOptions = {
    plugins: {
      legend: {
        display: false
      },
    },
    onClick(_event, elements, _chart) {
      if (elements.length) {
        if (validChilds[elements[0].index].type === 'directory') {
          dispatch('navigate', {
            route: validChilds[elements[0].index].path
          });
        }
      }
    }
  }

  function restoreVisibility() {
    if (chart && mutatedVisibility) {
      mutatedVisibility = false;
      const length = chart?.data.datasets[0].data.length ?? 0;
      for (let idx = 0; idx < length; idx++)
        if (!chart?.getDataVisibility(idx))
          chart?.toggleDataVisibility(idx);
    }
  }

  function toggleItem(idx: number) {
    mutatedVisibility = true;
    chart?.toggleDataVisibility(idx);
    chart?.update();
  }
</script>

<div id="main">
  <h2 id="title">Showing {validChilds.length} out of {validChilds.length + remain} elements</h2>
  <div id="content">
    <div id="chart">
      <Chart type='pie' data={chartData} {options} bind:chart />
    </div>
    <div id="legend">
      <h3>Items</h3>
      <ul>
        {#key validChilds}
          {#each validChilds as child, idx}
            <li use:tooltip={{ content: child.size.value?.toString() + ' B' }}>
              <input type="checkbox" checked={true} on:change={_ => toggleItem(idx)}>{child.name}
            </li>
          {/each}
        {/key}
      </ul>
    </div>
  </div>
</div>

<style>
  #main {
    display: flex;
    flex-direction: column;
    justify-content: center;
    height: 100%;
    width: calc(100% - 1em);
  }
  
  #title {
    text-align: center;
  }

  #content {
    display: flex;
    overflow: hidden;
    height: 100%;
    margin: 0 0 2em 0;
  }

  #chart {
    display: flex;
    flex: 4 1;
    justify-content: center;
    align-items: center;
    min-width: 10%;
    max-width: 80%;
  }

  #legend {
    display: flex;
    text-align: center;
    flex-direction: column;
    flex: 1 1;
    justify-content: right;
    align-items: auto;
  }

  ul {
    overflow-y: auto;
    overflow-x: hidden;
    padding: 0 0.5em;
    margin: 0;
  }

  li {
    text-align: start;
    padding: 0.4em;
    list-style: none;
    text-overflow: ellipsis;
    overflow: hidden;
    white-space: nowrap;
    min-width: 2em;
    max-width: 15em;
  }
</style>