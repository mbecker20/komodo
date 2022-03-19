import { defineConfig } from "vite";

export default defineConfig({
  build: {
    outDir: "build",
    target: "esnext",
    ssr: true,
    "ssr.target": "webworker",
    rollupOptions: {
      input: "./src/cli.tsx",
    },
  },
});
