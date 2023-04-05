<script lang="ts">
    import MdRefresh from "svelte-icons/md/MdRefresh.svelte";


    export let callback: (key: string) => void;

    export let refresh_session: () => Promise<String[]>;

    export let allSessions: Array<String>;

    let results = allSessions;

    const search = (
        ev: Event & { currentTarget: EventTarget & HTMLInputElement }
    ) => {
        results = allSessions.filter((i: string) =>
            i
                .toLowerCase()
                .includes((ev.target as HTMLInputElement).value.toLowerCase())
        );
    };
    const refresh = async () => {
        allSessions = await refresh_session();
        results = allSessions;
    };
</script>

<div class="key-chooser stop-scroll">
    <div class="search">
        <input type="text" on:input={search} />
        <button on:click={refresh}>
            <MdRefresh />
        </button>
    </div>
    <div class="results">
        <ul>
            {#each results as item, i (i)}
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
            display: flex;
            input {
                border: none;
                background-color: rgba($color: #fff, $alpha: 0.2);
                border-radius: 1rem;
                margin: 0.5rem;
                width: calc(100% - 1rem);
                padding: 0.5rem;
                color: util.is-color-dark(config.get-color("popup-bg"));
                outline: none;
                text-align: center;
            }
            button {
                background-color: transparent;
                border: none;
                width: 3rem;
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
                    padding: 0.3rem 0.1rem;
                    border-radius: 0.5rem;
                    &:hover {
                        background-color: rgba($color: #000000, $alpha: 0.1);
                    }
                }
            }
        }
    }
</style>
