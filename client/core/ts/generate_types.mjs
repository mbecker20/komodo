import { exec } from "child_process";
import { readFileSync, writeFileSync } from "fs";
import path from "path";
import { fileURLToPath } from "url";

const __dirname = path.dirname(fileURLToPath(import.meta.url));

console.log("generating typescript types...");

const gen_command =
  "RUST_BACKTRACE=1 typeshare . --lang=typescript --output-file=./client/ts/src/types.ts";

exec(gen_command, (error, stdout, stderr) => {
	if (error) {
		console.error(error);
		return;
	}
	console.log("generated types using typeshare");
	fix_types();
	console.log("finished.");
});

function fix_types() {
	const types_path = __dirname + "/src/types.ts";
	const contents = readFileSync(types_path);
	const fixed = contents
    .toString()
    .replaceAll("ResourceTargetVariant", 'ResourceTarget["type"]')
	.replaceAll("AlertDataVariant", 'AlertData["type"]')
	.replaceAll("ProcedureConfigVariant", 'ProcedureConfig["type"]')
	.replaceAll("AlerterConfigVariant", 'AlerterConfig["type"]')
	writeFileSync(types_path, fixed);
}
