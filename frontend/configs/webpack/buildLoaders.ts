import ReactRefreshTypeScript from "react-refresh-typescript";

import MiniCssExtractPlugin from "mini-css-extract-plugin";
import { ModuleOptions } from "webpack";

import { BuildOptions } from "./types/types";

export function buildLoaders(options: BuildOptions): ModuleOptions["rules"] {
  const isDev = options.mode === "development";

  const assetLoader = {
    test: /\.(png|jpg|jpeg|gif)$/i,
    type: "asset/resource",
  };

  const svgrLoader = {
    test: /\.svg$/i,
    use: [
      {
        loader: "@svgr/webpack",
        options: {
          icon: true,
          svgoConfig: {
            plugins: [
              {
                name: "convertColors",
                params: {
                  currentColor: true,
                },
              },
            ],
          },
        },
      },
    ],
  };

  const cssLoaderWithModules = {
    loader: "css-loader",
    options: {
      modules: {
        localIdentName: isDev ? "[path][name]__[local]" : "[hash:base64:8]",
      },
    },
  };

  const scssLoader = {
    test: /\.(css|scss|sass)$/i,
    exclude: /mdeditor\.css$/i,
    use: [
      // Creates `style` nodes from JS strings
      isDev ? "style-loader" : MiniCssExtractPlugin.loader, // Translates CSS into CommonJS
      cssLoaderWithModules, // Compiles Sass to CSS
      "sass-loader",
    ],
  };

  // create separated css files
  const staticCssFilesLoader = {
    test: /mdeditor\.css$/i,
    use: [isDev ? "style-loader" : MiniCssExtractPlugin.loader, "css-loader"],
  };

  const tsLoader = {
    exclude: /node_modules/,
    test: /\.tsx?$/,
    use: [
      {
        loader: "ts-loader",
        options: {
          transpileOnly: true,
          getCustomTransformers: () => ({
            before: [isDev && ReactRefreshTypeScript()].filter(Boolean),
          }),
        },
      },
    ],
  };

  return [assetLoader, scssLoader, staticCssFilesLoader, tsLoader, svgrLoader];
}
