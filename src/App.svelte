<script lang="ts">
  import "./styles/globals.scss";
  import MainArea from "./lib/MainArea.svelte";
  import Nav from "./lib/Nav.svelte";
  import SerialChooser from "./lib/SerialChooser.svelte";
</script>

<main>
  <header>
    <Nav />
  </header>
  <div class="main">
    <SerialChooser />
    <MainArea />
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
