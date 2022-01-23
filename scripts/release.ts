import { execSync } from "child_process";
import { readJSONSync } from "fs-extra";

const { version: oldVersion } = readJSONSync("package.json");

execSync("npx bumpp", { stdio: "inherit" });

const { version } = readJSONSync("package.json");

if (oldVersion === version) {
  console.log("canceled");
  process.exit();
}

execSync("git add .", { stdio: "inherit" });

execSync(`git commit -m "${version}"`, { stdio: "inherit" });
execSync(`git tag -a v${version} -m "${version}"`, { stdio: "inherit" });
