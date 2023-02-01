<script lang="ts">
  import GridItem from "./GridItem.svelte";
  import type { GridItem as GridItemType, GridItemTemplate } from "./types";
  import { onMount } from "svelte";
  import panzoom, { type PanZoom } from "panzoom";
  import TempGridItem from "./TempGridItem.svelte";

  let items: Array<GridItemType> = [];
  let tempItem: GridItemType;

  let itemTemplates: GridItemTemplate = {
    button: {
      x: 0,
      y: 0,
      width: 1,
      height: 1,
      type: "button",
      data: {},
    },
    slider: {
      x: 0,
      y: 0,
      width: 1,
      height: 3,
      type: "slider",
      data: {},
    },
    rotator: {
      x: 0,
      y: 0,
      width: 1,
      height: 1,
      type: "rotator",
      data: {},
    },
  };

  let gridSize = 50;
  let gridTotalSize = {
    x: 10000 / gridSize,
    y: 5000 / gridSize,
  };
  let overflowContainer: HTMLDivElement;
  let gridContainer: HTMLDivElement;
  let panzoomInstance: PanZoom;

  onMount(() => {
    panzoomInstance = panzoom(gridContainer, {
      minZoom: 0.5,
      maxZoom: 2,
      bounds: true,
      boundsPadding: 1,
      onDoubleClick: (e) => {
        return false;
      },
    });
    panzoomInstance.moveTo(-5000, -2500);
  });
  const tempItemToItem = (gridItem: GridItemType) => {
    tempItem = undefined;
    items.push(gridItem);
    items = items;
  }

  export const createNewItem = (type: string) => {
    tempItem = itemTemplates[type];
  };
  
</script>

<div class="overflow-container" bind:this={overflowContainer}>
  <div class="grid-container" bind:this={gridContainer}>
    {#each items as item, i (i)}
      <GridItem gridData={item} {gridSize} />
    {/each}
    {#if tempItem}
      <TempGridItem data={tempItem} {gridSize} {gridContainer} callback={tempItemToItem} {panzoomInstance}/>
    {/if}
  </div>
</div>

<style lang="scss">
  .overflow-container {
    width: 100%;
    height: 100%;
    overflow: hidden;
    position: relative;
    .grid-container {
      width: 10000px;
      height: 5000px;
      $lineClr: rgba(100, 100, 100, 0.7);
      background-image: linear-gradient($lineClr 0.1em, transparent 0.1em),
        linear-gradient(90deg, $lineClr 0.1em, transparent 0.1em);
      background-size: 50px 50px;
    }
  }
</style>
