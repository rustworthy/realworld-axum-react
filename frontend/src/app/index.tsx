import { StrictMode } from "react";
import { createRoot } from "react-dom/client";

import { ThemeProvider, ToastProvider } from "@/shared/providers";

import { ReduxProvider } from "./providers/ReduxProvider";
import { RouterProvider } from "./providers/RouterProvider";

const root = createRoot(document.getElementById("root")!);

root.render(
  <StrictMode>
    <ReduxProvider>
      <ThemeProvider>
        <RouterProvider />
        <ToastProvider />
      </ThemeProvider>
    </ReduxProvider>
  </StrictMode>,
);
