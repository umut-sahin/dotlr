import {execSync} from 'child_process';
import fs from "fs/promises"


async function init() {
    console.log("Starting build...")
    execSync('tsc', {stdio: 'inherit'});
    await fs.cp("./src/pkg", "./dist/pkg", {recursive: true});
    await fs.unlink("./dist/pkg/package.json");
    await fs.unlink("./dist/pkg/README.md");
    await fs.unlink("./dist/pkg/.gitignore");
    console.log("Build complete")

}

init()
