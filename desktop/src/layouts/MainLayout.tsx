import "../styles/layout.css";

type Props = {
    children: React.ReactNode;
};

export function MainLayout({ children }: Props) {
    return (
        <div className="app">
            {children}
        </div>
    );
}