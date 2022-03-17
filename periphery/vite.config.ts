import { defineConfig } from "vite";

export default defineConfig({
  build: {
    outDir: "build",
    target: "esnext",
    ssr: "./src/main.ts",
  },
});
