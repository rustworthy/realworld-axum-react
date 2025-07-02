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

const commonKeys = {
    color: {
        text: {
            c01: "#111111",
            c02: "#ffffff",
        },
    },
};

export const LightTheme = {
    page: {
        home: {
            c01: commonKeys.color.text.c01,
        },
    },

    shared: {
        bg: {
            c01: commonKeys.color.text.c02,
        }, 
        button: {
            c01: commonKeys.color.text.c01,
            c02: commonKeys.color.text.c02,
            c03: "#848484"
        },
    },
}


export type AppThemeType = typeof LightTheme;
