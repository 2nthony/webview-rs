import { execSync } from "child_process";

const [newversion] = process.argv.slice(2);
if (!newversion) {
  console.log("Invalid newversion");
  process.exit();
}

execSync(`npm version ${newversion}`, { stdio: "inherit" });
execSync("git push --follow-tags", { stdio: "inherit" });
