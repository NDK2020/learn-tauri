import { defineConfig, loadEnv } from "vite";
import { fileURLToPath } from "url";
import vue from "@vitejs/plugin-vue";
import { setup_vite_plugins } from "./plugins";

// https://vitejs.dev/config/
export default defineConfig(async (env) => {
  const viteEnv = loadEnv(env.mode, process.cwd()) as ImportMetaEnv;
  const rootPath = fileURLToPath(new URL("./", import.meta.url));
  const srcPath = `${rootPath}src`;

  return {
    base: viteEnv.VITE_BASE_URL,
    resolve: {
      alias: {
        "~": rootPath,
        "@": srcPath,
      },
    },
    plugins: setup_vite_plugins(),
    css: {
      preprocessorOptions: {
        scss: {
          additionalData: '@import "./src/styles/index.scss";',
        },
      },
    },
    // Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
    // prevent vite from obscuring rust errors
    clearScreen: false,
    // tauri expects a fixed port, fail if that port is not available
    server: {
      port: 1420,
      strictPort: true,
    },
    // to make use of `TAURI_DEBUG` and other env variables
    // https://tauri.studio/v1/api/config#buildconfig.beforedevcommand
    envPrefix: ["VITE_", "TAURI_"],
    build: {
      // Tauri supports es2021
      target:
        process.env.TAURI_PLATFORM == "windows" ? "chrome105" : "safari13",
      // don't minify for debug builds
      minify: !process.env.TAURI_DEBUG ? "esbuild" : false,
      // produce sourcemaps for debug builds
      sourcemap: !!process.env.TAURI_DEBUG,
    },
  };
});
