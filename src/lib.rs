#![deny(clippy::all)]
use napi::*;
use web_view::*;

#[macro_use]
extern crate napi_derive;

#[napi(object)]
struct WebviewOptions {
  pub title: Option<String>,
  pub debug: Option<bool>,
  pub content: Option<String>,
  pub width: i32,
  pub height: i32,
}

#[napi]
fn create_window(options: WebviewOptions) -> Result<()> {
  let title = options.title.unwrap_or_default();
  let debug = options.debug.unwrap_or_default();
  let content = options.content.unwrap_or_default();
  let width = options.width;
  let height = options.height;

  web_view::builder()
    .title(&title)
    .content(Content::Html(content))
    .size(width, height)
    .user_data(())
    .debug(debug)
    .invoke_handler(|_webview, _arg| Ok(()))
    .run()
    .unwrap();

  Ok(())
}
