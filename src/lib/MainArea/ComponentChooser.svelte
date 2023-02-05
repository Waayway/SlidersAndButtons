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
  @use "../../styles/config/config.scss";
  @use "../../styles/config/util.scss";
  .main {
    padding: 1rem;
    display: flex;
    flex-direction: column;
    gap: 1rem;
    button {
      outline: none;
      border: none;
      background-color: config.get-color("sidebar-btn-bg");
      color: util.is-color-dark(config.get-color("sidebar-btn-bg"));
      padding: 1rem;
      max-width: 13rem;
      font-size: 1.4rem;
      border-radius: 1rem;
    }
  }
</style>
