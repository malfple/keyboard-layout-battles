<script lang="ts">
	import BattleInput, { type SubmitData } from "$lib/battle/BattleInput.svelte";
	import type { FinalizeBattleRequest, FinalizeBattleResponse } from "$lib/schema";
	import { getToastStore } from "@skeletonlabs/skeleton";
    import type { PageData } from "./$types";
	import { goto } from "$app/navigation";
	import ResultPanel from "$lib/ResultPanel.svelte";
	import { handleFetchPromise } from "$lib/api";

    export let data: PageData

    const toastStore = getToastStore();

    let battleResult: FinalizeBattleResponse | null = null;
    let resultBox: HTMLDivElement;

    function finalizeBattle(e: CustomEvent<SubmitData>) {
        toastStore.trigger({
            message: "submitting battle...",
            background: "variant-filled",
        });

        let req: FinalizeBattleRequest = {
            id: data.id,
            times: e.detail.times,
            comfort_choice: e.detail.comfort,
        }

        handleFetchPromise(
            fetch("/api/battle", {
                method: "PUT",
                headers: {
                    "Content-Type": "application/json",
                },
                body: JSON.stringify(req),
            }),
            (data: FinalizeBattleResponse) => {
                console.log("finalize battle!");
                battleResult = data;
                // scroll to view
                setTimeout(() => {
                    toastStore.trigger({
                        message: "Results submitted",
                        background: "variant-filled-success",
                    });
                    resultBox.scrollIntoView({ behavior: "smooth" });
                }, 1000)
            },
            toastStore,
        );
    }
</script>

<div class="container mx-auto p-8 flex flex-col h-screen">
    <div class="container mx-auto flex">
        <div class="space-y-10 flex flex-col">
            <h1 class="h1">Battle!</h1>
            <!-- <BattleTips /> -->
        </div>
    </div>
    <BattleInput wordPairs={data.words} on:submit={finalizeBattle}/>
</div>

{#if battleResult}
    <div bind:this={resultBox} class="container mx-auto p-8 flex justify-center">
        <div class="space-y-10 flex flex-col">
            <ResultPanel
                layouts={data.layouts}
                layout_id_1={battleResult.layout_id_1}
                layout_id_2={battleResult.layout_id_2}
                result={battleResult.result_data}
            />
            <button class="btn variant-filled-primary" on:click={() => goto("/battle/start")}>
                Battle again
            </button>
        </div>
    </div>
{/if}
