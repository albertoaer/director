<script lang="ts" context="module">
	import { Chart, registerables } from "chart.js";

  Chart.register(...registerables);
</script>

<script lang="ts">
  import type { ChartItem, ChartData, ChartType, ChartOptions, Plugin } from 'chart.js';

  export let type: ChartType;
  export let data: ChartData;
  export let options: ChartOptions | undefined = undefined;
  export let plugins: Plugin[] | undefined = undefined;

  export let chart: Chart | undefined;

  function createChart(item: ChartItem) {
    Chart.defaults.font.size = 18;

    chart = new Chart(item, { type, data, options, plugins });
    
    return {
      destroy() {
        chart?.destroy();
      }
    }
  }

  $: if (chart) {
    chart.config.data = data;
    chart.config.options = options;
    chart.update();
  }
</script>

{#key type}
  <canvas use:createChart></canvas>
{/key}

<style>
  canvas {
    height: 100%;
    width: 100%;
  }
</style>