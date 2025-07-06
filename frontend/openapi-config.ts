import type { ConfigFile } from "@rtk-query/codegen-openapi";

export default {
  schemaFile: "https://petstore3.swagger.io/api/v3/openapi.json",
  apiFile: "./src/api/base.ts",
  apiImport: "base",
  outputFile: "./src/api/generated.ts",
  exportName: "api",
  hooks: true,
} satisfies ConfigFile;
