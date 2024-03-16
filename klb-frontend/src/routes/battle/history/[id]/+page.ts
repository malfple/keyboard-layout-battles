import type { BattleHistory, GetBattleHistoryResponse } from '$lib/schema.js';
import { error } from '@sveltejs/kit';

export interface PageData {
    battle_history: BattleHistory
}

export async function load({ fetch, params }): Promise<PageData> {
    let id = params.id;

    return fetch("/api/battle/history/" + id)
    .then(resp => resp.json().then((data: GetBattleHistoryResponse) => {
        if(!resp.ok) throw {status: resp.status, data: data};
        return {
            battle_history: data.battle,
        }
    }))
    .catch((err: {status: number, data: GetBattleHistoryResponse}) => {
        error(err.status, `error: ${err.data.error}, msg: ${err.data.error_message}`);
    });
}