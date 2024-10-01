import pluginJs from "@eslint/js";
import tseslint from "typescript-eslint";

export default [
  {
    ignores: ["src/pkg/", "dist/", "node_modules/", "build.js"],
  },
  {
    files: ["src/**/*.{js,mjs,cjs,ts}"],
  },
  pluginJs.configs.recommended,
  ...tseslint.configs.recommended,
];
