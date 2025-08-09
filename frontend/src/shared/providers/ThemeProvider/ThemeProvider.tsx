import type { FC, PropsWithChildren } from "react";

import { ThemeProvider as StyledThemeProvider } from "@emotion/react";

import { useTernaryDarkMode } from "usehooks-ts";

import { GlobalStyles } from "./globalStyles";
import { DarkTheme } from "./variants/DarkTheme";
import { LightTheme } from "./variants/LightTheme";

export const ThemeProvider: FC<PropsWithChildren> = ({ children }) => {
  const { isDarkMode } = useTernaryDarkMode();
  const theme = isDarkMode ? DarkTheme : LightTheme;

  return (
    <StyledThemeProvider theme={theme}>
      <>
        <GlobalStyles />
        {children}
      </>
    </StyledThemeProvider>
  );
};
