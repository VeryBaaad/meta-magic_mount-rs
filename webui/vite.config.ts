/*
 * Copyright (C) 2026 meta-magic_mount-rs developers
 * SPDX-License-Identifier: GPL-v3
 */

import { defineConfig } from "vite";
import vue from "@vitejs/plugin-vue";

// https://vite.dev/config/
export default defineConfig({
  base: "./",
  build: {
    outDir: "../module/webroot",
    target: "esnext",
  },
  plugins: [vue()],
});
