<script lang="ts" context="module">
  import { writable, type Writable } from "svelte/store";
  import type { Alert, Detection } from "./model/alert";
  import { listen } from "@tauri-apps/api/event";
  import { invoke } from "@tauri-apps/api/tauri";
  import { message } from "@tauri-apps/api/dialog";
  import { DetectionList } from "./ds/detection_list";

  export const alerts$: Writable<Alert[] | undefined> = writable();
  export const detections$: DetectionList = new DetectionList();

  invoke<Alert[]>("request_alerts").then(alerts$.set);

  listen<Alert[]>("load-alerts", (event) => alerts$.set(event.payload));

  export async function saveAlerts(alerts: Alert[]) {
    try {
      const refresh = await invoke<boolean>('save_alerts', { alerts });
      if (refresh) detections$.clear();
      alerts$.set(alerts);
    } catch (err: any) {
      message(err.toString(), { type: 'error' })
    }
  }

  const intervalTime = 250;

  setInterval(async () => {
    const result = await invoke<Detection[]>("get_detections", {
      begin: detections$.count,
      count: 300
    });
    detections$.push(...result);
  }, intervalTime);
</script>