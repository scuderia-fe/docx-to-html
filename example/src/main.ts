import './main.css';
import * as wasm from 'docx-to-html';

const form = document.querySelector('form') as HTMLFormElement | null;
const fileInput = document.querySelector('input') as HTMLInputElement | null;
const submitButton = document.querySelector('button') as HTMLButtonElement | null;

if (fileInput) {
  fileInput.onchange = event => {
    event.preventDefault();
    const file = fileInput.files?.item(0);
    if (!file) return;

    if (submitButton) {
      submitButton.disabled = false;
    }
  }
}

if (form) {
  form.onsubmit = async event => {
    event.preventDefault();
    const fileInput = form.querySelector('input') as HTMLInputElement | null;
    if (!fileInput) return;

    fileInput.disabled = true;
    const file = fileInput.files?.item(0);
    if (!file) return;

    const start = window.performance.now();
    const buffer = await file.arrayBuffer().then(buffer => new Uint8Array(buffer));
    const out = wasm.convert(buffer);
    const end = window.performance.now();

    const performanceSpan = document.querySelector('#performance') as HTMLSpanElement | null;
    if (performanceSpan) {
      performanceSpan.innerHTML = `${end - start}ms`;
    }

    fileInput.disabled = false;

    const output = document.querySelector('#result') as HTMLIFrameElement | null;
    if (!output || !output.contentDocument) return;

    output.contentDocument.body.innerHTML = out;
    output.style.display = 'block';

    document.scrollingElement?.scrollTo({ top: output.offsetTop, behavior: 'smooth' })
  }
}