<script lang="ts">
  import { navigate } from "./FSManager.svelte";

  function handlePopState(event: PopStateEvent) {
    if (event.state?.route?.path)
      navigate(event.state.route.path, { historyRecord: false });
  }
</script>

<svelte:window on:popstate={handlePopState} ></svelte:window>