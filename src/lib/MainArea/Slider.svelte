<script lang="ts">
  import type { GridItem } from "./types";

  export let data: GridItem;
  export let gridSize: number;

  let width = gridSize * 0.8;
  let heigth = data.height * gridSize - (gridSize *0.2);
  let margins = gridSize * 0.1 + 1;
</script>

<div
  class="outer"
  style={`width: ${width}px; height: ${heigth}px; margin: ${margins}px;`}
>
  <div class="inner">
    {data.data.key || ""}
  </div>
</div>

<style lang="scss">
  @use "../../styles/config/config.scss";
  @use "../../styles/config/util.scss";
  .outer {
    background-color: config.get-color("key-outer-bg");
    border-radius: 0.3rem;
    display: flex;
    justify-content: center;
    align-items: center;
    .inner {
      width: 40%;
      height: calc(100% - 1rem);
      border-radius: 0.3rem;
      display: flex;
      justify-content: center;
      align-items: center;
      background-color: config.get-color("key-inner-bg");
      color: util.is-color-dark(config.get-color("key-inner-bg"));
    }
  }
</style>
