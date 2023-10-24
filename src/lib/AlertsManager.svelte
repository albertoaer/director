<script lang="ts" context="module">
  import { writable, type Writable } from "svelte/store";
  import type { Alert, AlertEvent } from "./model/alert";
  import { listen } from "@tauri-apps/api/event";
  import { invoke } from "@tauri-apps/api/tauri";
  import { message } from "@tauri-apps/api/dialog";
  import { DetectionList } from "./ds/detection_list";

  export const alerts$: Writable<Alert[] | undefined> = writable();
  export const detections$: DetectionList = new DetectionList();

  listen<AlertEvent>("alert", (event) => {
    if (event.payload.load) {
      alerts$.set(event.payload.load.alerts);
    } else if (event.payload.trigger) {
      const detection = event.payload.trigger;
      detections$.push(detection);
    }
  });

  export async function saveAlerts(alerts: Alert[]) {
    try {
      await invoke('save_alerts', { alerts });
      alerts$.set(alerts);
    } catch (err: any) {
      message(err.toString(), { type: 'error' })
    }
  }
</script>