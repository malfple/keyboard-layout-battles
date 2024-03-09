import type { GetLayoutResponse, Layout } from '$lib/schema';
import { error } from '@sveltejs/kit';

export interface PageData{
    id: number
    layout: Layout
}

export async function load({ fetch, params }): Promise<PageData> {
    var id = parseInt(params.slug.split("-", 1)[0]);

    if(!id) {
        error(500, "fail to parse param");
    }

    return fetch("/api/layout/" + id)
    .then(resp => resp.json().then((data: GetLayoutResponse) => {
        if(!resp.ok) throw {status: resp.status, data: data};
        return {
            id,
            layout: data.layout,
        }
    }))
    .catch((err: {status: number, data: GetLayoutResponse}) => {
        error(err.status, `error: ${err.data.error}, msg: ${err.data.error_message}`);
    });
}