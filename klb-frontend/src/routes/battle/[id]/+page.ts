import { handleFetchPromiseForLoad } from '$lib/api.js';
import type { GetBattleResponse } from '$lib/schema.js';

export interface PageData {
    id: string
    words: string[][]
}

export async function load({ fetch, params }): Promise<PageData> {
    let id = params.id;

    return handleFetchPromiseForLoad(fetch("/api/battle/" + id), (data: GetBattleResponse) => {
        return {
            id,
            words: data.words,
        };
    });
}