<script lang="ts">
	import Key from "./Key.svelte";

    export let layoutData: string;
    export let heatmap: boolean = false;

    let paddedLayoutData: string;
    $: paddedLayoutData = layoutData.padEnd(31, ' ');

    let rowsLeft: string[];
    $: rowsLeft = [
        paddedLayoutData.substring(0, 5),
        paddedLayoutData.substring(10, 15),
        paddedLayoutData.substring(21, 26),
    ];
    let rowsRight: string[]
    $: rowsRight = [
        paddedLayoutData.substring(5, 10),
        paddedLayoutData.substring(15, 21),
        paddedLayoutData.substring(26, 31),
    ];
</script>

<div class="inline-block">
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