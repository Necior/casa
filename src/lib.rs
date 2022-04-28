use pyo3::prelude::{pyfunction, pymodule, wrap_pyfunction, PyModule, PyResult, Python};

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
    Ok(())
}
