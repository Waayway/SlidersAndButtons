<script lang="ts">
  import { Pane, Splitpanes } from "svelte-splitpanes";
  import ComponentChooser from "./MainArea/ComponentChooser.svelte";
  import Grid from "./MainArea/Grid.svelte";

  let createNewItem: (type: string) => void;
  
  let addToGridCallback = (type: "slider" | "button" | "rotator") => {
    createNewItem(type);
  }

</script>

<Splitpanes theme="">
  <Pane minSize={40} class="grid">
    <Grid bind:createNewItem={createNewItem} />
  </Pane>
  <Pane class="components">
    <ComponentChooser addToGridCallback={addToGridCallback}/>
  </Pane>
</Splitpanes>

<style global lang="scss">
  @use "../styles/config/config.scss";

  .splitpanes {
    border-top: 2px config.get-color("background-darker") solid;
    .splitpanes__pane {
      &.components {
        background-color: config.get-color("background-lighter");
      }
    }
    .splitpanes__splitter {
      background-color: config.get-color("background-darker");
      position: relative;
      min-width: 3px;

      &:before {
        content: "";
        position: absolute;
        left: 0;
        top: 0;
        transition: opacity 0.4s;
        background-color: #2db9d2;
        opacity: 0;
        z-index: 1;
      }
      &:hover:before {
        opacity: 1;
      }
      &.splitpanes__splitter__active {
        z-index: 2; /* Fix an issue of overlap fighting with a near hovered splitter */
      }
    }

    &.splitpanes--vertical > .splitpanes__splitter:before {
      left: -3px;
      right: -3px;
      height: 100%;
      cursor: col-resize;
    }
    &.splitpanes--horizontal > .splitpanes__splitter:before {
      top: -3px;
      bottom: -3px;
      width: 100%;
      cursor: row-resize;
    }
  }
</style>
