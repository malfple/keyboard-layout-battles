<script lang="ts">
	import BattleInput from "$lib/battle/BattleInput.svelte";
    import type { PageData } from "./$types";

    export let data: PageData
</script>

<div class="container mx-auto p-8 flex flex-col h-full w-full">
    <div class="container mx-auto flex">
        <div class="space-y-10 flex flex-col">
            <h1 class="h1">Battle!</h1>
            <div>
                Type type type! End every word with a spacebar! Mistakes will reset the word and the timer.
            </div>
        </div>
    </div>
    <BattleInput wordPairs={data.words} />
</div>