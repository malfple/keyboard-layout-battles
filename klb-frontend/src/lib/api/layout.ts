export interface LayoutLite {
    id: number
    name: string
    layout_data: string
    rating: number
    rating_comfort: number
}

export interface GetLayoutListResponse {
    layouts: LayoutLite[]
    error: string
    error_message: string
}

export function getLayoutList(): Promise<GetLayoutListResponse> {
    return fetch("/api/layouts")
    .then(resp => resp.json())
    .then(data => {
        return data as GetLayoutListResponse
    })
    .catch(err => {
        return err as GetLayoutListResponse
    });
}