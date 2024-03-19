<script lang="ts">
	import { goto } from "$app/navigation";
	import type { LayoutLite } from "$lib/schema";
	import type { PageData } from "./$types";

	export let data: PageData;

	let topLayouts: LayoutLite[] = [];
	data.layouts.forEach((layout) => {
		topLayouts.push(layout);
	});
	topLayouts.sort((a, b) => {
		return b.rating - a.rating;
	});
	if(topLayouts.length > 10) topLayouts.length = 10;
</script>

<div class="container h-full p-8 mx-auto flex justify-center">
	<div class="space-y-20 flex flex-col items-center">
		<h1 class="h1">Welcome to Keyboard Layout Battles!</h1>
		<div class="grid grid-cols-2 gap-10 p-4">
			<div class="space-y-10">
				<h2 class="h2">What is Keyboard Layout Battles?</h2>
				<p>
					Keyboard Layout Battles provides an environment where alternative keyboard layouts can be compared to each other
					in a subjective way by having a human typer be a judge of a match between 2 keyboard layouts.
					Scores can be accumulated and ratings can be calculated for each layout similar to chess.
				</p>
				<h2 class="h2">Learn about alt keyboard layouts</h2>
				<p>
					If you are unfamiliar with alternative keyboard layouts,
					here are some links that can get you started.
				</p>
				<div>
					<ul class="list-nav">
						<li>
							<a href="https://getreuer.info/posts/keyboards/alt-layouts/index.html" target="_blank" rel="noreferrer">
								Pascal Getreuer's guide to alt keyboard layouts
							</a>
						</li>
						<li>
							<a href="https://www.reddit.com/r/KeyboardLayouts/comments/fevb94/a_brief_summary_of_alternative_keyboard_layout/" target="_blank" rel="noreferrer">
								r/KeyboardLayouts
							</a>
						</li>
						<li>
							<a href="https://discord.gg/2qq8qmDtFf" target="_blank" rel="noreferrer">
								AKL discord server
							</a>
						</li>
					</ul>
				</div>
				<h2 class="h2">Issues or suggestions</h2>
				<p>
					If you find any bugs or you have any suggestions in mind,
					feel free to submit your thoughts on the
					<a class="text-primary-500 underline" href="https://github.com/malfple/keyboard-layout-battles/issues" target="_blank">
						Github issues page.
					</a>
				</p>
			</div>
			<div class="space-y-4">
				<h2 class="h2">Top Layouts</h2>
				<div class="table-container">
					<div class="table table-compact table-interactive">
						<thead>
							<tr>
								<th>Name</th>
								<th>Rating</th>
								<th>Comfort</th>
							</tr>
						</thead>
						<tbody>
							{#each topLayouts as layout}
								<tr on:click={() => {goto(`/layout/${layout.id}-${layout.name}`)}}>
									<td>{layout.name}</td>
									<td>{layout.rating}</td>
									<td>{layout.rating_comfort}</td>
								</tr>
							{/each}
						</tbody>
					</div>
				</div>
			</div>
		</div>
	</div>
</div>
