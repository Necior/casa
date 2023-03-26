use axum::response::Redirect;
use axum::routing::{get, post};
use axum::{Form, Router};
use chrono::NaiveDate;
use minijinja::render;
use rand::seq::SliceRandom;
use rusqlite::types::ToSqlOutput;
use rusqlite::{Connection, Result, ToSql};
use serde::{Deserialize, Serialize, Serializer};
use std::cmp::Reverse;
use std::collections::HashMap;
use std::convert::{TryFrom, TryInto};
use std::fmt::{Display, Formatter};
use std::hash::Hash;
use std::net::SocketAddr;
use std::sync::atomic::{AtomicU64, Ordering};

const QUOTES: [&str; 17] = [
    "Bardziej od pieniędzy, potrzebujesz miłości. Miłość to siła nabywcza szczęścia.",
    "Chciałoby się być bogatym, aby już nie myśleć o pieniądzach, ale większość bogatych i tak nie myśli o niczym innym.",
    "Człowiek najpierw pragnie być pięknym, potem bogatym a na końcu tylko zdrowym.",
    "Człowiek z klasą nie rozdrabnia się nad sprawami pieniędzy.",
    "Gdy nie wiadomo o co chodzi, wiadomo, że chodzi o pieniądze. Podobnie jest z konkordatem, który dla finansów państwa okazał się istną czarną dziurą. Pochłania coraz więcej pieniędzy z państwowej kasy, a duchowni wynajdują różne sposoby, by zapewnić finansowanie z niej Kościoła.",
    "Gdy pieniądze mówią, prawda milczy.",
    "Grosz do grosza, a będzie kokosza.",
    "I znowu człowiek wydaje pieniądze, których nie ma, na rzeczy, których nie potrzebuje, by imponować ludziom, których nie lubi.",
    "Inteligencję człowieka można zobaczyć w tym, jak zarabia pieniądze. Jego mądrość w tym, jak je wydaje.",
    "Jeśli możesz policzyć, ile masz pieniędzy, to nie jesteś specjalnie bogaty.",
    "Kobietom pieniądze potrzebne nie są. Bo i po co? Nie piją, w kości nie grają, a kobietami, psiakrew, są przecież same.",
    "Pieniądze są materialną formą zasady mówiącej, że ludzie, którzy chcą załatwiać ze sobą interesy, muszą to robić w formie handlu, płacąc wartością za wartość.",
    "Pieniądze! Ze wszystkich wynalazków ludzkości – ten wynalazek jest najbliższy szatanowi.",
    "W sferze materialnej dawać znaczy być bogatym. Nie jest bogatym ten, kto dużo ma, lecz ten, kto dużo daje.",
    "Z pieniędzmi nie jest tak dobrze, jak jest źle bez nich.",
    "Ziarnko do ziarnka zbierając, do niczego nie dojdziesz, chyba żebyś żył kilkaset lat.",
    "Żyje się za pieniądze, ale nie warto żyć dla pieniędzy.",
];

static COUNTER: AtomicU64 = AtomicU64::new(0);

fn get_month_name(a: u16) -> &'static str {
    match a {
        1 => "styczeń",
        2 => "luty",
        3 => "marzec",
        4 => "kwiecień",
        5 => "maj",
        6 => "czerwiec",
        7 => "lipiec",
        8 => "sierpień",
        9 => "wrzesień",
        10 => "październik",
        11 => "listopad",
        12 => "grudzień",
        _ => "nieznany miesiąc",
    }
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
enum Currency {
    // Remember to update `try_from` when adding new currencies.
    PLN,
    EUR,
    USD,
    GBP,
}

impl ToSql for Currency {
    fn to_sql(&self) -> Result<ToSqlOutput<'_>> {
        Ok(ToSqlOutput::from(format!("{:?}", self))) // TODO: don't abuse debug formatting
    }
}

impl TryFrom<String> for Currency {
    type Error = ();

