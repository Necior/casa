# casa

A simple (no DBMS, no authentication, no JavaScript) PWA (Progressive Web App) to track personal expenses.

## Properties

* Made in Python 3.8 using FastAPI;
* CSS supports both light and dark themes, depending on agent preferences, thanks to water.css;
* Stores data in a single file (this constraint might be relaxed in the future);
* Doesn't require JavaScript runtime; in fact there is no single JS line in the codebase;
* Supports only one (language, currency) combination, namely (Polish, PLN).

## Installation and usage

```bash
python3.8 -m venv venv # create a virtual environment
source venv/bin/activate # activate the virtual environment
pip install -r requirements.txt # install dependencies

# Please read https://fastapi.tiangolo.com/deployment/manually/ before deploying into production
uvicorn main:app --host 127.0.0.1 --port 8080
```

