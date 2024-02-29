<script lang="ts">
	import type { PageData } from "./+page";

	export let data: PageData;
</script>

<div class="container mx-auto p-8 flex justify-center">
    <div class="space-y-10 flex flex-col">
        <h1 class="h1">{data.layout.name}</h1>
        <p>
            {data.layout.rating}
        </p>
    </div>
</div>