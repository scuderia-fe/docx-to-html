<h3 align="center">
    <a href="https://scuderia-fe.github.io/docx-to-html">
      <img src="./.github/assets/logo.webp">
    </a>
    @scuderia-fe/docx-to-html
</h3>

> WARNING: WIP package. Do not use

<hr />

<p align="center">
  <i align="center">Blazing fast docx to html converter 🚀</i>
</p>

<h4 align="center">
  <a href="https://github.com/scuderia-fe/docx-to-html/graphs/contributors">
   <img alt="GitHub contributors" src="https://img.shields.io/github/contributors-anon/scuderia-fe/docx-to-html">
  </a>
  <a href="https://opensource.org/licenses/MIT">
    <img alt="GitHub License" src="https://img.shields.io/github/license/scuderia-fe/docx-to-html">
  </a>
  <a href="https://www.npmjs.com/package/@scuderia-fe/docx-to-html">
    <img alt="NPM Version" src="https://img.shields.io/npm/v/%40scuderia-fe%2Fdocx-to-html">
  </a>
  <br>
</h4>

## Node

### Installation

You can install the package with your favorite package manager

```bash
npm install @scuderia-fe/docx-to-html
```

Your project needs to support `asyncWebAssembly`:

- [Vite](https://github.com/Menci/vite-plugin-wasm)

```bash
npm install --save-dev vite-plugin-wasm vite-plugin-top-level-await
```

```ts
import wasm from "vite-plugin-wasm";
import topLevelAwait from "vite-plugin-top-level-await";

export default defineConfig({
  plugins: [wasm(), topLevelAwait()],
});
```

- Webpack

```js
module.exports = {
  experiments: {
    asyncWebAssembly: true,
    topLevelAwait: true,
  },
};
```

### Usage

```ts
import { convert } from "@scuderia-fe/docx-to-html";

const buffer = await file
  .arrayBuffer()
  .then((buffer) => new Uint8Array(buffer));
const html = wasm.convert(buffer);
```

### Rust

> WIP
