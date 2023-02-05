<script lang="ts">
  import "./styles/globals.scss";
  import MainArea from "./lib/MainArea.svelte";
  import Nav from "./lib/Nav.svelte";
  import SerialChooser from "./lib/SerialChooser.svelte";
  let saveItemsCallback: () => void;
  let globalSaveCallback = () => {
    saveItemsCallback();
  };
</script>

<main>
  <header>
    <Nav bind:globalSaveCallback />
  </header>
  <div class="main">
    <SerialChooser />
    <MainArea bind:saveItemsCallback />
  </div>
</main>

<style lang="scss">
  main {
    display: flex;
    flex-direction: column;
    height: 100vh;
    .main {
      display: flex;
      flex-direction: column;

      height: 100%;
    }
  }
</style>
