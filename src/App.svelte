<script lang="ts">
  import Nav from "./lib/Nav.svelte";
    import SerialChooser from "./lib/SerialChooser.svelte";
</script>

<main class="bg-bg w-full h-screen">
  <header>
    <Nav></Nav>
  </header>
  <div class="grid grid-cols-4">
    <div class="col-span-2">
      <SerialChooser />
    </div>    
    <div class="col-span-1">
      
    </div>
  </div>
</main>