<script lang="ts" context="module">
    const REPEATS = 3;
    const ROUNDS = 3;
    // total words in a pair = REPEAT * ROUNDS * 2

    export interface SubmitData {
        times: number[][]
        comfort: number[]
    }
</script>

<script lang="ts">
	import { ALLOWED_CHARS } from "$lib/keyboard/KeyboardInput.svelte";
	import { ProgressRadial } from "@skeletonlabs/skeleton";

	import { createEventDispatcher, onMount } from "svelte";

    export let wordPairs: string[][];
    
    const dispatch = createEventDispatcher<{submit:SubmitData}>();

    // pre-process
    let wordsOk = true;

    for(let wordPair of wordPairs) {
        if(wordPair.length != 2) {
            wordsOk = false;
        }
    }

    if(!wordsOk) {
        console.log("error", wordPairs)
    }

    let sections: string[][] = [];

    for(let wordPair of wordPairs) {
        let words = [];
        for(let i=0; i<3; i++) words.push(wordPair[0]);
        for(let i=0; i<3; i++) words.push(wordPair[1]);
        for(let i=0; i<3; i++) words.push(wordPair[0]);
        for(let i=0; i<3; i++) words.push(wordPair[1]);
        sections.push(words);
    }

    // game elements
    let progress = 0;
    let topText = "";
    let wordCaretBox: HTMLDivElement;
    let wordBox: HTMLDivElement;
    let comfortBox: HTMLDivElement;
    let caret: HTMLDivElement;
    let selector: HTMLDivElement;

    let pairIndex = 0; // 0..len(wordPairs)
    let round = 0; // 0..=ROUNDS. when round==ROUNDS, it's the comfort stage
    let currWordIndex = 0; // 0..2
    let repeat = 0; // 0..REPEATS

    let currWord = "";
    let letterIndex = 0;
    let timeWordStart = 0;
    let timeSinceWordStart = 0;
    let lastTime = 0;

    let selected = 0;

    let times: number[][][] = [];
    let comfortPick: number[] = [];
    for(let i in wordPairs) {
        times.push([]);
        for(let j in wordPairs[i]) {
            times[i].push([]);
        }
    }

    let timerFrame: number;
    function step() {
        if(timeWordStart == 0) {
            timeSinceWordStart = 0;
        } else {
            timeSinceWordStart = Date.now() - timeWordStart;
        }
        timerFrame = requestAnimationFrame(step);
    }

    onMount(() => {
        render();
        moveCaret();
        moveSelector();
        timerFrame = requestAnimationFrame(step);
        return () => cancelAnimationFrame(timerFrame);
    });

    function isDone(): boolean {
        return pairIndex == wordPairs.length
    }

    function render() {
        if(isDone()) {
            comfortBox.classList.add("invisible");
            comfortBox.style.opacity = "0";
            topText = "All done!";
            return;
        }

        if(round < ROUNDS) {
            // render current word
            currWord = wordPairs[pairIndex][currWordIndex];
            wordBox.classList.remove("invisible");
            caret.hidden = false;
            // comfort box
            comfortBox.classList.add("invisible");
            comfortBox.style.opacity = "0";
            // top text
            topText = `Type ${currWord} | end it with a space`;
        } else {
            // render current word
            wordBox.classList.add("invisible");
            caret.hidden = true;

            comfortBox.classList.remove("invisible");
            comfortBox.style.opacity = "100";

            topText = "Which layout is more comfortable? (press 1 or 2 to select and space/enter to decide)"
        }
    }

    function moveCaret() {
        if(wordBox.children.length == 0) return;
        let letterSpan = wordBox.children[0] as HTMLSpanElement;
        caret.style.transform = `translate(${letterSpan.offsetLeft + letterIndex*letterSpan.offsetWidth}px, ${letterSpan.offsetTop}px)`;
    }

    function moveSelector() {
        let comfortButton = comfortBox.children[selected] as HTMLButtonElement;
        selector.style.transform = `translate(${comfortButton.offsetLeft}px, ${comfortButton.offsetTop}px)`;
        (selector.children[0] as HTMLDivElement).style.width = `${comfortButton.offsetWidth}px`;
    }

    function nextState() {
        if(isDone()) return;

        letterIndex = 0;
        if(round < ROUNDS) repeat++; // normal word
        else round++; // exit comfort stage

        if(repeat == REPEATS) {
            repeat = 0;
            currWordIndex++;
        }
        
        if(currWordIndex == 2) {
            currWordIndex = 0;
            round++;
        }
            
        if(round > ROUNDS) {
            round = 0;
            pairIndex++;
        }

        progress = (pairIndex * (ROUNDS + 1) * 2 * REPEATS
                    + round * 2 * REPEATS
                    + currWordIndex * REPEATS
                    + repeat)
                    / (wordPairs.length * (ROUNDS + 1) * 2 * REPEATS) * 100;
        
        if(isDone()) {
            submit();
        }
    }

    function resetWord() {
        for(let child of wordBox.children) {
            child.classList.remove("text-primary-500");
        }
    }

    // main logic here
    function onKeyDown(e: KeyboardEvent) {
        if(isDone()) return;

        if(round < ROUNDS) {
            if(ALLOWED_CHARS.includes(e.key) || e.key == " ") {
                let letterSpan = wordBox.children[letterIndex] as HTMLSpanElement;
                if(letterIndex == currWord.length && e.key == " ") { // correct spacebar
                    let timeTaken = Date.now() - timeWordStart;
                    times[pairIndex][currWordIndex].push(timeTaken);
                    lastTime = timeTaken;
                    timeWordStart = 0;
                    nextState();
                    render();
                    resetWord();
                } else if(letterIndex < currWord.length && e.key == letterSpan.innerText) { // correct letter
                    if(letterIndex == 0) timeWordStart = Date.now();
                    letterSpan.classList.add("text-primary-500");
                    letterIndex++;
                } else { // wrong
                    timeWordStart = 0;
                    wordCaretBox.classList.add("animate-shake");
                    resetWord();
                    letterIndex = 0;
                }
                moveCaret();
            }
        } else {
            if(e.key == "1") {
                selected = 0;
                moveSelector();
            } else if(e.key == "2") {
                selected = 1;
                moveSelector();
            } else if(e.key == " " || e.key == "Enter") {
                e.preventDefault();
                comfortPick.push(selected+1); // backend comfort choice is 1-based
                nextState();
                render();
            }
        }
    }

    function submit() {
        // select best times
        let bestTimes: number[][] = [];
        for(let i=0; i<wordPairs.length; i++) {
            bestTimes.push([]);
            for(let j=0; j<2; j++) {
                let timeArray = times[i][j];
                timeArray.sort((a, b) => (a-b));
                bestTimes[i].push(timeArray[0]+timeArray[1]+timeArray[2]);
            }
        }
        console.log("submit", bestTimes, comfortPick);

        dispatch("submit", {
            times: bestTimes,
            comfort: comfortPick,
        });
    }
