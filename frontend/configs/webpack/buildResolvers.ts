import { Configuration } from "webpack";

import { BuildOptions } from "./types/types";

export function buildResolvers(options: BuildOptions): Configuration["resolve"] {
  return {
    extensions: [".tsx", ".ts", ".js", ".svg"],
    alias: {
      "@": options.paths.src,
    },
  };
}
