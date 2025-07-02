import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import { ThemeProvider } from "./theme";
import { ThemeVariantEnum } from "./theme/theme.types";
import { RouterProvider } from "./Router/RouterProvider";

const root = createRoot(document.getElementById('root')!);

root.render(
    <StrictMode>
        <ThemeProvider themeVariant={ThemeVariantEnum.Light}>
            <RouterProvider />
        </ThemeProvider>
    </StrictMode>
);