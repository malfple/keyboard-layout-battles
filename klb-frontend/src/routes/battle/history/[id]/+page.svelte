<script lang="ts">
	import type { PageData } from "./$types";

    export let data: PageData;

    let layout_1 = data.layouts.get(data.battle_history.layout_id_1);
    let layout_2 = data.layouts.get(data.battle_history.layout_id_2);

    let result = data.battle_history.result_data;
</script>

<div class="container mx-auto p-8 flex justify-center">
    <div class="space-y-10 flex flex-col">
        <h1 class="h1">Battle History {data.battle_history.id}</h1>
        {#if layout_1 && layout_2}
            <div class="grid grid-cols-3 gap-10 text-center">
                <h2 class="h2">
                    {layout_1.name}
                </h2>
                <h2 class="h2">
                    vs
                </h2>
                <h2 class="h2">
                    {layout_2.name}
                </h2>

                {#each result.words as word}
                    <div>
                        <div class="italic opacity-50">Translated word</div>
                        <div class="text-2xl">{word.translated_1}</div>
                    </div>
                    <div>
                        <div class="italic opacity-50">Original word</div>
                        <div class="text-2xl">{word.original}</div>
                    </div>
                    <div>
                        <div class="italic opacity-50">Translated word</div>
                        <div class="text-2xl">{word.translated_2}</div>
                    </div>
                    
                    <div>
                        <div class="italic opacity-50">Time (ms)</div>
                        <div class="text-2xl">{word.time_1}</div>
                    </div>
                    <div />
                    <div>
                        <div class="italic opacity-50">Time (ms)</div>
                        <div class="text-2xl">{word.time_2}</div>
                    </div>

                    <div class="text-primary-500 text-3xl">
                        {#if word.comfort_choice == 1}&#10004; {/if}
                    </div>
                    <div class="italic opacity-50">Comfort choice</div>
                    <div class="text-primary-500 text-3xl">
                        {#if word.comfort_choice == 2}&#10004; {/if}
                    </div>
                    
                    <hr>
                    <hr>
                    <hr>
                {/each}

                <div class="text-primary-500 text-3xl">
                    {#if result.score > 0}Winner{/if}
                </div>
                <div>
                    <div class="italic opacity-50">Speed Score</div>
                    <div class="text-2xl">{result.score}</div>
                </div>
                <div class="text-primary-500 text-3xl">
                    {#if result.score < 0}Winner{/if}
                </div>

                <div class="text-primary-500 text-3xl">
                    {#if result.comfort_score > 0}Winner{/if}
                </div>
                <div>
                    <div class="italic opacity-50">Comfort Score</div>
                    <div class="text-2xl">{result.comfort_score}</div>
                </div>
                <div class="text-primary-500 text-3xl">
                    {#if result.comfort_score < 0}Winner{/if}
                </div>

                <hr>
                <hr>
                <hr>

                <div>{data.battle_history.layout_1_rating} ({data.battle_history.rating_1_gain>0?"+":""}{data.battle_history.rating_1_gain})</div>
                <div class="italic opacity-50">Speed rating changes</div>
                <div>{data.battle_history.layout_2_rating} ({data.battle_history.rating_2_gain>0?"+":""}{data.battle_history.rating_2_gain})</div>
            </div>
        {:else}
            Something wrong happened
        {/if}
    </div>
</div>