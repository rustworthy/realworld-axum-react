import dotEnv from "dotenv";
import path from "path";
import webpack from "webpack";

import { BuildMode, BuildPaths, BuildPlatform, buildWebpack } from "./configs";

interface EnvVariables {
  mode?: BuildMode;
  analyzer?: boolean;
  port?: number;
  platform?: BuildPlatform;
}

export default (env: EnvVariables) => {
  dotEnv.config({ path: "./.env" });

  const paths: BuildPaths = {
    output: path.resolve(__dirname, "build"),
    entry: path.resolve(__dirname, "src/app", "index.tsx"),
    html: path.resolve(__dirname, "public", "index.html"),
    public: path.resolve(__dirname, "public"),
    src: path.resolve(__dirname, "src"),
  };

  const config: webpack.Configuration = buildWebpack({
    port: env.port ?? 8005,
    mode: env.mode ?? "development",
    paths,
    analyzer: env.analyzer,
    platform: env.platform ?? "desktop",
  });

  return config;
};
