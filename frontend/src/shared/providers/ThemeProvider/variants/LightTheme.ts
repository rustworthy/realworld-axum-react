const palette = {
  c00: "#55595c",
  c01: "#111111",
  c02: "#ffffff",
  c03: "#5CB85B",
  c04: "#449d44",
  c05: "#B85C5C",
  c06: "#CCCCCC",
};

export const LightTheme = {
  page: {
    home: {
      banner: {
        textColor: palette.c02,
        backgroundColor: palette.c03,
      },
    },
    article: {
      banner: {
        textColor: palette.c02,
        backgroundColor: "#333",
      },
      content: {
        textColor: palette.c01,
        backgroundColor: palette.c02,
      },
      tag: {
        textColor: palette.c02,
        backgroundColor: "#ccc",
      },
      separator: {
        backgroundColor: "#e5e5e5",
      },
      comment: {
        borderColor: "#e5e5e5",
        backgroundColor: palette.c02,
        footerBackgroundColor: "#f5f5f5",
        textColor: palette.c01,
      },
    },
    signin: {
      signUpLink: {
        textColor: palette.c03,
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
    textColor: palette.c01,
    backgroundColor: palette.c02,
    footer: {
      textColor: palette.c02,
      backgroundColor: "linear-gradient(#485563, #29323c)",
    },
  },

  shared: {
    input: {
      textColor: palette.c00,
      borderColor: "#00000026",
      borderColorFocused: "#66afe9",
      backgroundColor: palette.c02,
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

      primary: {
        textColor: palette.c03,
        borderColor: palette.c03,
        backgroundColor: "transparent",
        active: {
          textColor: palette.c02,
          backgroundColor: palette.c03,
        },
      },
      secondary: {
        textColor: palette.c06,
        borderColor: palette.c06,
        backgroundColor: "transparent",
        active: {
          textColor: palette.c02,
          backgroundColor: palette.c06,
        },
      },
      danger: {
        textColor: palette.c05,
        borderColor: palette.c05,
        backgroundColor: "transparent",
        active: {
          textColor: palette.c02,
          backgroundColor: palette.c05,
        },
      },
    },
  },
};

export type AppThemeType = typeof LightTheme;
