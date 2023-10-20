<script lang="ts" context="module">
  import { writable, type Writable } from "svelte/store";
  import type { Alert, AlertEvent, Detection } from "./model/alert";
  import { listen } from "@tauri-apps/api/event";
  import { invoke } from "@tauri-apps/api/tauri";
  import { message } from "@tauri-apps/api/dialog";

  export const alerts$: Writable<Alert[] | undefined> = writable();
  export const detections$: Writable<Detection[] | undefined> = writable();

  listen<AlertEvent>("alert", (event) => {
    if (event.payload.load) {
      alerts$.set(event.payload.load.alerts);
    } else if (event.payload.trigger) {
      const detection = event.payload.trigger;
      detections$.update(detections => {
        if (!detections) return [detection];
        detections.push(detection);
        return detections;
      });
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