# Partial chrome web extensions Rust bindings
Using the idl and json interface files at [chromium git repo chrome specific](https://github.com/chromium/chromium/tree/main/chrome/common/extensions/api) and [chromium git repo not chrome specific](https://github.com/chromium/chromium/tree/main/extensions/common/api) to write Rust bindings to the [chrome web extensions](https://developer.chrome.com/docs/extensions).

## Other crates that provide bindings to web extensions
[web-extensions](https://crates.io/crates/web-extensions)


This repo contains chrome-idl-parser and chrome-extensions crates.

```
❯ ./target/release/chrome-idl-parser --help
Usage: chrome-idl-parser --to <TO> [FROM]...

Arguments:
  [FROM]...  The path to the input directory with chrome json and idl files

Options:
  -t, --to <TO>  The file path to the src directory
  -h, --help     Print help
  -V, --version  Print version
```

## Bindings
As of February 2024 only the types and the types' properties are being generated.
