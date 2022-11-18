//! THIS FILE IS AUTOGENERATED, DO NOT MODIFY
//! When building, `cargo-leptos` generates this file based on
//! the `index.html` file specified in the Config.toml
//!
//! This file can be commited to version control. It only
//! changes when the configuration changes

#[cfg(feature = "leptos_autoreload")]
/// index.html content up to `<!-- INJECT HEAD -->` plus `cargo leptos` injected css and js content.
pub const HTML_START: &str = r##"<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="utf-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1" />
    <script type="module">import init from '/pkg/app.js';init('/pkg/app.wasm');</script>
    <link rel="preload" href="/pkg/app.wasm" as="fetch" type="application/wasm" crossorigin="">
    <link rel="stylesheet" href="/pkg/app.css">
    <link rel="modulepreload" href="/pkg/app.js">
    <script crossorigin="">(function () {
        var ws = new WebSocket('ws://127.0.0.1:3001/ws');
        ws.onmessage = (ev) => {
            console.log(`Reload message: `);
            if (ev.data === 'reload') window.location.reload();
        };
        ws.onclose = () => console.warn('Autoreload stopped. Manual reload necessary.');
    })()
    </script>"##;

#[cfg(not(feature = "leptos_autoreload"))]
/// index.html content up to `<!-- INJECT HEAD -->` plus `cargo leptos` injected css and js content.
pub const HTML_START: &str = r##"<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="utf-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1" />
    <script type="module">import init from '/pkg/app.js';init('/pkg/app.wasm');</script>
    <link rel="preload" href="/pkg/app.wasm" as="fetch" type="application/wasm" crossorigin="">
    <link rel="stylesheet" href="/pkg/app.css">
    <link rel="modulepreload" href="/pkg/app.js">"##;

/// index.html content from `<!-- INJECT HEAD -->` up to `<!-- INJECT BODY -->`
pub const HTML_MIDDLE: &str = r##"  </head>
  <body>"##;

/// index.html content from `<!-- INJECT BODY -->` until the end
pub const HTML_END: &str = r##"  </body>
</html>"##;
