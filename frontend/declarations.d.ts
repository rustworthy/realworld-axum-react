import { AppThemeType } from "./src/app/providers/ThemeProvider";

declare module "@emotion/react" {
  // eslint-disable-next-line @typescript-eslint/no-empty-object-type
  export interface Theme extends AppThemeType {}
}
