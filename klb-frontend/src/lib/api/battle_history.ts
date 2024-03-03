export interface BattleHistoryLite {
    id: number,
    layout_id_1: number,
    layout_id_2: number,
    layout_1_rating: number,
    layout_2_rating: number,
    rating_1_gain: number,
    rating_2_gain: number,
    is_personal: boolean,
    time_created: number,
}

export interface GetBattleHistoryListResponse {
    battles: BattleHistoryLite[]
}