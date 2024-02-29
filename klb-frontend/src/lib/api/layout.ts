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
