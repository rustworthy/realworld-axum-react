import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import App from "./App";
import "./global.css";
import { ThemeProvider } from "./theme";

const root = createRoot(document.getElementById('root')!);

root.render(
    <StrictMode>
        <ThemeProvider>
            <App />
        </ThemeProvider>
    </StrictMode>
);