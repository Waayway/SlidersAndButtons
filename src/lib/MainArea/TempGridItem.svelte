<script lang="ts">
  import Button from "./Button.svelte";
  import Grid from "./Grid.svelte";
  import Rotator from "./Rotator.svelte";
  import Slider from "./Slider.svelte";

  import offset from "mouse-event-offset";
  import type { GridItem } from "./types";
  import type { PanZoom } from "panzoom";

  export let data: GridItem;
  export let gridSize: number;
  export let gridContainer: HTMLDivElement;
  export let callback: (gridItem: GridItem) => void;
  export let panzoomInstance: PanZoom;

  let pos = {
    x: -100,
    y: -100,
  };

  const mouseMoveEvent = (ev: MouseEvent) => {
    let mousePos = offset(ev, ev.currentTarget);
    let scale = parseFloat(
      (ev.currentTarget as HTMLDivElement).style.transform
        .replace("matrix(", "")
        .split(",")[0]
    );
    pos.x = mousePos[0] * 1/scale - gridSize / 2;
    pos.y = mousePos[1] * 1/scale - gridSize / 2;
  };
  const mouseDownEvent = (ev: MouseEvent) => {
    let gridPos = {
      x: Math.floor(ev.offsetX / gridSize),
      y: Math.floor(ev.offsetY / gridSize),
    };
    data.x = gridPos.x;
    data.y = gridPos.y;
    callback(data);

    gridContainer.removeEventListener("mousemove", mouseMoveEvent);
    gridContainer.removeEventListener("mousedown", mouseDownEvent);
  };
  gridContainer.addEventListener("mousemove", mouseMoveEvent);
  gridContainer.addEventListener("mousedown", mouseDownEvent);
</script>

<div class="tempItem" style={`left: ${pos.x}px; top: ${pos.y}px;`}>
  {#if data.type == "button"}
    <Button {data} {gridSize} />
  {:else if data.type == "slider"}
    <Slider {data} {gridSize} />
  {:else if data.type == "rotator"}
    <Rotator {data} {gridSize} />
  {/if}
</div>

<style lang="scss">
  .tempItem {
    position: absolute;
    pointer-events: none;
  }
</style>