    fn try_from(value: String) -> std::result::Result<Self, Self::Error> {
        match value.as_str() {
            "PLN" => Ok(Currency::PLN),
            "EUR" => Ok(Currency::EUR),
            "USD" => Ok(Currency::USD),
            "GBP" => Ok(Currency::GBP),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
struct SpecificMonth {
    year: u16,
    month: u16,
}

impl Serialize for SpecificMonth {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // We use `Serialize` only for templating, let's reuse `Display`.
        serializer.serialize_str(format!("{}", self).as_str())
    }
}

impl TryFrom<String> for SpecificMonth {
    type Error = ();

    fn try_from(value: String) -> std::result::Result<Self, Self::Error> {
        // Expected format: "2022-12" or "2022-12-whatever".
        let parts: Vec<_> = value.split('-').collect();

        #[allow(clippy::get_first)]
        let (year_str, month_str) = (parts.get(0).ok_or(())?, parts.get(1).ok_or(())?);

        Ok(SpecificMonth {
            year: year_str.parse().unwrap(),
            month: month_str.parse().unwrap(),
        })
    }
}

impl Display for SpecificMonth {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", get_month_name(self.month), self.year)
    }
}

struct Expense {
    name: String,
    value: f64,
    date: SpecificMonth,
    currency: Currency,
}

impl Expense {
    fn to_eur_approx(&self) -> f64 {
        self.value
            * match self.currency {
                Currency::PLN => 0.21,
                Currency::EUR => 1.00,
                Currency::USD => 0.94,
                Currency::GBP => 1.15,
            }
    }
}

impl Serialize for Expense {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // We use `Serialize` only for templating, let's reuse `Display`.
        serializer.serialize_str(format!("{}", self).as_str())
    }
}

impl Display for Expense {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let income = self.value < 0.0;
        write!(
            f,
            "{} ({})",
            self.name,
            match (income, &self.currency) {
                (false, Currency::PLN) => format!("{} zł", self.value),
                (true, Currency::PLN) => format!("+{} zł", -self.value),
                (false, Currency::EUR) => format!("€{}", self.value),
                (true, Currency::EUR) => format!("+€{}", -self.value),
                (false, Currency::USD) => format!("${}", self.value),
                (true, Currency::USD) => format!("+${}", -self.value),
                (false, Currency::GBP) => format!("£{}", self.value),
                (true, Currency::GBP) => format!("+£{}", -self.value),
            },
        )
    }
}

impl std::fmt::Debug for Expense {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // Reuse `Display`.
        write!(f, "{}", self)
    }
}

trait Repository {
    fn add(&self, name: String, value: f64, date: NaiveDate, currency: Currency);
    fn list(&self) -> Vec<Expense>;
    fn balance(&self) -> HashMap<Currency, i64>;
    fn get_notepad(&self) -> String;
}

struct SQLiteRepository {
    connection: Connection,
}

impl Repository for SQLiteRepository {
    fn add(&self, name: String, value: f64, date: NaiveDate, currency: Currency) {
        self.connection
            .execute(
                "insert into expenses (name, value, date, currency) values (?1, ?2, ?3, ?4)",
                (name, value, date.format("%Y-%m-%d").to_string(), currency),
            )
            .unwrap();
    }

    fn list(&self) -> Vec<Expense> {
        let mut expenses: Vec<Expense> = Vec::new();
        let mut statement = self.connection.prepare("select name, cast(value as real), date, currency from expenses order by date desc, rowid desc").unwrap();
        let expenses_iter = statement
            .query_map([], |row| {
                Ok(Expense {
                    name: row.get(0)?,
                    value: row.get(1)?,
                    date: row.get::<usize, String>(2)?.try_into().unwrap(),
                    currency: row.get::<usize, String>(3).unwrap().try_into().unwrap(),
                })
            })
            .unwrap();
        for expense in expenses_iter {
            expenses.push(expense.unwrap());
        }

        expenses
    }

    fn balance(&self) -> HashMap<Currency, i64> {
        let mut map = HashMap::new();
        let mut p = self
            .connection
            .prepare("select currency, -sum(value) from expenses group by currency")
            .unwrap();

        let balance_iter = p
            .query_map([], |row| {
                let currency: String = row.get(0)?;
                let value: f64 = row.get(1)?;
                Ok((currency, value))
            })
            .unwrap();
        for bal in balance_iter {
            let (currency, value) = bal.unwrap();
            map.insert(currency.try_into().unwrap(), value.floor() as i64);
        }
        map
    }

    fn get_notepad(&self) -> String {
        self.connection
            .query_row(
                "select value from key_value_store where key = 'notepad'",
                [],
                |row| row.get(0),
            )
            .unwrap()
    }
}

