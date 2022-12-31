import { readdirSync, renameSync, rmdirSync } from "fs";

const files = readdirSync("./build/assets");

for (const file of files) {
  renameSync("./build/assets/" + file, "./build/" + file);
}

rmdirSync("./build/assets");

console.log("\npost build complete\n")