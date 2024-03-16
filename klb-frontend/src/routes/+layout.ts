import type { GetLayoutListResponse, LayoutLite } from "$lib/schema";
import { error } from "@sveltejs/kit";

export interface PageData{
    layouts: Map<number, LayoutLite>
}

export async function load({ fetch, params }): Promise<PageData> {
    return fetch("/api/layouts")
    .then(resp => resp.json().then((data: GetLayoutListResponse) => {
        if(!resp.ok) throw {status: resp.status, data: data};
        return {
            layouts: convertLayoutsToMap(data.layouts),
        }
    }))
    .catch((err: {status: number, data: GetLayoutListResponse}) => {
        error(err.status, `error: ${err.data.error}, msg: ${err.data.error_message}`);
    });
}

function convertLayoutsToMap(layouts: LayoutLite[]): Map<number, LayoutLite> {
    let m = new Map();
    for(let layout of layouts) {
        m.set(layout.id, layout);
    }
    return m;
}
