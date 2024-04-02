<script lang="ts">
	import type { LayoutLite, ResultData } from "./schema";

    export let layouts: Map<number, LayoutLite>;
    export let layout_id_1: number;
    export let layout_id_2: number;
    export let result: ResultData;

    let layout_1 = layouts.get(layout_id_1);
    let layout_2 = layouts.get(layout_id_2);

    let score_1 = 0;
    let score_2 = 0;
    let comfort_score_1 = 0;
    let comfort_score_2 = 0;
    for(let word of result.words) {
        if(word.score == 1) score_1++;
        else if(word.score == -1) score_2++;
        if(word.comfort_choice == 1) comfort_score_1++;
        else if(word.comfort_choice == 2) comfort_score_2++;
    }
</script>

{#if layout_1 && layout_2}
    <div class="grid grid-cols-3 gap-4 text-center">
        <h2 class="h2">
            <a href={`/layout/${layout_1.id}-${layout_1.name}`}>{layout_1.name}</a>
        </h2>
        <h2 class="h2">
            vs
        </h2>
        <h2 class="h2">
            <a href={`/layout/${layout_2.id}-${layout_2.name}`}>{layout_2.name}</a>
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
                <div class="text-2xl {word.score==0?"text-warning-500":""} {word.score==1?"text-primary-500":""} {word.score==-1?"text-error-500":""}">{word.time_1}</div>
            </div>
            <div />
            <div>
                <div class="italic opacity-50">Time (ms)</div>
                <div class="text-2xl {word.score==0?"text-warning-500":""} {word.score==-1?"text-primary-500":""} {word.score==1?"text-error-500":""}">{word.time_2}</div>
            </div>

            <div>
                {#if word.comfort_choice == 1}<div class="text-primary-500">Comfortable!</div>{/if}
                {#if word.comfort_choice == 0}<div class="text-warning-500">felt the same</div>{/if}
            </div>
            <div />
            <div>
                {#if word.comfort_choice == 2}<div class="text-primary-500">Comfortable!</div>{/if}
                {#if word.comfort_choice == 0}<div class="text-warning-500">felt the same</div>{/if}
            </div>
            
            <hr>
            <hr>
            <hr>
        {/each}

        <div class="text-3xl">
            {#if result.score > 0}<div class="text-primary-500">Win</div>{/if}
            {#if result.score == 0}<div class="text-warning-500">Draw</div>{/if}
        </div>
        <div>
            <div class="italic opacity-50">Speed Score</div>
            <div class="text-2xl">{score_1} - {score_2}</div>
        </div>
        <div class="text-primary-500 text-3xl">
            {#if result.score < 0}<div class="text-primary-500">Win</div>{/if}
            {#if result.score == 0}<div class="text-warning-500">Draw</div>{/if}
        </div>

        <div class="text-primary-500 text-3xl">
            {#if result.comfort_score > 0}<div class="text-primary-500">Win</div>{/if}
            {#if result.comfort_score == 0}<div class="text-warning-500">Draw</div>{/if}
        </div>
        <div>
            <div class="italic opacity-50">Comfort Score</div>
            <div class="text-2xl">{comfort_score_1} - {comfort_score_2}</div>
        </div>
        <div class="text-primary-500 text-3xl">
            {#if result.comfort_score < 0}<div class="text-primary-500">Win</div>{/if}
            {#if result.comfort_score == 0}<div class="text-warning-500">Draw</div>{/if}
        </div>
    </div>
{:else}
    Something wrong happened
{/if}