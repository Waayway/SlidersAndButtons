<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import { onMount } from "svelte";
  import type { KeyCombo } from "./types";
  export let callback: (key: string) => void;

    let allResults = [];

    let results = [];
    onMount(async () => {
        allResults = await invoke("get_possible_keys");
        results = allResults;
    });

    const search = (ev: Event & { currentTarget: EventTarget & HTMLInputElement; }) => {
        results = allResults.filter((i: string) => i.toLowerCase().includes((ev.target as HTMLInputElement).value.toLowerCase()));
    }

</script>

<div class="key-chooser stop-scroll">
  <div class="search">
    <input type="text" on:input={search} />
  </div>
  <div class="results">
    <ul>
        {#each results as item (item)}
            <li on:click={(ev) => callback(item)}>{item}</li>
        {/each}
    </ul>
  </div>
</div>

<style lang="scss">
  @use "../../styles/config/config.scss";
  @use "../../styles/config/util.scss";
  .key-chooser {
    position: absolute;
    $margin: 0.4rem;
    left: $margin;
    right: $margin;
    bottom: $margin;
    top: $margin;
    background-color: lighten(config.get-color("popup-bg"), 20%);
    border-radius: 1rem;
    z-index: 100;
    display: flex;
    flex-direction: column;
    overflow: hidden;

    .search {
      input {
        border: none;
        background-color: rgba($color: #fff, $alpha: 0.2);
        border-radius: 1rem;
        margin: .5rem;
        width: calc(100% - 1rem);
        padding: 0.5rem;
        color: util.is-color-dark(config.get-color("popup-bg"));
        outline: none;
        text-align: center;
      }
    }
    .results {
        overflow-y: scroll;
        height: 100%;
        ul {
            margin: 0;
            padding: 0 1rem;
            list-style: none;
            text-align: left;
            li {
                background-color: transparent;
                cursor: pointer;
                transition: background-color 300ms ease-out;
                padding: .3rem .1rem;
                border-radius: .5rem;
                &:hover {
                    background-color: rgba($color: #000000, $alpha: 0.1);
                }
            }
        }
    }
  }
</style>
