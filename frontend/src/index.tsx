import { StrictMode } from "react";
import { createRoot } from "react-dom/client";

import { RouterProvider } from "./Router/RouterProvider";
import { ThemeProvider } from "./theme";
import { ThemeVariantEnum } from "./theme/theme.types";

const root = createRoot(document.getElementById("root")!);

root.render(
  <StrictMode>
    <ThemeProvider themeVariant={ThemeVariantEnum.Light}>
      <RouterProvider />
    </ThemeProvider>
  </StrictMode>,
);
