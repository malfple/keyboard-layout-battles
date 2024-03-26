<script lang="ts" context="module">
    const TOP_LAYOUT_COUNT = 7;
</script>

<script lang="ts">
	import { onMount } from "svelte";
	import type { GetLayoutListResponse, LayoutLite } from "$lib/schema";
	import { goto } from "$app/navigation";
	import { getToastStore } from "@skeletonlabs/skeleton";
	import { handleFetchPromise } from "$lib/api";

    const toastStore = getToastStore();
    let layouts: LayoutLite[];
    let topLayouts: LayoutLite[];
    let topComfortLayouts: LayoutLite[];

    let sort: "" | "id-asc" | "id-dsc" | "name-asc" | "name-dsc" | "rating-asc" | "rating-dsc" | "comfort-asc" | "comfort-dsc" = "";
    $: sort, sortLayouts(), layouts = layouts;

    onMount(async () => {
        handleFetchPromise(fetch("/api/layouts"), (data: GetLayoutListResponse) => {
            layouts = data.layouts;

            topLayouts = structuredClone(layouts);
            topLayouts.sort((a, b) => {
                return b.rating - a.rating;
            });
            if(topLayouts.length > TOP_LAYOUT_COUNT) topLayouts.length = TOP_LAYOUT_COUNT;
            
            topComfortLayouts = structuredClone(layouts);
            topComfortLayouts.sort((a, b) => {
                return b.rating_comfort - a.rating_comfort;
            });
            if(topComfortLayouts.length > TOP_LAYOUT_COUNT) topComfortLayouts.length = TOP_LAYOUT_COUNT;
        }, toastStore);
    });

    function compareString(a: string, b: string): number {
        if(a < b) return -1;
        if(a > b) return 1;
        return 0;
    }

    function sortLayouts() {
        switch(sort) {
            case "id-asc":
                layouts.sort((a, b) => (a.id - b.id));
                break;
            case "id-dsc":
                layouts.sort((a, b) => (b.id - a.id));
                break;
            case "name-asc":
                layouts.sort((a, b) => compareString(a.name, b.name));
                break;
            case "name-dsc":
                layouts.sort((a, b) => compareString(b.name, a.name));
                break;
            case "rating-asc":
                layouts.sort((a, b) => (a.rating - b.rating));
                break;
            case "rating-dsc":
                layouts.sort((a, b) => (b.rating - a.rating));
                break;
            case "comfort-asc":
                layouts.sort((a, b) => (a.rating_comfort - b.rating_comfort));
                break;
            case "comfort-dsc":
                layouts.sort((a, b) => (b.rating_comfort - a.rating_comfort));
                break;
        }
    }
</script>

<div class="container mx-auto p-8 flex justify-center">
    <div class="space-y-10 flex flex-col">
        <h1 class="h1">Layouts</h1>
        <p>
            List of layouts.
            "Rating" typically refers to speed rating which is measured by the typing time
            and "Comfort" refers to the comfort rating which based on the typer's preference.
            Higher rating generally means better, but keep in mind that ratings may fluctuate.
        </p>
        {#if layouts}
            <h2 class="h2">Top Layouts</h2>
            <div class="grid grid-cols-2 gap-10">
                <div>
                    <div class="p-4">
                        Speed rating
                    </div>
                    <div class="table-container">
                        <div class="table table-compact table-interactive">
                            <thead>
                                <tr>
                                    <th>Name</th>
                                    <th>Rating</th>
                                    <th>Comfort</th>
                                </tr>
                            </thead>
                            <tbody>
                                {#each topLayouts as layout}
                                    <tr on:click={() => {goto(`/layout/${layout.id}-${layout.name}`)}}>
                                        <td>{layout.name}</td>
                                        <td>{layout.rating}</td>
                                        <td>{layout.rating_comfort}</td>
                                    </tr>
                                {/each}
                            </tbody>
                        </div>
                    </div>
                </div>
                <div>
                    <div class="p-4">
                        Comfort rating
                    </div>
                    <div class="table-container">
                        <div class="table table-compact table-interactive">
                            <thead>
                                <tr>
                                    <th>Name</th>
                                    <th>Rating</th>
                                    <th>Comfort</th>
                                </tr>
                            </thead>
                            <tbody>
                                {#each topComfortLayouts as layout}
                                    <tr on:click={() => {goto(`/layout/${layout.id}-${layout.name}`)}}>
                                        <td>{layout.name}</td>
                                        <td>{layout.rating}</td>
                                        <td>{layout.rating_comfort}</td>
                                    </tr>
                                {/each}
                            </tbody>
                        </div>
                    </div>
                </div>
            </div>
            <h2 class="h2">Full list</h2>
            <p>
                Click on table headers to sort them.
            </p>
            <div class="table-container">
                <div class="table table-compact table-interactive">
                    <thead>
                        <tr>
                            <th
                                class={sort=="id-asc" ? "table-sort-asc" : (sort=="id-dsc" ? "table-sort-dsc" : "")}
                                on:click={() => {sort=="id-asc" ? sort = "id-dsc" : sort = "id-asc"}}
                            >ID</th>
                            <th
                                class={sort=="name-asc" ? "table-sort-asc" : (sort=="name-dsc" ? "table-sort-dsc" : "")}
                                on:click={() => {sort=="name-asc" ? sort = "name-dsc" : sort = "name-asc"}}
                            >Name</th>
                            <th
                                class={sort=="rating-asc" ? "table-sort-asc" : (sort=="rating-dsc" ? "table-sort-dsc" : "")}
                                on:click={() => {sort=="rating-dsc" ? sort = "rating-asc" : sort = "rating-dsc"}}
                            >Rating</th>
                            <th
                                class={sort=="comfort-asc" ? "table-sort-asc" : (sort=="comfort-dsc" ? "table-sort-dsc" : "")}
                                on:click={() => {sort=="comfort-dsc" ? sort = "comfort-asc" : sort = "comfort-dsc"}}
                            >Comfort</th>
                            <th>Layout Data</th>
                        </tr>
                    </thead>
                    <tbody>
                        {#each layouts as layout}
                            <tr on:click={() => {goto(`/layout/${layout.id}-${layout.name}`)}}>
                                <td>{layout.id}</td>
                                <td>{layout.name}</td>
                                <td>{layout.rating}</td>
                                <td>{layout.rating_comfort}</td>
                                <td>{layout.layout_data}</td>
                            </tr>
                        {/each}
                    </tbody>
                </div>
            </div>
        {/if}
    </div>
</div>