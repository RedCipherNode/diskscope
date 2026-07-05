import type { Drive } from "../../types/Drive";

type DriveBarProps = {
    drives: Drive[];
    selectedDrive: string;
    onDriveChange: (value: string) => void;
    canSearch: boolean;
};

export function DriveBar({
    drives,
    selectedDrive,
    onDriveChange,
    canSearch,
}: DriveBarProps) {
    return (
        <section>
            <select
                value={selectedDrive}
                onChange={(event) => onDriveChange(event.target.value)}
            >
                <option value="">
                    Select Drive
                </option>

                {drives.map((drive) => (
                    <option
                        key={drive.path}
                        value={drive.path}
                    >
                        {drive.name}
                    </option>
                ))}
            </select>

            <input
                type="text"
                placeholder={
                    canSearch
                        ? "Search files or folders..."
                        : "Search after scanning..."
                }
                disabled={!canSearch}
            />
        </section>
    );
}