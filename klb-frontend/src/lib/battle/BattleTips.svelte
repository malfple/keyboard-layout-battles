<script lang="ts" context="module">
    const TIPS: string[] = [
        "Type type type!",
        "End every word with a spacebar!",
        "Timer only begins when you press the first letter and until you hit the spacebar!",
        "Pause after every spacebar and think about the finger movement!",
        "Picked the wrong comfortable word? Just refresh the page! (This will reset the whole test).",
        "Mistakes will reset the word and the timer!",
        "Just for this test, making mistakes is better than typing slow!",
        "Take your time thinking about which keys to press before every word!",
        "Minimize pausing in the middle of a word! Press space to restart from the beginning.",
        "Type the whole word in 1 short burst! Or 2 if not possible.",
        "Pick the comfort based on the finger movement rather than how hard it is to type the word!",
    ];
</script>

<script>
    let tips = structuredClone(TIPS);

    // random shuffle
    for (let i = tips.length - 1; i>0; i--) {
        const j = Math.floor(Math.random() * (i + 1));
        [tips[i], tips[j]] = [tips[j], tips[i]];
    } 
</script>

<style lang="postcss">
    .animate-marquee {
        animation: marquee 180s linear infinite;
    }

    @keyframes marquee {
        0% { transform: translateX(0%); }
        100% { transform: translateX(-100%); }
    }
</style>

<div class="overflow-x-hidden text-xl">
    <div class="flex animate-marquee flex-nowrap">
        <div class="inline-block w-screen"></div>
        {#each tips as tip}
            <div class="whitespace-nowrap">{tip}</div>
            <div class="inline-block w-[50vw]"></div>
        {/each}
        <div class="inline-block w-screen"></div>
    </div>
</div>
