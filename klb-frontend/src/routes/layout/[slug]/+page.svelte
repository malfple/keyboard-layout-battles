<script lang="ts">
	import Keyboard from "$lib/keyboard/Keyboard.svelte";
    import type { PageData } from "./+page";

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
        <p>
            Rating: {data.layout.rating}
        </p>
        <p>
            Comfort Rating: {data.layout.rating_comfort}
        </p>
    </div>
</div>