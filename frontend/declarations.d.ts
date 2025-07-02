import { AppThemeType } from "./src/theme/theme.types";

declare module "styled-components" {
  export interface DefaultTheme extends AppThemeType {}
}