<script lang="ts">
  import AlertEditor from "../AlertEditor.svelte";
  import AlertTable from "../AlertTable.svelte";
  import Button from "../Button.svelte";
  import DirectoryContent from "../DirectoryContent.svelte";
  import { fade } from "svelte/transition";
  import type { Alert } from "../model/alert";
  import { detections$, alerts$, saveAlerts } from "../AlertManager.svelte";
  import { onDestroy } from "svelte";

  let editing = false;

  let alerts: Alert[] = [];

  $: if ($alerts$) alerts = $alerts$;

  function appendAlert(newAlert: Alert) {
    for (const [idx, alert] of alerts.entries()) {
      if (alert.name === newAlert.name) {
        alerts[idx] = newAlert;
        return;
      } 
    }
    alerts.push(newAlert);
    alerts = alerts;
  }

  function removeAlert(alert: Alert) {
    const idx = alerts.findIndex(x => x.name == alert.name);
    if (idx >= 0) {
      alerts.splice(idx, 1);
      alerts = alerts;
    }
  }

  let editing_alert: Alert | undefined = undefined;

  function saveChanges() {
    if (editing) {
      saveAlerts(alerts);
    }
  }

  function toggleEditing() {
    saveChanges();
    editing = !editing;
  }

  onDestroy(saveChanges);
</script>

<div id="main">
  {#key editing}
    {#if !editing}
      <h2>{$detections$.length} {$detections$.length == 1 ? "file has" : "files have"} triggered the alerts</h2>
      <div in:fade id="content">
        <DirectoryContent childs={$detections$} />
      </div>
    {:else}
      <h2>Editing current set of alerts</h2>
      <div id="editor">
        <div id="content">
          <AlertTable {alerts}
            on:select={ev => editing_alert = ev.detail}
            on:remove={ev => removeAlert(ev.detail)}
          />
        </div>
        <div>
          <AlertEditor alert={editing_alert} on:produce={ev => appendAlert(ev.detail)} />
        </div>
      </div>
    {/if}
  {/key}

  <footer>
    <Button on:click={toggleEditing}>{!editing ? "Edit Active Alerts" : "Finish Editing"}</Button>
  </footer>
</div>

  
<style>
  h2 {
    margin: 0;
    margin-bottom: 1em;
    text-align: center;
  }

  #main {
    display: flex;
    align-content: center;
    flex-direction: column;
    justify-content: space-between;
    height: 100%;
  }

  #content {
    height: 100%;
    overflow-y: auto;
  }

  footer {
    margin-top: 1em;
    display: flex;
    justify-content: center;
    align-items: center;
  }

  #editor {
    display: flex;
    flex-direction: row;
    justify-content: center;
    align-items: start;
    width: 100%;
    height: 100%;
  }

  #editor > * {
    text-align: center;
    flex: 1 1 0;
  }
</style>