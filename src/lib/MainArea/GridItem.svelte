<script lang="ts">
  import Button from "./Button.svelte";
  import Rotator from "./Rotator.svelte";
  import Slider from "./Slider.svelte";
  import type { GridItem } from "./types";
  export let gridData: GridItem;
  export let gridSize: number;
  let margins = {
    x: gridData.x * gridSize,
    y: gridData.y * gridSize,
  };
</script>

<div style={`top: ${margins.y}px; left: ${margins.x}px;`}>
  {#if gridData.type == "button"}
    <Button data={gridData} {gridSize} />
  {:else if gridData.type == "slider"}
    <Slider data={gridData} {gridSize} />
  {:else if gridData.type == "rotator"}
    <Rotator data={gridData} {gridSize} />
  {/if}
</div>

<style lang="scss">
  div {
    position: absolute;
  }
</style>
