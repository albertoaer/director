<script lang="ts">
  export let grow = false;
  export let effect = false;
</script>

<button on:click class:grow class:effect><slot /></button>

<style>
  button {
    display: flex;
    align-items: center;
    justify-content: center;
    background-color: var(--item-color);
    color: inherit;
    font-size: 1em;
    padding: 0.3em;
    border: none;
    margin: 0;
    border-radius: 5px;
    transition: 400ms all ease;
    user-select: none;
  }

  button.effect {
    box-shadow: 0px 5px 10px 0px black;
  }

  button.grow {
    flex-grow: 1;
  }
  
  button:hover {
    background-color: var(--item-active-color);
    cursor: pointer;
    transition: 300ms all ease-out;
  }

  button.effect:hover {
    box-shadow: 0px 10px 10px 0px black;
  }

  button:active {
    background-color: var(--item-active-color);
    cursor: pointer;
    transition: 200ms all ease-out;
  }
  
  button:not(.effect):active {
    transform: scale(1.1);
  }

  button.effect:active {
    box-shadow: 0px 5px 5px 0px black;
  }
</style>