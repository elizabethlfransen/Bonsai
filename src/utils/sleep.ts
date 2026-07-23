export default function sleep(ms: number, abortSignal?: AbortSignal): Promise<void> {
    return new Promise((resolve) => {
        const timeout = setTimeout(() => resolve(), ms);
        if(abortSignal)
            abortSignal.onabort = () => clearTimeout(timeout);
    });
}