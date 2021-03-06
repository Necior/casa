import datetime
import random
import sqlite3
from decimal import Decimal
from itertools import groupby
from typing import Dict

from fastapi import FastAPI, Form, Request
from fastapi.responses import FileResponse, HTMLResponse, RedirectResponse
from fastapi.templating import Jinja2Templates
from pydantic import BaseModel

from casa import QUOTES, get_month_name

app = FastAPI(openapi_url=None)
templates = Jinja2Templates(directory="./")


def format_expense(value: Decimal, currency: str) -> str:
    income = value < 0
    return {
        (True, "PLN"): f"+{-value} zł",
        (False, "PLN"): f"{value} zł",
        (True, "EUR"): f"+ €{-value}",
        (False, "EUR"): f"€{value}",
    }[(income, currency)]


class Expense(BaseModel):
    name: str
    value: Decimal  # if income, provide a negative value
    date: datetime.date
    currency: str = "PLN"


class SQLiteRepository:
    def __init__(self):
        self.connection = sqlite3.connect("db.sqlite")
        cur = self.connection.cursor()
        cur.execute(
            "create table if not exists expenses (name text, value text, date text, currency text)"
        )
        cur.execute(
            "create table if not exists key_value_store (key text, value text)"
        )
        self.connection.commit()

    def add(self, expense: Expense):
        cur = self.connection.cursor()
        cur.execute(
            "insert into expenses (name, value, date, currency) values (?, ?, ?, ?)",
            (
                expense.name,
                str(expense.value),
                str(expense.date),
                expense.currency,
            ),
        )
        self.connection.commit()

    def list(self):
        cur = self.connection.cursor()
        rows = cur.execute(
            "select name, value, date, currency from expenses order by date desc"
        )
        return [
            Expense(name=r[0], value=r[1], date=r[2], currency=r[3])
            for r in rows
        ]

    def balance(self) -> Dict[str, Decimal]:
        def to_decimal(s: float) -> Decimal:
            d = Decimal(int(s * 100))
            return d / 100

        cur = self.connection.cursor()
        rows = cur.execute(
            "select currency, -sum(value) from expenses group by currency"
        )
        return {row[0]: to_decimal(row[1]) for row in rows}

    def get_notepad(self, notepad_key: str = "notepad") -> str:
        """
        Return notepad as a trusted HTML.

        If not available, return an empty string.
        """
        cur = self.connection.cursor()
        rows = cur.execute(
            "select value from key_value_store where key = ?", (notepad_key,)
        )
        try:
            return next(iter(rows))[0]
        except StopIteration:
            return ""


repo = SQLiteRepository()


@app.get("/manifest.json")
def manifest():
    return {
        "name": "Casa",
        "short_name": "Casa",
        "display": "standalone",
        "start_url": "/",
        "theme_color": "#313131",
        "background_color": "#313131",
        "icons": [
            {
                "src": "/icon.png",
                "sizes": "192x192",
                "type": "image/png",
            }
        ],
    }


@app.get("/icon.png")
def icon():
    return FileResponse("./icon.png")


@app.get("/", response_class=HTMLResponse)
async def root(request: Request):
    expenses = groupby(repo.list(), key=lambda e: (e.date.year, e.date.month))
    final_exp = []
    for year_month, exp in expenses:
        final_exp.append(
            {
                "year": year_month[0],
                "month": get_month_name(year_month[1]),
                "items": list(exp),
            }
        )
    return templates.TemplateResponse(
        "./index.html",
        {
            "request": request,
            "today": datetime.datetime.now().strftime("%Y-%m-%d"),
            "random_quote": random.choice(QUOTES),
            "expenses": final_exp,
            "balance": repo.balance(),
            "expense_formatter": format_expense,
            "notepad": repo.get_notepad(),
        },
    )


@app.post("/add", response_class=HTMLResponse)
async def add(
    name: str = Form(...),
    value: Decimal = Form(...),
    date: datetime.date = Form(...),
    currency: str = Form(...),
):
    expense = Expense(name=name, value=value, date=date, currency=currency)
    repo.add(expense)
    return RedirectResponse(url="/", status_code=303)
