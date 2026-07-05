import { MainLayout } from "./layouts/MainLayout";
import { HomePage } from "./pages/HomePage";
import "./styles/variables.css";
import "./styles/globals.css";

function App() {
    return (
        <MainLayout>
            <HomePage />
        </MainLayout>
    );
}

export default App;