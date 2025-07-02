import { GlobalStyles } from './globalStyles';
import { ThemeProvider as StyledThemeProvider } from "styled-components";
import { LightTheme } from './variants/LightTheme';
import { FC, ReactNode } from 'react';
import { ThemeVariantEnum } from './theme.types';
import { DarkTheme } from './variants/DarkTheme';

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
    </StyledThemeProvider >
  )
};