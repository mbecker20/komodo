import { readFileSync, writeFileSync } from "fs";

const contents = readFileSync("build/monitor-cli.js").toString();
writeFileSync("build/monitor-cli.js", "#!/usr/bin/env node\n" + contents);

const rawpkg = readFileSync("package.json");
const pkgjson = JSON.parse(rawpkg);

delete pkgjson.devDependencies;
delete pkgjson.scripts;
// delete pkgjson.dependencies["@monitor/util"];
pkgjson.bin = "monitor-cli.js";

writeFileSync("build/package.json", JSON.stringify(pkgjson, undefined, 2));
