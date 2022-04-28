use pyo3::prelude::{pyfunction, pymodule, wrap_pyfunction, PyModule, PyResult, Python};

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

#[pyfunction]
fn get_month_name(a: usize) -> PyResult<String> {
    let r = match a {
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
    };
    Ok(r.to_string())
}

#[pymodule]
fn casa(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(get_month_name, m)?)?;
    m.add("QUOTES", QUOTES)?;
    Ok(())
}
