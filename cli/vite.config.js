import { defineConfig } from "vite";

export default defineConfig({
  build: {
    outDir: "build",
    target: "node12",
    ssr: true,
    rollupOptions: {
      input: "./src/cli.tsx",
    },
  },
});
