import { handleFetchPromiseForLoad } from '$lib/api.js';
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

    return handleFetchPromiseForLoad(fetch("/api/layout/" + id), (data: GetLayoutResponse) => {
        return {
            id,
            layout: data.layout,
        };
    });
}