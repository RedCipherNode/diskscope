import { invoke } from "@tauri-apps/api/core";

export async function getEngineVersion(): Promise<string> {
    return await invoke("get_engine_version");
}