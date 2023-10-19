<script lang="ts">
  import AlarmsEditor from "../AlarmsEditor.svelte";
  import AlarmsTable from "../AlarmsTable.svelte";
  import Button from "../Button.svelte";
  import DirectoryContent from "../DirectoryContent.svelte";
  import type { FSChild } from "../model/fs";
  import { fade } from "svelte/transition";
  import type { Alarm } from "../model/alarm";

  let editing = false;

  let childs: FSChild[] = []
  let alarms: Alarm[] = [];

  function appendAlarm(newAlarm: Alarm) {
    for (const [idx, alarm] of alarms.entries()) {
      if (alarm.name === newAlarm.name) {
        alarms[idx] = newAlarm;
        return;
      } 
    }
    alarms.push(newAlarm);
    alarms = alarms;
  }

  function removeAlarm(alarm: Alarm) {
    const idx = alarms.findIndex(x => x.name == alarm.name);
    if (idx >= 0) {
      alarms.splice(idx, 1);
      alarms = alarms;
    }
  }

  let active_alarm: Alarm | undefined = undefined;
</script>

<div id="content">
  {#key editing}
    <div in:fade>
      {#if !editing}
        <h2>{childs.length} {childs.length == 1 ? "file has" : "files have"} set off the alarms</h2>
        <DirectoryContent {childs} />
      {:else}
        <h2>Editing current set of alarms</h2>
        <div id="editor">
          <div>
            <AlarmsTable {alarms}
              on:select={ev => active_alarm = ev.detail}
              on:remove={ev => removeAlarm(ev.detail)}
            />
          </div>
          <div>
            <AlarmsEditor alarm={active_alarm} on:produce={ev => appendAlarm(ev.detail)} />
          </div>
        </div>
      {/if}
    </div>
  {/key}

  <footer>
    <Button on:click={_ => editing = !editing}>{!editing ? "Edit Active Alarms" : "Finish Editing"}</Button>
  </footer>
</div>

  
<style>
  h2 {
    margin: 0;
    margin-bottom: 1em;
    text-align: center;
  }

  #content {
    display: flex;
    align-content: center;
    flex-direction: column;
    justify-content: space-between;
    height: calc(100% - 4em);
  }

  footer {
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