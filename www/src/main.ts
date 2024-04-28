import "./style.css";
import viteLogo from "/vite.svg";
import typescriptLogo from "/typescript.svg";
import { greet, double_count } from "wasm-rust-template";

greet("Rustacean");

document.querySelector<HTMLDivElement>('#app')!.innerHTML = `
  <div class="logos">
    <a href="https://rustwasm.github.io/wasm-pack" target="_blank">
      <img
        src="https://rustwasm.github.io/wasm-pack/public/img/wasm-ferris.png"
        id="ferris"
        alt="Rust mascot"
      />
    </a>
    <a href="https://vitejs.dev" target="_blank">
      <img src="${viteLogo}" id="vite" alt="Vite logo" />
    </a>
    <a href="https://www.typescriptlang.org/" target="_blank">
      <img src="${typescriptLogo}" id="ts" alt="TypeScript logo" />
    </a>
  </div>

  <h1>WASM + Vite + TypeScript</h1>
  <div class="card">
    <button id="counter" type="button">Click me!</button>
  </div>
  <p class="read-the-docs">
    Click on the logos to learn more
  </p>
`

const button = document.querySelector("#counter")!;
button.addEventListener("click", () => double_count(button));