</script>

<style>
    .animate-shake {
        animation: shake 200ms;
    }

    @keyframes shake {
        0% { transform: translate(1px, 1px) rotate(0deg); }
        10% { transform: translate(-1px, -2px) rotate(-1deg); }
        20% { transform: translate(-3px, 0px) rotate(1deg); }
        30% { transform: translate(3px, 2px) rotate(0deg); }
        40% { transform: translate(1px, -1px) rotate(1deg); }
        50% { transform: translate(-1px, 2px) rotate(-1deg); }
        60% { transform: translate(-3px, 1px) rotate(0deg); }
        70% { transform: translate(3px, 1px) rotate(-1deg); }
        80% { transform: translate(-1px, -1px) rotate(1deg); }
        90% { transform: translate(1px, 2px) rotate(0deg); }
        100% { transform: translate(1px, -2px) rotate(-1deg); }
    }
</style>

{#if wordsOk}
    <div class="p-6 space-y-4 flex flex-col h-full">
        <div class="text-3xl inline-flex">
            <ProgressRadial value={progress} width="w-8" stroke={100} />
            &nbsp;
            {topText}
        </div>
        <div hidden class="animate-shake"></div> <!-- to trick tailwind so that it doesn't remove the animate-shake -->
        <div class="h-full items-center flex">
            <div class="w-full text-center">
                <div
                    bind:this={wordCaretBox}
                    class="relative inline-block"
                    on:animationend={() => wordCaretBox.classList.remove("animate-shake")}
                >
                    <div bind:this={caret} class="absolute top-0 left-0 bg-warning-500 w-1 h-20 animate-pulse transition-all duration-200" hidden />
                    <div bind:this={wordBox} class="text-7xl invisible">
                        {#each currWord as letter}
                            <span>{letter}</span>
                        {/each}
                    </div>
                </div>
                <div bind:this={comfortBox} class="relative grid grid-cols-2 invisible">
                    <button class="btn variant-soft-secondary text-5xl" on:click={() => {selected = 0; moveSelector()}}>
                        {#if pairIndex < wordPairs.length}
                            [1] {wordPairs[pairIndex][0]}
                        {/if}
                    </button>
                    <button class="btn variant-soft-secondary text-5xl" on:click={() => {selected = 1; moveSelector()}}>
                        {#if pairIndex < wordPairs.length}
                            [2] {wordPairs[pairIndex][1]}
                        {/if}
                    </button>
                    <div bind:this={selector} class="absolute transition-all duration-200">
                        <div class="absolute border-4 border-primary-500 rounded-full h-16 animate-pulse" />
                    </div>
                </div>
            </div>
        </div>
        <div>
            Timer: {timeSinceWordStart} {#if lastTime != 0}({lastTime}){/if}
        </div>
        <div>
            <!-- bot text -->
            {#if pairIndex < wordPairs.length}
                Pair: {wordPairs[pairIndex][0]} vs {wordPairs[pairIndex][1]} | Round {round} | Repeat {repeat}
            {/if}
        </div>
    </div>
{:else}
    <div>Some Error Happened</div>
{/if}

<svelte:window on:keydown={onKeyDown}/>
