<script lang="ts">
  import { createEventDispatcher } from "svelte";
  import type { FSChild } from "./model/fs";
  import Chart from "./Chart.svelte";
  import type { ChartData, ChartOptions } from "chart.js";

  const dispatch = createEventDispatcher<{ navigate: { route: string } }>();

  export let childs: FSChild[];
  
  let [chartData, validChilds, remain] = updateChartData(childs);
  $: [chartData, validChilds, remain] = updateChartData(childs);

  function updateChartData(childs: FSChild[]): [ChartData, FSChild[], number] {
    let validChilds = childs.filter(x => x.size.status === 'Known' || x.size.status === 'Calculated');
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
    onClick(_event, elements, _chart) {
      if (elements.length) {
        dispatch('navigate', {
          route: validChilds[elements[0].index].path
        });
      }
    },
  }
</script>

<div id="content">
  <Chart type='pie' data={chartData} {options} />
</div>

<style>
  #content {
    display: grid;
    place-items: center;
    width: 100%;
    height: 100%;
    padding: 2em;
    overflow: hidden;
  }
</style>