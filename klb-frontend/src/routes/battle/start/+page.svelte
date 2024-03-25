<script lang="ts">
	import { goto } from "$app/navigation";
	import { handleFetchPromise } from "$lib/api";
	import KeyboardInputGuide from "$lib/guide/KeyboardInputGuide.svelte";
	import KeyboardInput, { type SubmitData } from "$lib/keyboard/KeyboardInput.svelte";
	import type { CreateBattleRequest, CreateBattleResponse } from "$lib/schema";
	import { getModalStore, getToastStore } from "@skeletonlabs/skeleton";

    const toastStore = getToastStore();
    const modalStore = getModalStore();
    let disableSubmit = false;

    async function toBattle(e: CustomEvent<SubmitData>) {
        disableSubmit = true;

        let req: CreateBattleRequest = {
            base_layout_data: e.detail.layoutData,
            is_personal: false,
        }

        handleFetchPromise(
            fetch("/api/battle", {
                method: "POST",
                headers: {
                    "Content-Type": "application/json",
                    "Accept": "application/json",
                },
                body: JSON.stringify(req),
            }),
            (data: CreateBattleResponse) => {
                console.log("start battle!")
                goto("/battle/" + data.id);
            },
            toastStore,
        );
    }
</script>

<div class="container mx-auto p-8 flex justify-center">
    <div class="space-y-10 flex flex-col">
        <h1 class="h1">Input base layer</h1>
        <p>
            Provide the keyboard layout that you are using right now. Use this same layout for the battle.
        </p>
        <p>
            <button class="btn variant-glass-primary animate-pulse" on:click={() => {
                modalStore.trigger({
                    type: "component",
                    component: { ref: KeyboardInputGuide },
                })
            }}>How to input your keyboard layout</button>
        </p>
        <p>
            <KeyboardInput on:submit={toBattle} {disableSubmit} />
        </p>
    </div>
</div>