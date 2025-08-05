import type { AppThemeType } from "./src/app/providers/ThemeProvider";

export type { AppState } from "./src/app/providers/ReduxProvider/store";

declare module "@emotion/react" {
  // eslint-disable-next-line @typescript-eslint/no-empty-object-type
  export interface Theme extends AppThemeType {}
}
