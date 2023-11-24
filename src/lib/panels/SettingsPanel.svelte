<script lang="ts">
  import { startup$, add_startup, remove_startup } from "../SettingsManager.svelte";
  import { open } from "@tauri-apps/api/dialog";
  import AddRemoveList from "../AddRemoveList.svelte";

  async function tryAdd() {
    const result = await open({
      directory: true,
      multiple: false,
      recursive: false,
      title: 'Open Directory'
    });
    if (result && typeof result === 'string')
      add_startup(result);
  }
</script>

<h2>Scan on startup</h2>
<div id="container">
  <AddRemoveList
    items={$startup$}
    on:add={tryAdd}
    on:remove={ev => remove_startup(ev.detail)}
  >
  </AddRemoveList>
</div>

<style>
  #container {
    width: 75%;
    margin: auto;
  }
</style>