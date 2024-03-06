<!-- Key marker requires the parent to have "relative" class -->

<script lang="ts">
    export let top = 0;
    export let left = 0;
    export let ping: boolean;
</script>

<div class="absolute transition-all duration-200" style="top: {top}px; left: {left}px">
    <div class="absolute h-12 w-12 rounded-md border-2 border-error-500"></div>
    <div class="absolute h-12 w-12 rounded-md border-2 border-error-500 animate-ping" hidden={!ping}></div>
</div>