<script lang="ts">
	import Keyboard from "$lib/keyboard/Keyboard.svelte";
    import type { PageData } from "./$types";

	export let data: PageData;
</script>

<div class="container mx-auto p-8 flex justify-center">
    <div class="space-y-10 flex flex-col">
        <h1 class="h1">{data.layout.name}</h1>
        <p>
            <Keyboard layoutData={data.layout.layout_data} />
        </p>
        <p>
            <Keyboard layoutData={data.layout.layout_data} heatmap={true} />
        </p>
        {#if data.layout.description}
            <p>
                {data.layout.description}
            </p>
        {/if}
        <p>
            Rating: {data.layout.rating}
        </p>
        <p>
            Comfort Rating: {data.layout.rating_comfort}
        </p>
        <p>
            Rating Data:
        </p>
        <pre>{JSON.stringify(data.layout.rating_data, null, 4)}</pre>
    </div>
</div>