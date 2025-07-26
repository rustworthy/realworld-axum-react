import { createGlobalStyle } from "styled-components";

export const GlobalStyles = createGlobalStyle`
   * {
          box-sizing: border-box;
          ::-webkit-scrollbar {
            width: 14px;
            height: 14px;
          }

          ::-webkit-scrollbar-track {
            background: transparent;
          }

          ::-webkit-scrollbar-thumb {
            background: #d8d8d8;
            border-radius: 6px;
            background-clip: content-box;
            border-left: solid 4px transparent;
            border-right: solid 4px transparent;
          }
        }
        h1,
        h2,
        h3,
        h4,
        h5,
        h6,
        p,
        ul,
        li,
        body {
          margin: 0;
          padding: 0;
        }

        ul, li, a { 
          color: inherit;
          text-decoration: none;
        }

        ul {
          list-style: none;
        }
`;
