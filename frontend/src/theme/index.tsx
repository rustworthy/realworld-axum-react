import { FC, ReactNode } from "react";

import { ThemeProvider as StyledThemeProvider } from "styled-components";

import { GlobalStyles } from "./globalStyles";
import { ThemeVariantEnum } from "./theme.types";
import { DarkTheme } from "./variants/DarkTheme";
import { LightTheme } from "./variants/LightTheme";

interface IThemeWrapper {
  themeVariant?: ThemeVariantEnum;
  children: ReactNode;
}

export const ThemeProvider: FC<IThemeWrapper> = ({ themeVariant = ThemeVariantEnum.Light, children }) => {
  const theme = themeVariant === ThemeVariantEnum.Light ? LightTheme : DarkTheme;

  return (
    <StyledThemeProvider theme={theme}>
      <>
        <GlobalStyles />
        {children}
      </>
    </StyledThemeProvider>
  );
};
