<script lang="ts">
  import MdDelete from "svelte-icons/md/MdDelete.svelte";
  import type { GridItem, KeyCombo } from "./types";

  export let gridData: GridItem;
  export let active: boolean;
  export let updateKey: (key: string) => void;
  export let deleteSelf: () => void;

  let keyCombo: KeyCombo = { modifiers: {} };

  let deleting = false;

  const onKeyPress = (ev: KeyboardEvent) => {
    if (ev.key == "Enter") {
      updateKey((ev.currentTarget as HTMLInputElement).value);
    }
  };

  const sureDelete = () => {
    deleting = true;
  };

  let ctrlClicked = false;
</script>

<div class={`popup ${active ? "active" : ""}`}>
  {#if gridData.type == "button"}
    <div class="key">
      <input type="text" placeholder="Key on keypad" on:keypress={onKeyPress} value={gridData.data.key} />
    </div>
    <div class="keypress">
      <p>Control:</p>
      <input type="checkbox" bind:checked={keyCombo.modifiers.ctrl} />

      <p>Alt:</p>
      <input type="checkbox" bind:checked={keyCombo.modifiers.alt} />

      <p>Shift:</p>
      <input type="checkbox" bind:checked={keyCombo.modifiers.shift} />

      <p>Win/Super:</p>
      <input type="checkbox" bind:checked={keyCombo.modifiers.super} />
    </div>
  {/if}
  <div class="deleteDiv">
    <button class="delete" on:click={sureDelete}>
      <div><MdDelete /></div>
      <p>Delete</p>
    </button>
    <div class={`sure ${deleting ? "active" : ""}`}>
      <button class="yes" on:click={() => deleteSelf()}>Yes</button>
      <p>Are you sure</p>
      <button
        class="no"
        on:click={() => {
          console.log(keyCombo);
          deleting = false;
        }}>No</button
      >
    </div>
  </div>
</div>

<style lang="scss">
  @use "../../styles/config/config.scss";
  @use "../../styles/config/util.scss";
  .popup {
    z-index: 100;
    display: none;
    background-color: config.get-color("popup-bg");
    color: util.is-color-dark(config.get-color("popup-bg"));
    border-radius: 0.5rem;
    flex-direction: column;
    padding: 0.5rem;
    gap: 0.5rem;

    position: absolute;
    left: 100%;
    top: 50%;
    transform: translate(0, -50%);
    &.active {
      display: flex;
    }
    input {
      border: none;
      background-color: rgba($color: #fff, $alpha: 0.1);
      border-radius: 1rem;
      padding: .5rem;
      color: util.is-color-dark(config.get-color("popup-bg"));
      outline: none;
      text-align: center;
      &::placeholder {
        font-style: italic;
        color: transparantize(
          util.is-color-dark(config.get-color("popup-bg")),
          50%
        );
      }
    }
    .keypress {
        display: grid;
        grid-template-columns: 1fr 1fr;
        p {
          line-height: 0;
          text-align: right;
        }
        input {
          text-align: left;
        }
    }
    .deleteDiv {
      position: relative;
      .delete {
        width: calc(100% - 0.7rem);
        box-sizing: content-box;
        background-color: config.get-color("error");
        color: util.is-color-dark(config.get-color("error"));
        height: 2rem;
        display: flex;
        padding: 0.2rem;
        justify-content: center;
        align-items: center;
        border-radius: 1rem;
        border: 0;
        div {
          height: 1.5rem;
        }
        p {
          margin: 0 0.3rem;
          line-height: 0;
        }
      }
      .sure {
        box-sizing: content-box;

        height: 2rem;
        padding: 0.2rem;

        display: none;
        position: absolute;
        top: 0;
        left: 0;
        right: 0;
        background-color: config.get-color("popup-bg");
        border: config.get-color("error") 2px solid;
        border-radius: 1rem;
        justify-content: space-between;
        align-items: center;
        button {
          border: none;
          outline: none;
          background-color: transparent;
          &.yes {
            color: config.get-color("popup-del-yes");
          }
          &.no {
            color: config.get-color("popup-del-no");
          }
        }

        &.active {
          display: flex;
        }
      }
    }
  }
</style>
