import { defineConfig } from "vite";

export default defineConfig({
  build: {
    outDir: "build",
    target: "esnext",
    ssr: true,
    rollupOptions: {
      input: "./src/cli.tsx",
    },
  },
});
