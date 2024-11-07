import { execSync } from "child_process";
import fs from "fs/promises";

async function init() {
  console.log("Starting build...");
  await fs
    .unlink("./src/pkg/dotlr_bg.wasm.d.ts")
    .catch(() => console.warn("No dotlr_bg.wasm.d.ts found"));
  execSync("tsc", { stdio: "inherit" });
  await fs.cp("./src/pkg", "./dist/pkg", { recursive: true });
  await fs
    .unlink("./dist/pkg/package.json")
    .catch(() => console.warn("No package.json found"));
  await fs
    .unlink("./dist/pkg/README.md")
    .catch(() => console.warn("No README.md found"));
  await fs
    .unlink("./dist/pkg/.gitignore")
    .catch(() => console.warn("No .gitignore found"));
  console.log("Build complete");
}

init();
