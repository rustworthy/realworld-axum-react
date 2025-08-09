import { AppThemeType } from "../theme.types";

const palette = {
  c00: "#55595c",
  c01: "#111111",
  c02: "#ffffff",
  c03: "#5CB85B",
  c04: "#449d44",
  c05: "#EFEFEF",
};

export const DarkTheme: AppThemeType = {
  page: {
    home: {
      bannner: {
        textColor: palette.c01,
        backgroundColor: palette.c03,
      },
    },
    signup: {
      signInLink: {
        textColor: palette.c03,
      },
    },
  },
  mainLayout: {
    logo: {
      textColor: palette.c03,
    },
    textColor: palette.c05,
    backgroundColor: palette.c01,
    footer: {
      textColor: palette.c02,
      backgroundColor: "linear-gradient(#485563, #29323c)",
    },
  },
  shared: {
    input: {
      textColor: palette.c05,
      borderColor: "#00000026",
      borderColorFocused: "#66afe9",
      backgroundColor: "#222222",
      errorColor: "#ff0000",
      otp: {
        separator: {
          backgroundColor: palette.c03,
        },
      },
    },
    button: {
      textColor: palette.c02,
      borderColor: palette.c03,
      backgroundColor: palette.c03,
      backgroundColorActive: palette.c04,
    },
  },
};
