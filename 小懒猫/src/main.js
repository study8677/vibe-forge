import { createApp } from "./app.js";
import { createAppStore } from "./store.js";

const root = document.querySelector("#app");
const store = createAppStore();

createApp(root, store);
