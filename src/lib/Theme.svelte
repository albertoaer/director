<script lang="ts" context="module">
  import { getCurrent } from "@tauri-apps/api/window";
  import { writable, type Writable } from "svelte/store";

  let theme$: Writable<string> = writable('dark');

  export function setTheme(theme: string) {
    theme$.set(theme);
  }

  getCurrent().onThemeChanged(ev => {
    theme$.set(ev.payload);
  })

  getCurrent().theme().then(theme => {
    if (theme)
      theme$.set(theme);
  });
</script>

<svelte:head>
  <link rel="stylesheet" href="/themes/{$theme$}.css">
</svelte:head>