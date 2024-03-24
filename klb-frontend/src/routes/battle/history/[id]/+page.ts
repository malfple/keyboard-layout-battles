import { handleFetchPromiseForLoad } from '$lib/api.js';
import type { BattleHistory, GetBattleHistoryResponse } from '$lib/schema.js';

export interface PageData {
    battle_history: BattleHistory
}

export async function load({ fetch, params }): Promise<PageData> {
    let id = params.id;

    return handleFetchPromiseForLoad(fetch("/api/battle/history/" + id), (data: GetBattleHistoryResponse) => {
        return {
            battle_history: data.battle,
        }
    });
}