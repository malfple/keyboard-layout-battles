<script>
	import KeyboardInput from "$lib/KeyboardInput.svelte";

</script>
<div class="container mx-auto p-8 flex justify-center">
    <div class="space-y-10 flex flex-col">
        <h1 class="h1">Input base layer</h1>
        <p>
            Provide the keyboard layout that you are using right now. Use this same layout for the battle.
        </p>
        <p>
            <KeyboardInput />
        </p>
    </div>
</div>