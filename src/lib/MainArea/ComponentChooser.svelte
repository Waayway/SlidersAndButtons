<script lang="ts">
  import type { Component } from "./types";

  export let addToGridCallback: ( type: "slider" | "button" | "rotator") => void;

  let components: Array<Component> = [
    {
      name: "Button",
      type: "button",
      inputs: {
        key: true,
      },
    },
    {
      name: "Slider",
      type: "slider",
      inputs: {},
    },
    {
      name: "Potmeter",
      type: "rotator",
      inputs: {},
    },
  ];

  let onComponentAdd = (ev) => {
    addToGridCallback(ev.target.value);
  }
</script>

<div class="main">
  <h3>Components to add to grid:</h3>
  {#each components as component}
    <button on:click={onComponentAdd} value={component.type}>{component.name}</button>
  {/each}
</div>

<style lang="scss">
  .main {
    padding: 1rem;
  }
</style>
