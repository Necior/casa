import datetime
import random
import shelve
from decimal import Decimal
from itertools import groupby

from fastapi import FastAPI, Form, Request
from fastapi.responses import FileResponse, HTMLResponse, RedirectResponse
from fastapi.templating import Jinja2Templates
from pydantic import BaseModel

from casa import QUOTES, get_month_name

app = FastAPI(openapi_url=None)
templates = Jinja2Templates(directory="./")


class Expense(BaseModel):
    name: str
    value: Decimal  # if income, provide a negative value
    date: datetime.date
    currency: str = "PLN"

    def format(self):
        try:
            c = self.currency
        except AttributeError:
            c = "PLN"
        income = self.value < 0
        return {
            (True, "PLN"): f"+{-self.value} zł",
            (False, "PLN"): f"{self.value} zł",
            (True, "EUR"): f"+ €{-self.value}",
            (False, "EUR"): f"€{self.value}",
        }[(income, c)]


class FileRepository:
    def __init__(self):
        self.db_path = "./db"
        with shelve.open(self.db_path) as db:
            if "history" not in db:
                db["history"] = []

    def add(self, expense: Expense):
        with shelve.open(self.db_path) as db:
            db["history"] = db["history"] + [expense]

    def list(self):
        with shelve.open(self.db_path) as db:
            return list(
                sorted(db["history"], key=lambda e: e.date, reverse=True)
            )


repo = FileRepository()


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
