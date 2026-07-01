/*
 * Copyright (C) 2026 meta-magic_mount-rs developers
 * SPDX-License-Identifier: GPL-v3
 */

import { render } from "solid-js/web";

import App from "./App.tsx";

import "./init";
import "./app.css";
import "./layout.css";

const root = document.querySelector("#app");

if (root instanceof HTMLElement) {
  render(() => <App />, root);
}
