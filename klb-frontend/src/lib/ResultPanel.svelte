<script lang="ts">
	import type { LayoutLite, ResultData } from "./schema";

    export let layouts: Map<number, LayoutLite>;
    export let layout_id_1: number;
    export let layout_id_2: number;
    export let result: ResultData;

    let layout_1 = layouts.get(layout_id_1);
    let layout_2 = layouts.get(layout_id_2);
</script>

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
    </div>
{:else}
    Something wrong happened
{/if}