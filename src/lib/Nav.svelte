<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri";
    import Logo from "./Logo.svelte";
    
    export let globalSaveCallback: () => void;
    let callback = async () => {
        globalSaveCallback();
        await invoke("save_config");
    }
</script>

<nav>
    <div class="logo"><Logo /></div>
    <div class="right-menu">
        <div class="save" on:click={callback}>
            Save
        </div>
    </div>
</nav>

<style lang="scss">
@use "../styles/config/config.scss";
@use "../styles/config/util.scss";
nav {
    display: flex;
    justify-content: space-between;
    background-color: config.get-color("background-darker");
    padding: 1rem;
    .logo {
        width: 5rem;
    }
    .right-menu {
        display: flex;
        justify-content: center;
        align-items: center;
        // p-4 cursor-pointer bg-primary rounded-2xl shadow-2xl
        .save {
            padding: 1rem;
            cursor: pointer;
            background-color: config.get-color('primary');
            border-radius: 1rem;
            box-shadow: 1px 1px 1px 1px rgba($color: #000000, $alpha: 0.6);
            color: util.is-color-dark(config.get-color('primary'));
        }
    }
}
</style>
