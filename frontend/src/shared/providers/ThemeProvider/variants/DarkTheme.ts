import { AppThemeType } from "../theme.types";

const palette = {
  c00: "#55595c",
  c01: "#111111",
  c02: "#ffffff",
  c03: "#5CB85B",
  c04: "#449d44",
};

export const DarkTheme: AppThemeType = {
  page: {
    home: {
      c01: palette.c01,
      c02: palette.c01,
      c03: palette.c03,
    },
    signup: {
      c01: palette.c03,
    },
  },

  mainLayout: {
    c01: palette.c02,
    c02: palette.c01,
    header: {
      c01: palette.c03,
      c02: palette.c02,
    },
    footer: {
      c01: palette.c02,
      c02: "linear-gradient(#485563, #29323c)",
    },
  },

  shared: {
    input: {
      textColor: palette.c00,
      borderColor: "rgba(0, 0, 0, 0.15)",
      backgroundColor: "#ffffff",
      backgroundColorFocused: "#66afe9",
      errorColor: "#ff0000",
    },
    button: {
      c01: palette.c01,
      c02: palette.c02,
      c03: palette.c03,
      c04: palette.c04,
    },
  },
};
