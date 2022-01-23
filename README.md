# webview-rs

⚡️ Currently is still in development.

Rust webview for Node.js.

## Install

```console
npm i webview-rs
```

## Usage

```ts
const { createWindow } = require("webview-rs");

createWindow({
  title: "Hello Webview",
  width: 600,
  height: 400,
  content: `<h1>Hello Webview</h1>`,
});
```

## License

MIT © [2nthony](https://github.com/2nthony)
