import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";

import type { Drive } from "../types/Drive";
import type { ScanSummary } from "../types/ScanSummary";

export async function getEngineVersion(): Promise<string> {
    return await invoke("get_engine_version");
}

export async function getDrives(): Promise<Drive[]> {
    return await invoke("get_drives");
}

export async function startScan(path: string): Promise<void> {
    await invoke("start_scan", { path });
}

export async function onScanFinished(
    callback: (summary: ScanSummary) => void,
): Promise<() => void> {
    const unlisten = await listen<ScanSummary>(
        "scan-finished",
        (event) => {
            callback(event.payload);
        },
    );

    return unlisten;
}