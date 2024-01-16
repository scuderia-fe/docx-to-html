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
  <a href="https://opensource.org/licenses/Apache-2.0">
    <img alt="GitHub License" src="https://img.shields.io/github/license/scuderia-fe/docx-to-html">
  </a>
  <img alt="NPM Version" src="https://img.shields.io/npm/v/%40scuderia-fe%2Fdocx-to-html">
  <br>
</h4>

## Node

### Installation

```bash
npm install @scuderia-fe/docx-to-html
yarn add @scuderia-fe/docx-to-html
pnpm add @scuderia-fe/docx-to-html
bun add @scuderia-fe/docx-to-html
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
