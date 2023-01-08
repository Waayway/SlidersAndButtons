<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri";
    import { onMount } from "svelte";
    import MdRefresh from "svelte-icons/md/MdRefresh.svelte";

    let options: Array<String>;

    let refresh = async () => {
        options = await invoke("getSerialPorts");
    };
    let selected;

    onMount(async () => await refresh());
</script>

<div class="flex p-3 text-text items-center">
    <h3 class="text-xl">Select Serial port of Arduino</h3>
    <select value={selected} class="bg-bg-lighter rounded-lg ml-2 p-2">
        {#if options}
            {#each options as option}
                <option value={option}>
                    {option}
                </option>
            {/each}
        {/if}
    </select>
    <button class="w-8" on:click={refresh}>
        <MdRefresh />
    </button>
</div>
