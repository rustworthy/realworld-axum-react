import { AppThemeType } from "./src/app/providers/ThemeProvider";

declare module "styled-components" {
  // eslint-disable-next-line @typescript-eslint/no-empty-object-type
  export interface DefaultTheme extends AppThemeType {}
}
