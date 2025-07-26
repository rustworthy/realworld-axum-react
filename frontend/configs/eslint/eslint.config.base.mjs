import pluginJs from "@eslint/js";
import globals from "globals";
import tseslint from "typescript-eslint";
import eslintReact from "eslint-plugin-react";
import eslintReactHooks from "eslint-plugin-react-hooks";
import eslintReactRefresh from "eslint-plugin-react-refresh";
import prettierPlugin from "eslint-plugin-prettier";
import eslintConfigPrettier from "eslint-config-prettier";
import svgJSX from "eslint-plugin-svg-jsx";
import eslintPluginImport from "eslint-plugin-import";

const importRestrictionsFSD = [
  // shared restrictions
  {
    target: "./src/shared",
    from: "./src/entities",
    message: "[FSD] shared cannot import from entities",
  },
  {
    target: "./src/shared",
    from: "./src/features",
    message: "[FSD] shared cannot import from features",
  },
  {
    target: "./src/shared",
    from: "./src/widgets",
    message: "[FSD] shared cannot import from widgets",
  },
  {
    target: "./src/shared",
    from: "./src/pages",
    message: "[FSD] shared cannot import from pages",
  },
  {
    target: "./src/shared",
    from: "./src/app",
    message: "[FSD] shared cannot import from app",
  },

  // entities restrictions
  {
    target: "./src/entities",
    from: "./src/features",
    message: "[FSD] shared cannot import from features",
  },
  {
    target: "./src/entities",
    from: "./src/widgets",
    message: "[FSD] shared cannot import from widgets",
  },
  {
    target: "./src/entities",
    from: "./src/pages",
    message: "[FSD] shared cannot import from pages",
  },
  {
    target: "./src/entities",
    from: "./src/app",
    message: "[FSD] shared cannot import from app",
  },

  // features restrictions
  {
    target: "./src/features",
    from: "./src/widgets",
    message: "[FSD] shared cannot import from widgets",
  },
  {
    target: "./src/features",
    from: "./src/pages",
    message: "[FSD] shared cannot import from pages",
  },
  {
    target: "./src/features",
    from: "./src/app",
    message: "[FSD] shared cannot import from app",
  },

  // widgets restrictions
  {
    target: "./src/widgets",
    from: "./src/pages",
    message: "[FSD] shared cannot import from pages",
  },
  {
    target: "./src/widgets",
    from: "./src/app",
    message: "[FSD] shared cannot import from app",
  },

  // pages restrictions
  {
    target: "./src/pages",
    from: "./src/app",
    message: "[FSD] shared cannot import from app",
  },
];


/** @type {import('eslint').FlatConfigArray} */
export default tseslint.config(
  // Global settings
  {
    settings: {
      react: {
        version: "detect",
      },
      "import/resolver": {
        typescript: {
          project: "./tsconfig.json",
        },
      }
    },
  },
  // Ignore patterns
  {
    ignores: ["node_modules", "dist", "build", "storybook-static"],
  },
  // Plugins definition
  {
    plugins: {
      "@typescript-eslint": tseslint.plugin,
      react: eslintReact,
      "react-hooks": eslintReactHooks,
      "react-refresh": eslintReactRefresh,
      prettier: prettierPlugin,
      "svg-jsx": svgJSX,
      import: eslintPluginImport,
    },
  },
  // Recommended configurations
  pluginJs.configs.recommended,
  ...tseslint.configs.recommended,
  // Language options and globals
  {
    languageOptions: {
      globals: {
        ...globals.browser,
        ...globals.node,
        ...globals.es2020,
      },
      parserOptions: {
        project: true,
      },
    },
  },
  // Rules for all source files
  {
    files: ["**/*.{ts,tsx,js,jsx}"],
    rules: {
      ...prettierPlugin.configs.recommended.rules,
      ...eslintConfigPrettier.rules,

      // SVG JSX rules
      "svg-jsx/camel-case-dash": "error",
      "svg-jsx/camel-case-colon": "error",
      "svg-jsx/no-style-string": "error",

      // React and general rules
      "react-refresh/only-export-components": ["warn", { allowConstantExport: true }],
      "prefer-const": "error",
      "react/jsx-curly-brace-presence": ["warn", { props: "never", children: "never" }],
      "react/function-component-definition": ["warn", { namedComponents: "arrow-function" }],
      "react/self-closing-comp": ["error", { component: true, html: true }],
      "max-lines": ["warn", { max: 500 }],

      // TypeScript rules
      "@typescript-eslint/no-unused-vars": [
        "error",
        {
          args: "all",
          argsIgnorePattern: "^_",
          caughtErrors: "all",
          caughtErrorsIgnorePattern: "^_",
          destructuredArrayIgnorePattern: "^_",
          varsIgnorePattern: "^_",
          ignoreRestSiblings: true,
        },
      ],
      "@typescript-eslint/no-explicit-any": "error",

      // General rules
      "no-console": "error",
      "no-restricted-imports": [
        "error",
        {
          patterns: [
            {
              group: ["src/*"],
              message: "Avoid importing directly from 'src/'. Use aliased or relative paths.",
            },
          ],
        },
      ],
      "import/no-restricted-paths": [
        "error",
        { 
          zones: importRestrictionsFSD 
        }
      ],

      // Prettier with import sorting
      "prettier/prettier": [
        "error",
        {
          singleQuote: false,
          printWidth: 130,
          tabWidth: 2,
          trailingComma: "all",
          bracketSpacing: true,
          jsxBracketSameLine: false,
          arrowParens: "always",
          endOfLine: "auto",
          plugins: ["@trivago/prettier-plugin-sort-imports"],
          importOrder: [
            "^react",
            "^axios",
            "^formik",
            "^@reduxjs/(.*)$",
            "^@storybook/(.*)$",
            "^@emotion/(.*)$",
            "^@mui/(.*)$",
            "<THIRD_PARTY_MODULES>",
            "^@shared/(.*)$",
            "^@entities/(.*)$",
            "^@features/(.*)$",
            "^@widgets/(.*)$",
            "^@pages/(.*)$",
            "^@app/(.*)$",
            "^[../]",
            "^[./]",
          ],
          importOrderSeparation: true,
          importOrderSortSpecifiers: true,
        },
      ],
    },
  },
  // Disable type checking for specific files
  {
    files: [".storybook/main.ts", "./configs/eslint/**/*", "eslint.config.mjs"],
    ...tseslint.configs.disableTypeChecked,
  }
);
