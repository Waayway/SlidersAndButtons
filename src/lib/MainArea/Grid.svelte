<script lang="ts">
  import GridItem from "./GridItem.svelte";
  import type { GridItem as GridItemType, GridItemTemplate } from "./types";
  import { onMount } from "svelte";
  import panzoom, { type PanZoom } from "panzoom";
  import TempGridItem from "./TempGridItem.svelte";
  import { invoke } from "@tauri-apps/api/tauri";

  let items: GridItemType[] = [];
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

  onMount(async () => {
    panzoomInstance = panzoom(gridContainer, {
      minZoom: 0.5,
      maxZoom: 2,
      bounds: true,
      boundsPadding: 1,
      onDoubleClick: (e) => {
        return false;
      },
      beforeWheel: (e) => {
        let shouldIgnore = Boolean((e.target as HTMLElement).closest(".stop-scroll"));
        return shouldIgnore;
      }
    });
    panzoomInstance.moveTo(-5000, -2500);
    items = await invoke("get_grid_config");
  });

  const tempItemToItem = (gridItem: GridItemType) => {
    tempItem = null;
    items.push(gridItem);
    items = items;
  };

  export const createNewItem = (type: string) => {
    tempItem = structuredClone(itemTemplates[type]);
  };

  export const saveItems = () => {
    console.log(items[0].data);
    invoke("save_grid_config", { gridItems: items });
  };

  const deleteItemFromItems = (gridData: GridItemType) => {
    items = items.filter((obj) => obj !== gridData);
  };
</script>

<div class="overflow-container" bind:this={overflowContainer}>
  <div class="grid-container" bind:this={gridContainer}>
    {#each items as item, i (item)}
      <GridItem
        gridData={item}
        {gridSize}
        deleteSelfFromItems={deleteItemFromItems}
        itemIndex={i}
      />
    {/each}
    {#if tempItem}
      <TempGridItem
        data={tempItem}
        {gridSize}
        {gridContainer}
        callback={tempItemToItem}
      />
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
