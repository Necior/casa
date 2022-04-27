import datetime
import random
import shelve
from decimal import Decimal
from itertools import groupby

from fastapi import FastAPI, Form, Request
from fastapi.responses import FileResponse, HTMLResponse, RedirectResponse
from fastapi.templating import Jinja2Templates
from pydantic import BaseModel

app = FastAPI(openapi_url=None)
templates = Jinja2Templates(directory="./")


def random_quote():
    return random.choice(
        [
            "Bardziej od pieniędzy, potrzebujesz miłości. Miłość to siła nabywcza szczęścia.",  # noqa
            "Chciałoby się być bogatym, aby już nie myśleć o pieniądzach, ale większość bogatych i tak nie myśli o niczym innym.",  # noqa
            "Człowiek najpierw pragnie być pięknym, potem bogatym a na końcu tylko zdrowym.",  # noqa
            "Człowiek z klasą nie rozdrabnia się nad sprawami pieniędzy.",  # noqa
            "Gdy nie wiadomo o co chodzi, wiadomo, że chodzi o pieniądze. Podobnie jest z konkordatem, który dla finansów państwa okazał się istną czarną dziurą. Pochłania coraz więcej pieniędzy z państwowej kasy, a duchowni wynajdują różne sposoby, by zapewnić finansowanie z niej Kościoła.",  # noqa
            "Gdy pieniądze mówią, prawda milczy.",  # noqa
            "Grosz do grosza, a będzie kokosza.",  # noqa
            "I znowu człowiek wydaje pieniądze, których nie ma, na rzeczy, których nie potrzebuje, by imponować ludziom, których nie lubi.",  # noqa
            "Inteligencję człowieka można zobaczyć w tym, jak zarabia pieniądze. Jego mądrość w tym, jak je wydaje.",  # noqa
            "Jeśli możesz policzyć, ile masz pieniędzy, to nie jesteś specjalnie bogaty.",  # noqa
            "Kobietom pieniądze potrzebne nie są. Bo i po co? Nie piją, w kości nie grają, a kobietami, psiakrew, są przecież same.",  # noqa
            "Pieniądze są materialną formą zasady mówiącej, że ludzie, którzy chcą załatwiać ze sobą interesy, muszą to robić w formie handlu, płacąc wartością za wartość.",  # noqa
            "Pieniądze! Ze wszystkich wynalazków ludzkości – ten wynalazek jest najbliższy szatanowi.",  # noqa
            "W sferze materialnej dawać znaczy być bogatym. Nie jest bogatym ten, kto dużo ma, lecz ten, kto dużo daje.",  # noqa
            "Z pieniędzmi nie jest tak dobrze, jak jest źle bez nich.",  # noqa
            "Ziarnko do ziarnka zbierając, do niczego nie dojdziesz, chyba żebyś żył kilkaset lat.",  # noqa
            "Żyje się za pieniądze, ale nie warto żyć dla pieniędzy.",  # noqa
        ]
    )


def get_month_name(month):
    months_pl = [
        "styczeń",
        "luty",
        "marzec",
        "kwiecień",
        "maj",
        "czerwiec",
        "lipiec",
        "sierpień",
        "wrzesień",
        "październik",
        "listopad",
        "grudzień",
    ]
    return months_pl[month - 1]


class Expense(BaseModel):
    name: str
    value: Decimal  # if income, provide a negative value
    date: datetime.date


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

    def progress(self, target=1e6):
        with shelve.open(self.db_path) as db:
            return -1 * int(sum(x.value for x in db["history"])) * 100 / target


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
    expenses = []
    for year_month, exp in groupby(
        repo.list(), key=lambda e: (e.date.year, e.date.month)
    ):
        exp = list(exp)  # since we want to iterate multiple times
        expenditure = 0
        income = 0
        for e in exp:
            if e.value > 0:
                expenditure += e.value
            else:
                income -= e.value
        expenses.append(
            {
                "year": year_month[0],
                "month": get_month_name(year_month[1]),
                "total": income - expenditure,
                "expenditure": expenditure,
                "income": income,
                "items": exp,
            }
        )
    return templates.TemplateResponse(
        "./index.html",
        {
            "request": request,
            "today": datetime.datetime.now().strftime("%Y-%m-%d"),
            "random_quote": random_quote(),
            "expenses": expenses,
            "progress": repo.progress(),
        },
    )


@app.post("/add", response_class=HTMLResponse)
async def add(
    name: str = Form(...),
    value: Decimal = Form(...),
    date: datetime.date = Form(...),
):
    expense = Expense(name=name, value=value, date=date)
    repo.add(expense)
    return RedirectResponse(url="/", status_code=303)
