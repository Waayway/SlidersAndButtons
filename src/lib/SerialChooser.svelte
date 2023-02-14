<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import { onMount } from "svelte";
  import MdRefresh from "svelte-icons/md/MdRefresh.svelte";

  let options: Array<String>;
    let selected;

  let refresh = async () => {
    options = await invoke("get_serial_ports");
    selected = await invoke("get_serial_config");
  };

  onMount(async () => await refresh());

  let update_serial_port = async (e) => {
    await invoke("save_serial_port", { serialport: selected });
  };
</script>

<div class="serialChooser">
  <h3>Select Serial port of Arduino</h3>
  <select
    bind:value={selected}
    on:change={update_serial_port}
  >
    <option value={""}>No Selected</option>
    {#if options}
      {#each options as option}
        <option value={option}>
          {option}
        </option>
      {/each}
    {/if}
  </select>
  <button on:click={refresh}>
    <MdRefresh />
  </button>
</div>

<style lang="scss">
  @use "../styles/config/config.scss";
  @use "../styles/config/util.scss";
  .serialChooser {
    display: flex;
    padding: 1rem;
    h3 {
        font-size: 1.2rem;
        white-space: nowrap;
        line-height: 0;
    }
    select {
        background-color: config.get-color("background-lighter");
        color: util.is-color-dark(config.get-color("background-lighter"));
        border-radius: .5rem;
        margin-left: .3rem;
        padding: .2rem;
    }
    button {
        width: 2.5rem;
        cursor: pointer;
        border: none;
        background: transparent;
        color: util.is-color-dark(config.get-color("background"));
    }
  }
</style>
