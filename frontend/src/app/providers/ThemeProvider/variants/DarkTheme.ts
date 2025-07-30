// export const globalDefaultThemeMUI = {
//   fonts: {
//     fontFamily: "NonDotteeRegular",
//     fontFace: `
//         @font-face {
//           font-family: 'NonDotteeRegular';
//           src: url(https://gl-static.int07.glowfinsvs.com/b1/fonts/NonDotteeRegular.ttf) format('truetype');
//           font-weight: 400;
//           font-style: normal;
//         },
//         @font-face {
//           font-family: 'NonDotteeBold';
//           src: url(https://gl-static.int07.glowfinsvs.com/b1/fonts/NonDotteeBold.ttf) format('truetype');
//           font-weight: 700;
//           font-style: normal;
//         }
//       `,
//   },
// }
import { AppThemeType } from "../theme.types";

const commonKeys = {
  color: {
    text: {
      c00: "#55595c",
      c01: "#111111",
      c02: "#ffffff",
      c03: "#5CB85B",
      c04: "#449d44",
    },
  },
};

export const DarkTheme: AppThemeType = {
  page: {
    home: {
      c01: commonKeys.color.text.c01,
      c02: commonKeys.color.text.c02,
      c03: commonKeys.color.text.c03,
    },
    signup: {
      colorAccent: commonKeys.color.text.c03,
    },
  },

  mainLayout: {
    c01: commonKeys.color.text.c01,
    c02: commonKeys.color.text.c02,
    header: {
      c01: commonKeys.color.text.c03,
      c02: "#ffffff",
    },
    footer: {
      c01: "#ffffff",
      c02: "linear-gradient(#485563, #29323c)",
    },
  },

  shared: {
    input: {
      textColor: commonKeys.color.text.c00,
      borderColor: "rgba(0, 0, 0, 0.15)",
      backgroundColor: "#ffffff",
      backgroundColorFocused: "#66afe9",
      errorColor: "#ff0000",
    },
    button: {
      c01: commonKeys.color.text.c01,
      c02: commonKeys.color.text.c02,
      c03: commonKeys.color.text.c03,
      c04: commonKeys.color.text.c04,
    },
  },
};
