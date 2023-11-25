<script lang="ts">
  import { startup$, addStartup, removeStartup, rerunStartup } from "../SettingsManager.svelte";
  import { open } from "@tauri-apps/api/dialog";
  import AddRemoveList from "../AddRemoveList.svelte";
  import ReloadIcon from '@iconify/icons-mdi/reload';
  import Button from "../Button.svelte";
  import Icon from "@iconify/svelte";

  async function tryAdd() {
    const result = await open({
      directory: true,
      multiple: false,
      recursive: false,
      title: 'Open Directory'
    });
    if (result && typeof result === 'string')
      addStartup(result);
  }
</script>

<div id="header">
  <h2>Scan on startup</h2>
  <div id="buttons">
    <Button on:click={rerunStartup}><Icon icon={ReloadIcon} /></Button>
  </div>
</div>
<div id="container">
  <AddRemoveList
    items={$startup$}
    on:add={tryAdd}
    on:remove={ev => removeStartup(ev.detail)}
  >
  </AddRemoveList>
</div>

<style>
  #header {
    display: flex;
    flex-direction: row;
    align-items: center;
    gap: 1em;
  }

  #buttons {
    font-size: 1.3em;
  }

  #container {
    width: 75%;
    margin: auto;
  }
</style>