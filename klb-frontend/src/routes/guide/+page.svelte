<script lang="ts">
	import KeyboardInputGuide from "$lib/guide/KeyboardInputGuide.svelte";
	import { getModalStore } from "@skeletonlabs/skeleton";

    const modalStore = getModalStore();
</script>

<div class="container mx-auto p-8 flex justify-center">
    <div class="space-y-10 flex flex-col">
        <h1 class="h1">Guide</h1>
        <p>
            Contains some guides that are also shown on other pages.
        </p>
        <ul class="list">
            <li>
                <button class="btn variant-glass-primary" on:click={() => {
                    modalStore.trigger({
                        type: "component",
                        component: { ref: KeyboardInputGuide },
                    })
                }}>How to input your keyboard layout</button>
            </li>
        </ul>
    </div>
</div>