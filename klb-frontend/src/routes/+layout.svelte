<script lang="ts">
	import '../app.postcss';
	import { AppShell, AppRail, AppRailAnchor, initializeStores, Toast, Modal } from '@skeletonlabs/skeleton';
	// import type { PageData } from './$types';

	// export let data: PageData;

	initializeStores();

	// console.log("Retrieve top level data layouts", data, data.layouts.get(2));
</script>

<Toast />
<Modal />

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
				aspectRatio="aspect-[5/1]"
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
</AppShell>
