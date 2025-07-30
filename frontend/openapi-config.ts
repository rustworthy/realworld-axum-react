import type { ConfigFile } from "@rtk-query/codegen-openapi";
import { config } from "dotenv";

export default (() => {
  config();
  const schemaFile = process.env.OPENAPI_SCHEMA_URL;
  if (!schemaFile) {
    throw new Error(`Make sure .env file exists and contains "OPENAPI_SCHEMA_URL"`);
  }
  return {
    schemaFile,
    apiFile: "./src/shared/api/base.ts",
    apiImport: "base",
    outputFile: "./src/shared/api/generated.ts",
    exportName: "api",
    hooks: true,
  } satisfies ConfigFile;
})();
