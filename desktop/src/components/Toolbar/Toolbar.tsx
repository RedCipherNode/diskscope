import "./Toolbar.css";

type ToolbarProps = {
    canScan: boolean;
    scanning: boolean;
    onScan: () => void;
};

export function Toolbar({
    canScan,
    scanning,
    onScan,
}: ToolbarProps)  {
    return (
        <header className="toolbar">
            <h1>DiskScope</h1>

            <div className="toolbar-actions">
                <button
                    disabled={!canScan || scanning}
                    onClick={onScan}
                >
                    {scanning ? "Scanning..." : "Scan"}
                </button>

                <button>
                    Settings
                </button>
            </div>
        </header>
    );
}