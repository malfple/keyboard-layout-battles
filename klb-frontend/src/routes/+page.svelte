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
				</p>
				<h2 class="h2">Discord</h2>
				<p>
					There is a
					<a
						class="text-primary-500 underline"
						href="https://discord.gg/2qq8qmDtFf"
						target="_blank"
						rel="noreferrer"
					>
						discord server
					</a> for alternative keyboard layouts.
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
