import { AppThemeType } from "./src/shared/providers/ThemeProvider";

declare module "@emotion/react" {
  // eslint-disable-next-line @typescript-eslint/no-empty-object-type
  export interface Theme extends AppThemeType {}
}

declare const __ENV__: "production" | "development";
