import type { ToastStore } from "@skeletonlabs/skeleton";
import { error } from "@sveltejs/kit";

interface ResponseBase {
    error: string
    error_message: string
}

// handleFetchPromiseForLoad handles the promise from a fetch call
export async function handleFetchPromiseForLoad<PageDataType, RespType extends ResponseBase>(
    fetchPromise: Promise<Response>,
    handleData: (data: RespType) => PageDataType,
): Promise<PageDataType> {
    return fetchPromise
    .then(
        resp => resp.json()
        .then((data: RespType) => {
            if(!resp.ok) throw {status: resp.status, data: data};
            return handleData(data);
        })
        .catch(() => {
            throw {status: resp.status, data: {error_message: "fail to parse json"}}
        })
    )
    .catch((err: {status: number, data: RespType}) => {
        error(err.status, `error: ${err.data.error}, msg: ${err.data.error_message}`);
    });
}

// handleFetchPromise handles promise for client fetch calls, should not be called on server side (because it uses toast)
export async function handleFetchPromise<RespType extends ResponseBase>(
    fetchPromise: Promise<Response>,
    handleData: (data: RespType) => void,
    toastStore: ToastStore,
): Promise<void> {
    return fetchPromise
    .then(
        resp => resp.json()
        .then((data: RespType) => {
            if(!resp.ok) throw {status: resp.status, data: data};
            handleData(data);
        })
        .catch(() => {
            throw {status: resp.status, data: {error_message: "fail to parse json"}}
        })
    )
    .catch((err: {status: number, data: RespType}) => {
        toastStore.trigger({
            message: `error ${err.status}: ${err.data.error}, msg: ${err.data.error_message}`,
            background: "variant-filled-error"
        });
    });
}
