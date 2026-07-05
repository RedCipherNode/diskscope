type ContentProps = {
    summary: {
        total_files: number;
        total_directories: number;
        total_size: number;
    } | null;
};

export function Content({ summary }: ContentProps) {
    return (
        <main className="content">
            <aside>
                Tree
            </aside>

            <section>
                <h2>Details</h2>

                {!summary && (
                    <p>Select a drive and click Scan.</p>
                )}

                {summary && (
                    <>
                        <p>Total Files : {summary.total_files}</p>

                        <p>Total Directories : {summary.total_directories}</p>

                        <p>Total Size : {summary.total_size}</p>
                    </>
                )}
            </section>
        </main>
    );
}