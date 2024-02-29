<script lang="ts">
	import { onMount } from "svelte";
	import type { GetLayoutListResponse, LayoutLite } from "$lib/api";
	import { goto } from "$app/navigation";

    let layouts: LayoutLite[];

    onMount(async () => {
        fetch("/api/layouts")
        .then(resp => resp.json())
        .then((data: GetLayoutListResponse) => {
            layouts = data.layouts;
        })
        .catch((err: GetLayoutListResponse) => {
            console.log(err);
        });
    })
</script>

<div class="container mx-auto p-8 flex justify-center">
    <div class="space-y-10 flex flex-col">
        <h1 class="h1">Layouts</h1>
        <p>
            List of layouts
        </p>
        {#if layouts}
            <div class="table-container">
                <div class="table table-interactive">
                    <thead>
                        <tr>
                            <th>ID</th>
                            <th>Name</th>
                            <th>Rating</th>
                            <th>Comfort</th>
                        </tr>
                    </thead>
                    <tbody>
                        {#each layouts as layout}
                            <tr on:click={() => {goto(`/layout/${layout.id}-${layout.name}`)}}>
                                <td>{layout.id}</td>
                                <td>{layout.name}</td>
                                <td>{layout.rating}</td>
                                <td>{layout.rating_comfort}</td>
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