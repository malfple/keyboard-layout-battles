<script lang="ts">
	import { goto } from "$app/navigation";
	import type { BattleHistoryLite, GetBattleHistoryListResponse } from "$lib/schema";
	import { getToastStore } from "@skeletonlabs/skeleton";
	import { onMount } from "svelte";
	import type { PageData } from "./$types";
	import { handleFetchPromise } from "$lib/api";

    export let data: PageData;

    const toastStore = getToastStore();
    
    let battles: BattleHistoryLite[];

    onMount(async () => {
        handleFetchPromise(fetch("/api/battle/histories?limit=10"), (data: GetBattleHistoryListResponse) => {
            battles = data.battles;
        }, toastStore);
    })
</script>

<div class="container mx-auto p-8 flex justify-center">
    <div class="space-y-10 flex flex-col">
        <h1 class="h1">Battle</h1>
        <p>
            Start a battle!
        </p>
        <p>
            <button type="button" class="btn variant-filled-primary h-48 w-96 rounded-xl" on:click={() => goto("/battle/start")}>
                <div class="whitespace-normal">
                    <div class="text-2xl">Start Battle</div>
                    <div>Start a battle with english words and the global layout pool</div>
                </div>
            </button>
        </p>
        <h2 class="h2">Recent Battles</h2>
        {#if battles}
            <div class="table-container">
                <div class="table table-compact table-interactive">
                    <thead>
                        <tr>
                            <th>ID</th>
                            <th>Layout 1</th>
                            <th>Rating</th>
                            <th>Layout 2</th>
                            <th>Rating</th>
                        </tr>
                    </thead>
                    <tbody>
                        {#each battles as battle}
                            <tr on:click={() => {goto(`/battle/history/${battle.id}`)}}>
                                <td>{battle.id}</td>
                                <td>{data.layouts.get(battle.layout_id_1)?.name}</td>
                                <td>{battle.layout_1_rating} ({battle.rating_1_gain>0?"+":""}{battle.rating_1_gain})</td>
                                <td>{data.layouts.get(battle.layout_id_2)?.name}</td>
                                <td>{battle.layout_2_rating} ({battle.rating_2_gain>0?"+":""}{battle.rating_2_gain})</td>
                            </tr>
                        {/each}
                    </tbody>
                </div>
            </div>
        {:else}
            <div class="placeholder" />
        {/if}
    </div>
</div>