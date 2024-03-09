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

export interface CreateBattleRequest {
    base_layout_data: string
    is_personal: boolean
}

export interface CreateBattleResponse {
    id: string
    words: string[][]
    error: string
    error_message: string
}

export interface GetBattleResponse {
    words: string[][]
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
