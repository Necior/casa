# casa

A simple (SQLite-based, no authentication, no JavaScript) PWA (Progressive Web App) to track personal expenses.

## Properties

* Made in Python 3.8 using FastAPI with a pinch of Rust;
* CSS supports both light and dark themes, depending on agent preferences, thanks to water.css;
* Stores data in a single file (this constraint might be relaxed in the future);
* Doesn't require JavaScript runtime; in fact there is no single JS line in the codebase;
* Supports only two (language, currency) combinations, namely (Polish, PLN) and (Polish, EUR).

## Installation and usage

```bash
poetry install
poetry run maturin develop

# Please read https://fastapi.tiangolo.com/deployment/manually/ before deploying into production
poetry run uvicorn main:app --host 127.0.0.1 --port 8080
```

