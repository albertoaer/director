<script lang="ts">
  const maxFontSize = 1.8;
  const minFontSize = 0.6;

  let fontSize = (maxFontSize + minFontSize) / 2;

  const sizeUnits: [string, number][] = [['B', 0], ['KB', 3], ['MB', 6], ['GB', 9], ['TB', 12]];
  let sizeUnitsIdx = 0;

  function updateFontSize(value: number, allow: boolean) {
    if (value == 0 || !allow) return;
    const mov = value / -Math.abs(value);
    fontSize = Math.min(Math.max(minFontSize, fontSize + (mov * 0.2)), maxFontSize);
    console.log(fontSize);
  }

  function updateSizeUnit() {
    sizeUnitsIdx = (sizeUnitsIdx + 1) % sizeUnits.length;
  }
</script>

<!-- svelte-ignore a11y-no-noninteractive-element-interactions -->
<table
  on:wheel={ev => updateFontSize(ev.deltaY, ev.ctrlKey)}
  style="--font-size: {fontSize}em"
>
  <tr>
    <th>Name</th>
    <th class="action" on:click={_ => updateSizeUnit()}>Size ({sizeUnits[sizeUnitsIdx][0]})</th>
    <th>Modified</th>
    <th>Created</th>
  </tr>
  <slot sizeUnit={{ symbol: sizeUnits[sizeUnitsIdx][0], factor: 1 / Math.pow(10, sizeUnits[sizeUnitsIdx][1])}} />
</table>

<style>
  table {
    width: 100%;
    text-align: left;
    border-spacing: 0px;
    border-collapse: collapse;
    table-layout: fixed;
    font-size: var(--font-size);
  }

  tr {
    position: sticky;
    background-color: var(--background-color);
    top: 0;
  }

  th {
    padding: 0.3em 0;
    user-select: none;
  }
  
  th.action:hover {
    text-decoration: underline;
    cursor: pointer;
  }
</style>