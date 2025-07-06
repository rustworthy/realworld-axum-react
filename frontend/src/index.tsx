import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import { Provider as StateProvider } from "react-redux";

import { RouterProvider } from "./Router/RouterProvider";
import { store } from "./store";
import { ThemeProvider } from "./theme";
import { ThemeVariantEnum } from "./theme/theme.types";

const root = createRoot(document.getElementById("root")!);

root.render(
  <StrictMode>
    <StateProvider store={store}>
      <ThemeProvider themeVariant={ThemeVariantEnum.Light}>
        <RouterProvider />
      </ThemeProvider>
    </StateProvider>
  </StrictMode>,
);
