# casa

A simple (SQLite-based, no authentication, no JavaScript) PWA (Progressive Web App) to track personal expenses.

## Properties

* Made in Rust;
* CSS supports both light and dark themes, depending on agent preferences, thanks to water.css;
* Stores data in a single file (this constraint might be relaxed in the future);
* Doesn't require JavaScript runtime; in fact there is no single JS line in the codebase;
* Supports only two (language, currency) combinations, namely (Polish, PLN) and (Polish, EUR).

## Installation and usage

```bash
cargo run --release
```

