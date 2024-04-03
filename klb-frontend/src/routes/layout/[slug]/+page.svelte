<script lang="ts">
	import Keyboard from "$lib/keyboard/Keyboard.svelte";
	import { onMount } from "svelte";
    import type { PageData } from "./$types";
	import { handleFetchPromise } from "$lib/api";
	import type { BattleHistoryLite, GetBattleHistoryListResponse } from "$lib/schema";
	import { getToastStore } from "@skeletonlabs/skeleton";
	import { goto } from "$app/navigation";

	export let data: PageData;

    const toastStore = getToastStore();

    let battles: BattleHistoryLite[];

    onMount(async () => {
        handleFetchPromise(fetch(`/api/battle/histories?limit=50&layout_id=${data.id}`), (data: GetBattleHistoryListResponse) => {
            battles = data.battles;
        }, toastStore);
    })
</script>

<div class="container mx-auto p-8 flex justify-center">
    <div class="space-y-10 flex flex-col items-center">
        <h1 class="h1">{data.layout.name}</h1>
        <div>
            <Keyboard layoutData={data.layout.layout_data} />
        </div>
        <div>
            <Keyboard layoutData={data.layout.layout_data} heatmap={true} />
        </div>
        <div class="grid grid-cols-2 gap-10">
            <div class="space-y-10">
                {#if data.layout.description}
                    <div>
                        {data.layout.description}
                    </div>
                {/if}
                <div>
                    Rating: {data.layout.rating}
                </div>
                <div>
                    Comfort Rating: {data.layout.rating_comfort}
                </div>
                <div>
                    Rating Data:
                </div>
                <pre>{JSON.stringify(data.layout.rating_data, null, 4)}</pre>
            </div>
            <div>
                <h3 class="h3 p-4">
                    Recent Battles
                </h3>
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
    </div>
</div>