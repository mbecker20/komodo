import { readFileSync, writeFileSync } from "fs";

const contents = readFileSync("build/cli.js").toString();
writeFileSync("build/cli.js", "#!/usr/bin/env node\n" + contents);

const pkgjson = JSON.parse(readFileSync("package.json"));

delete pkgjson.devDependencies;
delete pkgjson.scripts;
// delete pkgjson.dependencies["@monitor/util"];
// pkgjson.bin = "cli.js";

writeFileSync("build/package.json", JSON.stringify(pkgjson, undefined, 2));
