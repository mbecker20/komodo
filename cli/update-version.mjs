import { readFileSync, writeFileSync } from "fs";

const rawpkg = readFileSync("package.json");
const pkgjson = JSON.parse(rawpkg);

const [major, minor, increment] = pkgjson.version
  .split(".")
  .map((item) => Number(item));

pkgjson.version = `${major}.${minor}.${increment + 1}`

writeFileSync(
  "package.json",
  JSON.stringify(pkgjson, undefined, 2)
);
