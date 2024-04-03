<script lang="ts">
	import ResultPanel from "$lib/ResultPanel.svelte";
	import Keyboard from "$lib/keyboard/Keyboard.svelte";
import type { PageData } from "./$types";

    export let data: PageData;
</script>

<div class="container mx-auto p-8 flex justify-center">
    <div class="space-y-10 flex flex-col">
        <h1 class="h1">Battle History {data.battle_history.id}</h1>

        <p>
            If the times are too close, it is regarded as a draw.
        </p>
        
        <ResultPanel
            layouts={data.layouts}
            layout_id_1={data.battle_history.layout_id_1}
            layout_id_2={data.battle_history.layout_id_2}
            result={data.battle_history.result_data}
        />

        <div class="grid grid-cols-3 gap-4 text-center">
            <hr>
            <hr>
            <hr>

            <div>{data.battle_history.layout_1_rating} ({data.battle_history.rating_1_gain>0?"+":""}{data.battle_history.rating_1_gain})</div>
            <div class="italic opacity-50">Speed rating changes</div>
            <div>{data.battle_history.layout_2_rating} ({data.battle_history.rating_2_gain>0?"+":""}{data.battle_history.rating_2_gain})</div>
        </div>

        <h2 class="h2 pt-10">Base Layout Used</h2>
        <Keyboard layoutData={data.battle_history.base_layout_data} heatmap={true} />
    </div>
</div>