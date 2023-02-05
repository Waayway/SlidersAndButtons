import type { SvelteComponent } from "svelte"

export type GridItem = {
    x: number,
    y: number, 
    width: number,
    height: number,
    type: "slider" | "button" | "rotator"
    data: GridItemData
}
export type GridItemTemplate = {
    button: GridItem,
    slider: GridItem,
    rotator: GridItem,
}

export type GridItemData = {
    key?: string,
}

export type Component = {
    name: string,
    type: "slider" | "button" | "rotator"
    inputs: {
        key?: boolean,
    }
}

export type KeyCombo = {
    key?: string,
    modifiers: {
        alt?: boolean,
        ctrl?: boolean,
        shift?: boolean,
        super?: boolean
    }
}