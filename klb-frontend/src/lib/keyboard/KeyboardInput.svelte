<script lang="ts" context="module">
    const REQUIRED_CHARS = "abcdefghijklmnopqrstuvwxyz'"
    export const ALLOWED_CHARS = "abcdefghijklmnopqrstuvwxyz.,<>;:\'\"/?()[]{}-";
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
</script>

<script lang="ts">
	import { getToastStore } from "@skeletonlabs/skeleton";

	import KeyMarker from "./KeyMarker.svelte";
    import Keyboard from "./Keyboard.svelte";
	import { createEventDispatcher } from "svelte";

    const toastStore = getToastStore();
    const dispatch = createEventDispatcher<{submit:{layoutData: string}}>();

    export let disableSubmit = false;
    let layoutData = "";
    let markerTop = 0;
    let markerLeft = 0;
    let ready = false;
    $: ready = layoutData.length == 31;

    function onKeyDown(e: KeyboardEvent) {
        if(layoutData.length == 31) {
            return;
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
                markerTop = POS_TOP[layoutData.length];
                markerLeft = POS_LEFT[layoutData.length];
            }
        }
    }

    function reset() {
        layoutData = "";
        markerTop = 0;
        markerLeft = 0;
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