import { useEffect, useState } from "react";
import { getEngineVersion } from "./services/engine";

function App() {
    const [version, setVersion] = useState("Loading...");

    useEffect(() => {
        async function load() {
            try {
                const version = await getEngineVersion();
                setVersion(version);
            } catch (error) {
                console.error(error);
                setVersion("Error");
            }
        }

        load();
    }, []);

    return (
        <main>
            <h1>DiskScope</h1>
            <p>Engine Version: {version}</p>
        </main>
    );
}

export default App;