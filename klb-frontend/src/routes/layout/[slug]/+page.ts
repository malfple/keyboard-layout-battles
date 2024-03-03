import type { GetLayoutResponse, Layout } from '$lib/api';
import { error } from '@sveltejs/kit';

export interface PageData{
    id: number
    layout: Layout
}

export async function load({ fetch, params }): Promise<PageData> {
    var id = parseInt(params.slug.split("-", 1)[0]);

    return fetch("/api/layout/" + id)
    .then(resp => resp.json())
    .then((data: GetLayoutResponse) => {
        return {
            id,
            layout: data.layout,
        }
    })
    .catch((err: GetLayoutResponse) => {
        error(404, err.error_message);
    })
}