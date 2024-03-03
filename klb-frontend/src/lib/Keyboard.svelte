<script lang="ts">
	import Key from "./Key.svelte";

    export let layoutData: string;
    export let heatmap: boolean = false;

    let rowsLeft: string[] = [
        layoutData.substring(0, 5),
        layoutData.substring(10, 15),
        layoutData.substring(21, 26),
    ];
    let rowsRight: string[] = [
        layoutData.substring(5, 10),
        layoutData.substring(15, 21),
        layoutData.substring(26, 31),
    ]
</script>

<div>
    <div class="grid grid-cols-2">
        <div class="grid grid-rows-3 gap-1">
            {#each rowsLeft as row}
                <div class="grid grid-cols-6 gap-1">
                    {#each row as ch}
                        <Key char={ch} {heatmap} />
                    {/each}
                </div>
            {/each}
        </div>
        <div class="grid grid-rows-3 gap-1">
            {#each rowsRight as row}
                <div class="grid grid-cols-6 gap-1">
                    {#each row as ch}
                        <Key char={ch} {heatmap} />
                    {/each}
                </div>
            {/each}
        </div>
    </div>
</div>