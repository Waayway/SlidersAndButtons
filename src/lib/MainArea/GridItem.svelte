<script lang="ts">
  import Button from "./Button.svelte";
  import ItemPopup from "./ItemPopup.svelte";
  import Rotator from "./Rotator.svelte";
  import Slider from "./Slider.svelte";
  import type { GridItem } from "./types";
  export let gridData: GridItem;
  export let gridSize: number;
  export let deleteSelfFromItems: (gridData: GridItem) => void;
  export let itemIndex: number;

  import { itemOpen } from "../../stores";

  let margins = {
    x: gridData.x * gridSize,
    y: gridData.y * gridSize,
  };

  let isPopupActive = false;

  itemOpen.subscribe((itemNum) => {
    isPopupActive = itemIndex == itemNum;
  })

  const openPopup = (ev: MouseEvent) => {
    if (
      (ev.target as HTMLDivElement).classList.contains("draggable") ||
      (ev.target as HTMLDivElement).classList.contains("outer") ||
      (ev.target as HTMLDivElement).classList.contains("inner")
    ) {
      if (!isPopupActive) {
        itemOpen.set(itemIndex);
      } else {
        itemOpen.set(-1);
      }
    }
  };

  const updateKey = (key: string) => {
    gridData.data.key = key;
  }

  const deleteSelf = () => {
    deleteSelfFromItems(gridData);
    itemOpen.set(-1);
  }

</script>

<button
  style={`top: ${margins.y}px; left: ${margins.x}px;`}
  on:click={openPopup}
>
  {#if gridData.type == "button"}
    <Button data={gridData} {gridSize} />
  {:else if gridData.type == "slider"}
    <Slider data={gridData} {gridSize} />
  {:else if gridData.type == "rotator"}
    <Rotator data={gridData} {gridSize} />
  {/if}
  <ItemPopup {gridData} active={isPopupActive} {updateKey} {deleteSelf}/>
</button>

<style lang="scss">
  button {
    position: absolute;
    margin: 0;
    padding: 0;
    background-color: transparent;
    border: 0;
  }
</style>
