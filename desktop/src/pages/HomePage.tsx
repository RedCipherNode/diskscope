import { Toolbar } from "../components/Toolbar/Toolbar";
import { DriveBar } from "../components/DriveBar/DriveBar";
import { Content } from "../components/Content/Content";
import { Treemap } from "../components/Treemap/Treemap";

export function HomePage() {
    return (
        <>
            <Toolbar />
            <DriveBar />
            <Content />
            <Treemap />
        </>
    );
}