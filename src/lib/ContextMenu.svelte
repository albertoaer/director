<script lang="ts" context="module">
  export type ContextMenuEvent = (ev: MouseEvent) => void;
  export type ContextMenuPayload = {
    id: string
  };

  export function ctxPayload(element: HTMLElement, payload: ContextMenuPayload) {
    (element as any).payload = payload;
  }

  function getPayload(element: HTMLElement | EventTarget | null): ContextMenuPayload | null {
    if (!element) return null;
    return (element as any).payload ?? null;
  }
</script>

<script lang="ts">
  let menu: HTMLElement;
  let activeElement: EventTarget | null = null;

  function show(x: number, y: number) {
    menu.style.left = x + 'px';
    menu.style.top = y + 'px';
    menu.style.display = 'table';
    menu.focus();
  };

  function contextMenu(ev: MouseEvent) {
    ev.preventDefault();
    activeElement = ev.currentTarget;
    show(ev.clientX, ev.clientY);
  }

  function blur() {
    menu.style.display = 'none';
    activeElement = null;
  }
</script>

<!-- svelte-ignore a11y-no-noninteractive-tabindex -->
<!-- svelte-ignore a11y-click-events-have-key-events -->
<!-- svelte-ignore a11y-no-noninteractive-element-interactions -->
<table id="menu" tabindex="0" bind:this={menu} on:blur={blur} on:click|stopPropagation={blur}>
  <slot name="menu" {activeElement} payload={getPayload(activeElement)} />
</table>
<slot {show} {contextMenu} {activeElement} payload={getPayload(activeElement)} />

<style>
  #menu {
    position: absolute;
    display: none;
    background-color: var(--nav-bar-color);
    padding: 0.2em;
    border-radius: 2px;
    outline: none;
  }
</style>