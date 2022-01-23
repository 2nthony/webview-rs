# webview-rs

![](https://badgen.net/npm/v/webview-rs?label=&color=cyan)

Rust webview for Node.js.

## Install

```console
npm i webview-rs
```

## Usage

```ts
const { createWindow } = require("webview-rs");

const html = `
<!DOCTYPE html>
<html>
  <head>
    <style>
      body {
        text-align: center;
      }
    </style>
  </head>

  <body>
    <h1>Webview RS for Node.js</h1>
    <button onclick="sayhi()">Say hi</button>

    <script>
      function sayhi() {
        alert("hi from webview rs");
      }
    </script>
  </body>
</html>
`;

createWindow({
  title: "Webview rs",
  width: 400,
  height: 600,
  content: html,
});
```

<p align="center">
  <img src="https://cdn.jsdelivr.net/gh/2nthony/statics@main/uPic/202dj0IlqXf4.png">
</p>

## Tests

- [x] macOS 12.1 with intel i5
- [ ] Any Windows

## Credits

- [Boscop/web-view](https://github.com/Boscop/web-view)
- [napi-rs/napi-rs](https://github.com/napi-rs/napi-rs)

## Contributes

PRs welcome!

## License

MIT © [2nthony](https://github.com/2nthony)
