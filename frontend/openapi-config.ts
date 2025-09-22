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
    // we do not want the codegen to destructure and export hooks,
    // e.g. `useReadArtcileQuery`, `useCreateaArticleMutation`, etc.,
    // for us. Istead, we want to first enhance the generated `api`
    // and after that export the hooks;
    // see: https://redux-toolkit.js.org/rtk-query/api/created-api/code-splitting
    hooks: false,
  } satisfies ConfigFile;
})();
