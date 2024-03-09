import type { GetBattleResponse } from '$lib/schema.js';
import { error } from '@sveltejs/kit';

export interface PageData {
    id: string
    words: string[][]
}

export function load({ fetch, params }): Promise<PageData> {
    let id = params.id;

    return fetch("/api/battle/" + id)
    .then(resp => resp.json().then((data: GetBattleResponse) => {
        if(!resp.ok) throw {status: resp.status, data: data};
        return {
            id,
            words: data.words,
        }
    }))
    .catch((err: {status: number, data: GetBattleResponse}) => {
        error(err.status, `error: ${err.data.error}, msg: ${err.data.error_message}`);
    });
}