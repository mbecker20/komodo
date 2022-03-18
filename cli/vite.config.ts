import { defineConfig } from "vite";

export default defineConfig({
  build: {
    outDir: "build",
    target: "esnext",
    ssr: "./src/monitor-cli.tsx",
    rollupOptions: {
      input: "./src/monitor-cli.tsx",
    },
  },
});
