<script lang="ts" context="module">
    const requiredChars = "abcdefghijklmnopqrstuvwxyz'"
    const allowedChars = "abcdefghijklmnopqrstuvwxyz.,<>;:\'\"/?()[]{}";
    const posTop = [
        0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
        52, 52, 52, 52, 52, 52, 52, 52, 52, 52, 52,
        104,104,104,104,104,104,104,104,104,104,
        104,
    ];
    const posLeft = [
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

    const toastStore = getToastStore();

    let layoutData = "";
    let markerTop = 0;
    let markerLeft = 0;
    let ready = false;
    $: ready = layoutData.length == 31;

    function onKeyDown(e: KeyboardEvent) {
        if(layoutData.length == 31) {
            return;
        }

        if(allowedChars.includes(e.key)) {
            // console.log(e.key)
            if(layoutData.includes(e.key)) { // already exist
                toastStore.trigger({
                    message: "duplicate keys not allowed!",
                    background: "variant-filled-error",
                })
            } else {
                layoutData = layoutData + e.key;
                markerTop = posTop[layoutData.length];
                markerLeft = posLeft[layoutData.length];
            }
        }
    }

    function reset() {
        layoutData = "";
        markerTop = 0;
        markerLeft = 0;
    }

    function toBattle() {
        console.log(layoutData);
        // validate base layout
        // character completeness
        for(let c of requiredChars) {
            if(!layoutData.includes(c)) {
                toastStore.trigger({
                    message: `key ${c} is missing`,
                    background: "variant-filled-error",
                })
                return;
            }
        }
    }
</script>
<div class="inline-block relative">
    <Keyboard bind:layoutData={layoutData}/>
    <KeyMarker top={markerTop} left={markerLeft} ping={!ready}/>
    <div class="p-4">
        <button class="btn variant-filled-error" on:click={reset}>Reset</button>
        <button class="btn variant-filled-primary" disabled={!ready} on:click={toBattle}>Use This!</button>
    </div>
</div>

<svelte:window on:keydown={onKeyDown} />
