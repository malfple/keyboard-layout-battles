<script lang="ts">
	import { goto } from "$app/navigation";
	import type { BattleHistoryLite, GetBattleHistoryListResponse } from "$lib/schema";
	import { getToastStore } from "@skeletonlabs/skeleton";
	import { onMount } from "svelte";

    const toastStore = getToastStore();
    
    let battles: BattleHistoryLite[];

    onMount(async () => {
        fetch("/api/battle/histories?limit=10")
        .then(resp => resp.json().then((data: GetBattleHistoryListResponse) => {
            if(!resp.ok) throw {status: resp.status, data: data};
            battles = data.battles;
        }))
        .catch((err: {status: number, data: GetBattleHistoryListResponse}) => {
            toastStore.trigger({
                message: `error: ${err.data.error}, msg: ${err.data.error_message}`,
                background: "variant-filled-error"
            });
        });
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
                <div class="table table-interactive">
                    <thead>
                        <tr>
                            <th>ID</th>
                            <th>Layout 1</th>
                            <th>Layout 2</th>
                        </tr>
                    </thead>
                    <tbody>
                        {#each battles as battle}
                            <tr>
                                <td>{battle.id}</td>
                                <td>{battle.layout_id_1}</td>
                                <td>{battle.layout_id_2}</td>
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