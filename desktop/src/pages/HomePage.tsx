import { useEffect, useState } from "react";

import { Toolbar } from "../components/Toolbar/Toolbar";
import { DriveBar } from "../components/DriveBar/DriveBar";
import { Content } from "../components/Content/Content";
import { Treemap } from "../components/Treemap/Treemap";

import {
    getDrives,
    startScan,
    onScanFinished,
} from "../services/engine";

import type { Drive } from "../types/Drive";
import type { ScanSummary } from "../types/ScanSummary";

export function HomePage() {
    const [drives, setDrives] = useState<Drive[]>([]);
    const [selectedDrive, setSelectedDrive] = useState("");

    const [summary, setSummary] = useState<ScanSummary | null>(null);

    const [scanning, setScanning] = useState(false);

    useEffect(() => {
        getDrives().then(setDrives);

        let unlisten: (() => void) | undefined;

        onScanFinished((summary) => {
            setSummary(summary);
            setScanning(false);
        }).then((callback) => {
            unlisten = callback;
        });

        return () => {
            unlisten?.();
        };
    }, []);

    async function handleScan() {
        if (!selectedDrive || scanning) {
            return;
        }

        setSummary(null);
        setScanning(true);

        await startScan(selectedDrive);
    }

    return (
        <>
            <Toolbar
                canScan={selectedDrive !== ""}
                scanning={scanning}
                onScan={handleScan}
            />

            <DriveBar
                drives={drives}
                selectedDrive={selectedDrive}
                onDriveChange={setSelectedDrive}
                canSearch={summary !== null}
            />

            <Content summary={summary} />

            <Treemap />
        </>
    );
}