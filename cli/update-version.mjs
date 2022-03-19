import { readFileSync, writeFileSync } from "fs";

const pkgjson = JSON.parse(readFileSync("package.json"));

const [major, minor, increment] = pkgjson.version
  .split(".")
  .map((item) => Number(item));

pkgjson.version = `${major}.${minor}.${increment + 1}`

writeFileSync(
  "package.json",
  JSON.stringify(pkgjson, undefined, 2)
);

console.log("version updated to", pkgjson.version);