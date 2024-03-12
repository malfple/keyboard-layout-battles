<script lang="ts">
	import type { SvelteComponent } from 'svelte';
    import { getModalStore } from '@skeletonlabs/skeleton';
    import preonicInput from '$lib/assets/preonic-input.png';
    import chocofiInput from '$lib/assets/chocofi-input.png';
    import ibmModelM from '$lib/assets/ibm-model-m-1987.jpg';
    import ibmModelMInput from '$lib/assets/ibm-model-m-1987-input.gif';
    import angleModInput from '$lib/assets/angle-mod-input.png';

    export let parent: SvelteComponent;

    const modalStore = getModalStore();
</script>

{#if $modalStore[0]}
	<div class="card p-8 w-modal-wide h-[90vh] shadow-xl space-y-10 overflow-auto">
        <input class="hidden" /> <!-- So that the modal starts at the top -->
		<h1 class="h1">How to input your current keyboard layout</h1>
		<p>
            First of all, let's talk about touch typing and the home row method.
            This is important as it affects the accuracy of the typing experience and scoring.
        </p>

        <h2 class="h2">Touch Typing and Home Row Method</h2>
        <p>
            There are a lot of resources only that you can find regarding touch typing
            like this <a class="text-primary-500 underline" href="https://en.wikipedia.org/wiki/Touch_typing" target="_blank">wikipedia page</a>.
            What's important is that you have to always use the same finger for a particular key (for example, always using middle finger to type "c" in QWERTY)
            to keep the experience consistent.
        </p>
        <p>
            Let's see how we can input for different types of keyboards.
        </p>

        <h2 class="h2">Ortholinear Keyboard</h2>
        <p>
            Ortholinear or Matrix keyboards are the easiest because they are arranged in a grid pattern.
            Therefore you can input the keyboard layout to the input matrix fairly intuitively.
        </p>
        <figure class="variant-ringed-primary p-4 rounded-md w-modal">
            <img alt="Preonic Keyboard" src={preonicInput} />
            <figcaption class="text-sm">Preonic Keyboard from DROP</figcaption>
        </figure>

        <h2 class="h2">Columnar Stagger Keyboard</h2>
        <p>
            Columnar stagger keyboards are also fairly straightforward.
            They are symmetrical and if you have a columnar keyboard, chances are you already know how to touch type.
            Inputting your layout for them is almost as easy as ortholinear keyboards.
        </p>
        <p>
            One rare case is if your keyboard has missing keys.
            In that case you can substitute it with another key such as a thumb key, but it is not optimal.
            There may be updates in the future to address this issue.
        </p>
        <figure class="variant-ringed-primary p-4 rounded-md w-modal">
            <img alt="Chocofi Keyboard" src={chocofiInput} />
            <figcaption class="text-sm">A small chocofi keyboard</figcaption>
        </figure>

        <h2 class="h2">Normal Keyboard</h2>
        <p>
            Normal row-staggered keyboards are not symmetrical so they may not be as straightforward,
            but the finger usage should still be the same as the previous types of keyboards.
            They just look a bit slanted.
        </p>
        <figure class="variant-ringed-primary p-4 rounded-md w-modal">
            <img alt="Chocofi Keyboard" src={ibmModelM} />
            <figcaption class="text-sm">A classic IBM Model M Keyboard</figcaption>
        </figure>
        <figure class="variant-ringed-primary p-4 rounded-md w-modal">
            <img alt="Chocofi Keyboard" src={ibmModelMInput} />
            <figcaption class="text-sm">And how to input it</figcaption>
        </figure>

        <p>
            Often, normal keyboard are modded with angle mod, wide mod, etc.
            If you are used to these mods,
            remember that the fingers you would use for the input matrix stays the same
            and the columns are consistent regardless of the
            mod that you use.
        </p>
        <p>
            A common mod is the angle mod like shown on
            <a class="text-primary-500 underline" href="https://colemakmods.github.io/ergonomic-mods/angle.html" target="_blank">Colemak-DH's page.</a>
            One tricky example is when you use angle mod on an ANSI keyboard where you have to relocate the "Z" key.
            If you use that, then when inputting the layout, you would still put in "Z" before "X" as if "Z" is hit by your pinky.
        </p>
        <figure class="variant-ringed-primary p-4 rounded-md w-modal">
            <img alt="Angle Mod" src={angleModInput} />
            <figcaption class="text-sm">Some examples of angle mod</figcaption>
        </figure>
        <p>
            At the end, the input matrix should be the same regardless of the mod that you use.
        </p>
        <button class="btn {parent.buttonNeutral}" on:click={parent.onClose}>Close</button>
	</div>
{/if}