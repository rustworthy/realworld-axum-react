import { StrictMode } from "react";
import { createRoot } from "react-dom/client";

import { ReduxProvider, ThemeProvider } from "@/shared/providers";
import { Toaster as ToastProvider } from "sonner";

import { RouterProvider } from "./providers/RouterProvider";

const root = createRoot(document.getElementById("root")!);

root.render(
  <StrictMode>
    <ReduxProvider>
      <ThemeProvider>
        <RouterProvider />
        <ToastProvider position="top-center" />
      </ThemeProvider>
    </ReduxProvider>
  </StrictMode>,
);
