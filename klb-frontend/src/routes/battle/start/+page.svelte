<script lang="ts">
	import { goto } from "$app/navigation";
	import KeyboardInput from "$lib/keyboard/KeyboardInput.svelte";
	import type { CreateBattleRequest, CreateBattleResponse } from "$lib/schema";
	import { getToastStore } from "@skeletonlabs/skeleton";

    const toastStore = getToastStore();
    let disableSubmit = false;

    async function toBattle(e: CustomEvent<{layoutData: string}>) {
        disableSubmit = true;

        let req: CreateBattleRequest = {
            base_layout_data: e.detail.layoutData,
            is_personal: false,
        }

        fetch("/api/battle", {
            method: "POST",
            headers: {
                "Content-Type": "application/json",
            },
            body: JSON.stringify(req),
        })
        .then(resp => resp.json().then((data: CreateBattleResponse) => {
            if(!resp.ok) throw {status: resp.status, data: data};
            console.log("start battle!")
            goto("/battle/" + data.id);
        }))
        .catch((err: {status: number, data: CreateBattleResponse}) => {
            toastStore.trigger({
                message: `error: ${err.data.error}, msg: ${err.data.error_message}`,
                background: "variant-filled-error"
            });
        });
    }
</script>

<div class="container mx-auto p-8 flex justify-center">
    <div class="space-y-10 flex flex-col">
        <h1 class="h1">Input base layer</h1>
        <p>
            Provide the keyboard layout that you are using right now. Use this same layout for the battle.
        </p>
        <p>
            <KeyboardInput on:submit={toBattle} {disableSubmit} />
        </p>
    </div>
</div>