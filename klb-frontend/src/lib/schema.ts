export interface BattleHistoryLite {
    id: number
    layout_id_1: number
    layout_id_2: number
    layout_1_rating: number
    layout_2_rating: number
    rating_1_gain: number
    rating_2_gain: number
    is_personal: boolean
    time_created: number
}

export interface GetBattleHistoryListResponse {
    battles: BattleHistoryLite[]
    error: string
    error_message: string
}

export interface ResultWordData {
    original: string
    translated_1: string
    translated_2: string
    time_1: number
    time_2: number
    score: number
    comfort_choice: number
}

export interface ResultData {
    words: ResultWordData[]
    score: number
    comfort_score: number
}

export interface BattleHistory {
    id: number
    layout_id_1: number
    layout_id_2: number
    base_layout_data: string
    user_id_typer: number
    layout_1_rating: number
    layout_2_rating: number
    rating_1_gain: number
    rating_2_gain: number
    result_data: ResultData
    is_personal: boolean
    time_created: Number
}

export interface GetBattleHistoryResponse {
    battle: BattleHistory
    error: string
    error_message: string
}

export interface CreateBattleRequest {
    base_layout_data: string
    is_personal: boolean
}

export interface CreateBattleResponse {
    id: string
    words: string[][] // [wordPairs][2]
    error: string
    error_message: string
}

export interface GetBattleResponse {
    words: string[][] // [wordPairs][2]
    error: string
    error_message: string
}

export interface FinalizeBattleRequest {
    id: string
    times: number[][] // [wordPairs][2]
    comfort_choice: number[]
}

export interface FinalizeBattleResponse {
    layout_id_1: number
    layout_id_2: number
    result_data: ResultData
    error: string
    error_message: string
}

export interface LayoutLite {
    id: number
    name: string
    layout_data: string
    rating: number
    rating_comfort: number
}

export interface Layout {
    id: number
    sequence_id: number
    name: string
    layout_data: string
    description: string
    rating: number
    rating_comfort: number
    rating_data: any
    time_created: number
    time_modified: number
}

export interface GetLayoutListResponse {
    layouts: LayoutLite[]
    error: string
    error_message: string
}

export interface GetLayoutResponse {
    layout: Layout
    error: string
    error_message: string
}
