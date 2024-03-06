<script lang="ts" context="module">
    const letterColor: Map<string, string> = new Map();

    letterColor.set("e", "300");
    letterColor.set("a", "400");
    letterColor.set("r", "400");
    letterColor.set("i", "400");
    letterColor.set("o", "400");
    letterColor.set("t", "400");
    letterColor.set("n", "400");
    letterColor.set("s", "400");
    letterColor.set("l", "500");
    letterColor.set("c", "500");
    letterColor.set("h", "500");
    letterColor.set("u", "600");
    letterColor.set("d", "600");
    letterColor.set("p", "600");
    letterColor.set("m", "600");
    letterColor.set("g", "600");
    letterColor.set("b", "700");
    letterColor.set("f", "700");
    letterColor.set("y", "700");
    letterColor.set("w", "700");
    letterColor.set("k", "800");
    letterColor.set("v", "800");
    letterColor.set("x", "900");
    letterColor.set("z", "900");
    letterColor.set("j", "900");
    letterColor.set("q", "900");
</script>

<script lang="ts">
    export let char: string;
    export let heatmap: boolean;

    let colorClass = "variant-ghost-secondary";
    if(heatmap) {
        let grad = letterColor.get(char);
        if(grad == undefined) {
            colorClass = "bg-secondary-900";
        } else {
            colorClass = "bg-secondary-" + grad;
        }
    }
</script>

<div class="flex {colorClass} w-12 h-12 rounded-md">
    <span class="m-auto text-2xl">{char}</span>
</div>