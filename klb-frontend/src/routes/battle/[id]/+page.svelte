<script lang="ts">
	import type { PageData } from "./+page";

    export let data: PageData

    console.log(data.words);
</script>

<div class="container mx-auto p-8 flex justify-center">
    <div class="space-y-10 flex flex-col">
        <h1 class="h1">Battle {data.id}</h1>
        <p>
            {data.words}
        </p>
    </div>
</div>