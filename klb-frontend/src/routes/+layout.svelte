<script lang="ts">
	import KeyboardInputGuide from '$lib/guide/KeyboardInputGuide.svelte';
	import '../app.postcss';
	import { AppShell, AppRail, AppRailAnchor, initializeStores, Toast, Modal } from '@skeletonlabs/skeleton';

	initializeStores();
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
				href="https://discord.gg/EXqV7W8MtY"
				target="_blank"
				rel="noreferrer"
			>
				Discord
			</AppRailAnchor>
			<AppRailAnchor
				href="https://twitter.com/SkeletonUI"
				target="_blank"
				rel="noreferrer"
			>
				Twitter
			</AppRailAnchor>
			<AppRailAnchor
				href="https://github.com/skeletonlabs/skeleton"
				target="_blank"
				rel="noreferrer"
			>
				GitHub
			</AppRailAnchor>
			<AppRailAnchor aspectRatio="aspect-[5/1]" href="/login" title="Login">Sample Tail</AppRailAnchor>
		</svelte:fragment>
	</AppRail>
	<!-- Page Route Content -->

	<slot />
</AppShell>
