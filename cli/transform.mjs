import { readFileSync, writeFileSync } from "fs";

const contents = readFileSync("build/monitor-cli.js").toString();
writeFileSync("build/monitor-cli.js", "#!/usr/bin/env node\n" + contents);