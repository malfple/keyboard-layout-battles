import type { Handle } from "@sveltejs/kit";

const BACKEND_URL = "https://klb-backend.fly.dev"
const PROXY_PATH = "/api";

const handleAPIProxy: Handle = async ({event}) => {
    // strip `/api-proxy` from the request path
    const strippedPath = event.url.pathname.substring(PROXY_PATH.length);

    // build the new URL path with your API base URL, the stripped path and the query string
    const urlPath = `${BACKEND_URL}${strippedPath}${event.url.search}`;
    const proxiedUrl = new URL(urlPath);

    return fetch(proxiedUrl.toString(), {
        // propagate the request method and body
        body: event.request.body,
        method: event.request.method,
        headers: event.request.headers,
    }).catch((err) => {
        console.log("Could not proxy API request: ", err);
        throw err;
    });
}

export const handle: Handle = async ({event, resolve}) => {
    // intercept requests to `/api-proxy` and handle them with `handleApiProxy`
    if (event.url.pathname.startsWith(PROXY_PATH)) {
        return await handleAPIProxy({ event, resolve });
    }

    const response = await resolve(event);
    return response
}
