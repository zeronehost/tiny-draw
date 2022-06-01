import "./style.css";

// const app = document.querySelector<HTMLDivElement>("#app")!;

// app.innerHTML = `
//   <h1>Hello Vite!</h1>
//   <a href="https://vitejs.dev/guide/features.html" target="_blank">Documentation</a>
// `;

import init, { TinyDraw } from "tiny-draw"
;(async () => {
  await init();
  const tiny = new TinyDraw("app", 500, 400);
  console.log(tiny);
})();
