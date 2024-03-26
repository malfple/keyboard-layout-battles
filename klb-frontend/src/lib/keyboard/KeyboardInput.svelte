<script lang="ts" context="module">
    const REQUIRED_CHARS = "abcdefghijklmnopqrstuvwxyz'"
    export const ALLOWED_CHARS = "abcdefghijklmnopqrstuvwxyz.,<>;:\'\"/?()[]{}-";
    export const CHAR_PREVENT_DEFAULT = ".,<>;:\'\"/?()[]{}-";
    const POS_TOP = [
        0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
        52, 52, 52, 52, 52, 52, 52, 52, 52, 52, 52,
        104,104,104,104,104,104,104,104,104,104,
        104,
    ];
    const POS_LEFT = [
        0,  52, 104,156,208,308,360,412,464,516,
        0,  52, 104,156,208,308,360,412,464,516,568,
        0,  52, 104,156,208,308,360,412,464,516,
        516,
    ];

    export interface SubmitData {
        layoutData: string
    }
</script>

<script lang="ts">
	import { getToastStore } from "@skeletonlabs/skeleton";

	import KeyMarker from "./KeyMarker.svelte";
    import Keyboard from "./Keyboard.svelte";
	import { createEventDispatcher } from "svelte";
	import { get } from "svelte/store";
	import { layoutDataStore } from "$lib/stores";

    const toastStore = getToastStore();
    const dispatch = createEventDispatcher<{submit:SubmitData}>();

    export let disableSubmit = false;
    export let useStore;

    let layoutData = "";
    if(useStore) { // load saved layout
        layoutData = get(layoutDataStore);
        if(layoutData != "") {
            toastStore.trigger({
                message: "Loaded layout from local storage! Make sure it is the current one you are using.",
                background: "variant-filled-primary",
            });
        }
    }
    let markerTop = 0;
    let markerLeft = 0;
    $: markerTop = POS_TOP[layoutData.length];
    $: markerLeft = POS_LEFT[layoutData.length];
    let ready = false;
    $: ready = layoutData.length == 31;

    function onKeyDown(e: KeyboardEvent) {
        if(layoutData.length == 31) {
            return;
        }

        if(CHAR_PREVENT_DEFAULT.includes(e.key)) {
            e.preventDefault();
        }

        if(ALLOWED_CHARS.includes(e.key)) {
            // console.log(e.key)
            if(layoutData.includes(e.key)) { // already exist
                toastStore.trigger({
                    message: "duplicate keys not allowed!",
                    background: "variant-filled-error",
                })
            } else {
                layoutData = layoutData + e.key;
            }
        }
    }

    function reset() {
        layoutData = "";
    }

    function submit() {
        // validate base layout
        // character completeness
        for(let c of REQUIRED_CHARS) {
            if(!layoutData.includes(c)) {
                toastStore.trigger({
                    message: `key ${c} is missing`,
                    background: "variant-filled-error",
                })
                return;
            }
        }

        // save layout
        layoutDataStore.set(layoutData);

        dispatch("submit", {
            layoutData: layoutData,
        })
    }
</script>
<div class="inline-block relative">
    <Keyboard bind:layoutData={layoutData}/>
    <KeyMarker top={markerTop} left={markerLeft} ping={!ready}/>
    <div class="p-4">
        <button class="btn variant-filled-error" on:click={reset}>Reset</button>
        <button class="btn variant-filled-primary" disabled={!ready || disableSubmit} on:click={submit}>Use This!</button>
    </div>
</div>

<svelte:window on:keydown={onKeyDown} />
