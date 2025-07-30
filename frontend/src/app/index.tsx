import { StrictMode } from "react";
import { createRoot } from "react-dom/client";

import { ThemeVariantEnum } from "@/shared/types/theme.types";
import { Toaster as ToastProvider } from "sonner";

import { ReduxProvider } from "./providers/ReduxProvider";
import { RouterProvider } from "./providers/RouterProvider";
import { ThemeProvider } from "./providers/ThemeProvider";

const root = createRoot(document.getElementById("root")!);

root.render(
  <StrictMode>
    <ReduxProvider>
      <ThemeProvider themeVariant={ThemeVariantEnum.Light}>
        <RouterProvider />
        <ToastProvider position="top-center" />
      </ThemeProvider>
    </ReduxProvider>
  </StrictMode>,
);
