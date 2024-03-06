<!-- Key marker requires the parent to have "relative" class -->

<script lang="ts">
    export let top = 0;
    export let left = 0;
    export let ping: boolean;
</script>

<div class="absolute top-0 left-0 transition-all duration-200" style="transform: translate({left}px, {top}px);">
    <div class="absolute h-12 w-12 rounded-md border-2 border-error-500"></div>
    <div class="absolute h-12 w-12 rounded-md border-2 border-error-500 animate-ping" hidden={!ping}></div>
</div>