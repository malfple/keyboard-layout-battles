import { handleFetchPromiseForLoad } from "$lib/api";
import type { GetLayoutListResponse, LayoutLite } from "$lib/schema";

export interface PageData{
    layouts: Map<number, LayoutLite>
}

export async function load({ fetch, params }): Promise<PageData> {
    return handleFetchPromiseForLoad(fetch("/api/layouts"), (data: GetLayoutListResponse) => {
        return {
            layouts: convertLayoutsToMap(data.layouts),
        };
    });
}

function convertLayoutsToMap(layouts: LayoutLite[]): Map<number, LayoutLite> {
    let m = new Map();
    for(let layout of layouts) {
        m.set(layout.id, layout);
    }
    return m;
}
