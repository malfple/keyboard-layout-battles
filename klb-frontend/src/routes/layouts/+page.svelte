<script lang="ts">
	import { Table, tableMapperValues, type TableSource } from "@skeletonlabs/skeleton";
    import { onMount } from "svelte";
    import { getLayoutList } from "$lib/api";
    import type { LayoutLite } from "$lib/api";

    let layouts: null | LayoutLite[];
    let tableSource: TableSource;

    onMount(async () => {
        getLayoutList()
        .then(data => {
            layouts = data.layouts;
            tableSource = {
                head: ["ID", "Name", "Rating", "Comfort Rating"],
                body: tableMapperValues(layouts, ["id", "name", "rating", "rating_comfort"]),
            }
        })
    });
</script>

<div class="container mx-auto p-8 flex justify-center">
    <div class="space-y-10 flex flex-col">
        <h1 class="h1">Layouts</h1>
        <p>
            List of layouts
        </p>
        {#if layouts}
            <Table source={tableSource} />
        {:else}
            Loading...
        {/if}
    </div>
</div>