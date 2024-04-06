<script lang="ts">
	import '../app.postcss';
	import { AppShell, AppRail, AppRailAnchor, initializeStores, Toast, Modal } from '@skeletonlabs/skeleton';
	// import type { PageData } from './$types';

	// export let data: PageData;

	initializeStores();

	// console.log("Retrieve top level data layouts", data, data.layouts.get(2));

	// initialize chart js
	import {
		Chart as ChartJS,
		Title,
		Tooltip,
		Legend,
		LineElement,
		LinearScale,
		PointElement,
		CategoryScale,
	} from 'chart.js';
	import { onMount } from 'svelte';

	ChartJS.register(
		Title,
		Tooltip,
		Legend,
		LineElement,
		LinearScale,
		PointElement,
		CategoryScale
	);

	let defaultDiv: HTMLDivElement;
	let borderDiv: HTMLDivElement;
	onMount(() => {
		ChartJS.defaults.borderColor = getComputedStyle(borderDiv).color;
		ChartJS.defaults.color = getComputedStyle(defaultDiv).color;
	});
</script>

<Toast />
<Modal />

<!-- dummy component to get color value -->
<div hidden bind:this={defaultDiv}></div>
<div hidden bind:this={borderDiv} class="text-surface-500"></div>

<AppShell>
	<AppRail slot="sidebarLeft" width="w-40" aspectRatio="aspect-[3/1]" class="pb-6">
		<AppRailAnchor href="/" title="Home">
			<strong slot="lead" class="text-xl uppercase">KLB</strong>
			<span>Keyboard Layout Battles</span>
		</AppRailAnchor>
		<AppRailAnchor href="/battle">
			<span class="text-base">Battle</span>
		</AppRailAnchor>
		<AppRailAnchor href="/layout" title="Layouts">
			<span class="text-base">Layouts</span>
		</AppRailAnchor>
		<AppRailAnchor href="/guide" title="Guide">
			<span class="text-base">Guides</span>
		</AppRailAnchor>
		<svelte:fragment slot="trail">
			<AppRailAnchor
				aspectRatio="aspect-[4/1]"
				href="https://github.com/malfple/keyboard-layout-battles"
				target="_blank"
				rel="noreferrer"
				title="Github"
			>
				Github
			</AppRailAnchor>
			<AppRailAnchor
				aspectRatio="aspect-[4/1]"
				href="https://discord.gg/2qq8qmDtFf"
				target="_blank"
				rel="noreferrer"
				title="Discord"
			>
				Discord
			</AppRailAnchor>
		</svelte:fragment>
	</AppRail>
	<!-- Page Route Content -->

	<slot />
	<!-- artificial safelist because THE ACTUAL GODDAMN SAFELIST IS NOT WORKING IN PRODUCTION!!!!!!! -->
	<div class="hidden bg-secondary-100" />
	<div class="hidden bg-secondary-200" />
	<div class="hidden bg-secondary-300" />
	<div class="hidden bg-secondary-400" />
	<div class="hidden bg-secondary-500" />
	<div class="hidden bg-secondary-600" />
	<div class="hidden bg-secondary-700" />
	<div class="hidden bg-secondary-800" />
	<div class="hidden bg-secondary-900" />
</AppShell>
