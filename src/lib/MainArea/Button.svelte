<script lang="ts">
  import type { GridItem } from "./types";

  export let data: GridItem;
  export let gridSize: number;

  let width = gridSize * 0.8;
  let heigth = data.height * gridSize - (gridSize *0.2);
  let margins = gridSize * 0.1 + 1;

  let dragBegin = (ev) => {};
  let onDrag = (ev) => {};
  let dragEnd = (ev) => {};
</script>

<div class="draggable">
  <div
    class="outer"
    style={`width: ${width}px; height: ${heigth}px; margin: ${margins}px;`}
  >
    <div class="inner">
      {data.data.key || ""}
    </div>
  </div>
</div>

<style lang="scss">
  @use "../../styles/config/config.scss";
  @use "../../styles/config/util.scss";
  .draggable {
    .outer {
      background-color: config.get-color("key-outer-bg");
      border-radius: 0.3rem;
      display: flex;
      justify-content: center;
      align-items: center;
      .inner {
        width: 80%;
        height: 80%;
        border-radius: 0.3rem;
        display: flex;
        justify-content: center;
        align-items: center;
        background-color: config.get-color("key-inner-bg");
        color: util.is-color-dark(config.get-color("key-inner-bg"));
      }
    }
  }
</style>
