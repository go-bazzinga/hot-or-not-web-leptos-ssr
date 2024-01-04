import WaspHlsPlayer from "https://esm.sh/wasp-hls@0.4.2";

export function buildPlayer(videoElement, config) {
    const wasp = new WaspHlsPlayer(videoElement, config);
    wasp.initialize({
        workerUrl: "/wasp/worker.js",
        wasmUrl: "/wasp/wasp_hls_bg.wasm",
    });

    return wasp;
}