<script lang="ts" context="module">
  export type ContextMenuEvent<T> = (element: T, x: number, y: number, event?: Event) => void;
</script>

<script lang="ts" generics="T">
  export let childs: T[];
  let menu: HTMLElement;
  let activeElement: T;
  
  function show(x: number, y: number) {
    menu.style.left = x + 'px';
    menu.style.top = y + 'px';
    menu.style.display = 'table';
    menu.focus();
  };

  const contextMenu: ContextMenuEvent<T> = (element: T, x: number, y: number, event?: Event) => {
    activeElement = element;
    event?.preventDefault();
    show(x, y);
  }

  function blur() {
    menu.style.display = 'none';
  }
</script>

<!-- svelte-ignore a11y-no-noninteractive-tabindex -->
<!-- svelte-ignore a11y-click-events-have-key-events -->
<!-- svelte-ignore a11y-no-noninteractive-element-interactions -->
<table id="menu" tabindex="0" bind:this={menu} on:blur={blur} on:click|stopPropagation={blur}>
  {#if activeElement} <!-- Ensure not null, weird intellisense resolution using null optional in type -->
    <slot name="menu" payload={activeElement} {childs} />
  {/if}
</table>
<slot {show} {contextMenu}/>

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