async fn root() -> axum::response::Html<String> {
    let mut grouped_expenses: HashMap<SpecificMonth, Vec<Expense>> = HashMap::new();
    let repo = get_repo();
    for expense in repo.list() {
        let month = expense.date;
        if let Some(v) = grouped_expenses.get_mut(&month) {
            v.push(expense);
        } else {
            grouped_expenses.insert(month, vec![expense]);
        }
    }

    let mut grouped_expenses = grouped_expenses.iter().collect::<Vec<_>>();
    grouped_expenses.sort_by_key(|x| Reverse((x.0.year, x.0.month)));

    let r = render!(
r#"
<!DOCTYPE html>
<html lang="pl">

<head>
    <meta charset="UTF-8">
    <title>Casa</title>
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/water.css@2/out/water.css">
    <link rel="apple-touch-icon" sizes="192x192" href="/icon.png">
    <link rel="manifest" href="/manifest.json" />
</head>

<body>
    <header>
        <h1>Casa</h1>
        <form action="/add" method="post">
            <input placeholder="Kremówki papieskie" autocomplete="off" name="name">
            <input id="value" autocomplete="off" placeholder="21,37" inputmode="decimal" pattern="[0-9]+(,[0-9]{2})?" type="text" name="value">
            <select name="currency" id="currency">
                <option value="">-- Wybierz walutę --</option>
                <option value="EUR">EUR</option>
                <option value="PLN">PLN</option>
                <option value="USD">USD</option>
                <option value="GBP">GBP</option>
		    </select>
            <input type="date" name="date" value="{{ today }}">
            <button type="submit">Dodaj</button>
        </form>
    </header>
    <a href="/">Odśwież</a>
    {% for (month, expenses) in grouped_expenses %}
        <details{% if loop.first %} open{% endif %}>
        <summary>{{ month }}</summary>
        {% for expense in expenses %}
          <p>{{ expense | escape }}</p>
        {% endfor %}
        </details>
    {% endfor %}
    <details>
    <summary>Podsumowanie</summary>
        <p><strong>tl;dr: ~€{{ total_eur }} łącznie.</strong></p>
        <ul>
            {% for (cur, bal) in balance %}
                <li>{{ cur }}: {{ bal }}</li>
            {% endfor %}
        </ul>
        <p>{{ notepad }}</p>
    </details>
    <footer>
        <p>
            <small>{{ random_quote }}</small>
        </p>
        <p><small>Liczba wizyt od ostatniego restartu: {{ visit_counter }}.</small></p>
        <p>Made with 🦀 by Adrian Sadłocha.</p>
    </footer>

"#,
        random_quote => QUOTES.iter().collect::<Vec<_>>().choose(&mut rand::thread_rng()),
        grouped_expenses => grouped_expenses,
        today => chrono::offset::Utc::now().format("%Y-%m-%d").to_string(),
        balance => repo.balance().iter().collect::<Vec<_>>(),
        notepad => repo.get_notepad(),
        visit_counter => COUNTER.fetch_add(1, Ordering::SeqCst),
        total_eur => {
            let mut total: f64 = 0.0;
            total += grouped_expenses
                .iter()
                .map(|pair| pair.1.iter().map(Expense::to_eur_approx).sum::<f64>())
                .sum::<f64>();
            -total.floor() as i64
        },
    );
    axum::response::Html(r)
}

#[derive(Debug, Deserialize)]
struct NewExpense {
    name: String,
    value: String,
    currency: Currency,
    date: String,
}

async fn add_expense(Form(new_expense): Form<NewExpense>) -> Redirect {
    let repo = get_repo();
    let date = NaiveDate::parse_from_str(new_expense.date.as_str(), "%Y-%m-%d").unwrap();
    let value = new_expense.value.replace(',', ".").parse().unwrap();
    repo.add(new_expense.name, value, date, new_expense.currency);
    Redirect::to("/")
}

async fn manifest() -> impl axum::response::IntoResponse {
    r##"{
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
                "type": "image/png"
            }
        ]
    }
"##
}

async fn icon() -> impl axum::response::IntoResponse {
    static ICON: &[u8; 24378] = include_bytes!("../icon.png");
    (
        axum::response::AppendHeaders([(axum::http::header::CONTENT_TYPE, "image/png")]),
        ICON.to_vec(),
    )
}

fn get_repo() -> SQLiteRepository {
    SQLiteRepository {
        connection: Connection::open("./db.sqlite").unwrap(),
    }
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root))
        .route("/add", post(add_expense))
        .route("/manifest.json", get(manifest))
        .route("/icon.png", get(icon));

    let addr = SocketAddr::from(([127, 0, 0, 1], 2137));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
