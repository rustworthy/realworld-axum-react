import pluginJs from "@eslint/js";
import globals from "globals";
import tseslint from "typescript-eslint";
import eslintReact from "eslint-plugin-react";
import eslintReactHooks from "eslint-plugin-react-hooks";
import eslintReactRefresh from "eslint-plugin-react-refresh";
import prettierPlugin from "eslint-plugin-prettier";
import eslintConfigPrettier from "eslint-config-prettier";
import svgJSX from "eslint-plugin-svg-jsx";

/** @type {import('eslint').Linter.FlatConfig[]} */
export default tseslint.config(
  {
    settings: {
      react: {
       version: "detect",
      },
    },
  },
  {
    plugins: {
      "@typescript-eslint": tseslint.plugin,
      react: eslintReact,
      "react-hooks": eslintReactHooks,
      "react-refresh": eslintReactRefresh,
      prettier: prettierPlugin,
      "svg-jsx": svgJSX,
    },
  },
  {
    ignores: ["node_modules", "dist", "build", "storybook-static"],
  },
  pluginJs.configs.recommended,
  ...tseslint.configs.recommended,
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
  {
    files: ["**/*.{ts,tsx,js}"],
    rules: {
      ...prettierPlugin.configs.recommended.rules,
      ...eslintConfigPrettier.rules,
      "svg-jsx/camel-case-dash": "error",
      "svg-jsx/camel-case-colon": "error",
      "svg-jsx/no-style-string": "error",
      "react-refresh/only-export-components": ["warn", { allowConstantExport: true }],
      "prefer-const": "error",
      "react/jsx-curly-brace-presence": ["warn", { props: "never", children: "never" }],
      "react/function-component-definition": ["warn", { namedComponents: "arrow-function" }],
      "react/self-closing-comp": ["error", { component: true, html: true }],
      "max-lines": ["warn", { max: 500 }],
      "@typescript-eslint/no-unused-vars": [
      "error",
        {
          "args": "all",
          "argsIgnorePattern": "^_",
          "caughtErrors": "all",
          "caughtErrorsIgnorePattern": "^_",
          "destructuredArrayIgnorePattern": "^_",
          "varsIgnorePattern": "^_",
          "ignoreRestSiblings": true
        }
      ],
      "@typescript-eslint/no-explicit-any": "error",
      "no-console": "error",
      "no-restricted-imports": [
        "error",
        {
          "patterns": [
            {
              "group": ["src/*"],
              "message": "Avoid importing directly from 'src/'. Use relative paths instead."
            }
          ]
        }
      ],
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
          plugins: ['@trivago/prettier-plugin-sort-imports'],
          importOrder: ["^react", "^axios", "^formik", "^@reduxjs/(.*)$", "^redux-saga/(.*)$", "^@storybook/(.*)$", "^@emotion/(.*)$", "^@mui/(.*)$", "^@packages/(.*)$", "<THIRD_PARTY_MODULES>", "^[../]", "^[./]"],
          importOrderSeparation: true,
          importOrderSortSpecifiers: true
        },
      ],
    },
  },
  {
    files: ['.storybook/main.ts', "./configs/eslint/**/*", "eslint.config.mjs"],
    ...tseslint.configs.disableTypeChecked,
  },
  {
    settings: {
      react: {
        version: "detect",
      },
    },
  }
);
