<script lang="ts">
	import { onMount } from "svelte";
	import type { GetLayoutListResponse, LayoutLite } from "$lib/schema";
	import { goto } from "$app/navigation";
	import { getToastStore } from "@skeletonlabs/skeleton";
	import { handleFetchPromise } from "$lib/api";

    const toastStore = getToastStore();
    let layouts: LayoutLite[];

    onMount(async () => {
        handleFetchPromise(fetch("/api/layouts"), (data: GetLayoutListResponse) => {
            layouts = data.layouts;
        }, toastStore);
    })
</script>

<div class="container mx-auto p-8 flex justify-center">
    <div class="space-y-10 flex flex-col">
        <h1 class="h1">Layouts</h1>
        <p>
            List of layouts.
            "Rating" typically refers to speed rating which is measured by the typing time
            and "Comfort" refers to the comfort rating which based on the typer's preference.
        </p>
        {#if layouts}
            <div class="table-container">
                <div class="table table-compact table-interactive">
                    <thead>
                        <tr>
                            <th>ID</th>
                            <th>Name</th>
                            <th>Rating</th>
                            <th>Comfort</th>
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
        {:else}
            <div class="placeholder" />
        {/if}
    </div>
</div>