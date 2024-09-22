use chrono::Local;
use pyo3::exceptions::PyException;
use pyo3::prelude::*;
use reqwest::blocking::Client;
use std::fs::{self, OpenOptions};
use std::io::Write;

#[pyclass]
struct SnusProps {
    api_key: String,
    search_type: String,
    search_param: String,
}

#[pymethods]
impl SnusProps {
    #[new]
    fn new(api_key: String, search_type: String, search_param: String) -> Self {
        SnusProps {
            api_key,
            search_type,
            search_param,
        }
    }

    fn snusbase(&self) -> PyResult<()> {
        todo();
        Ok(())
    }
}

#[pyfunction]
fn user_search(username: &str, write_to_file: bool) -> PyResult<()> {
    let client = Client::new();

    let sites = vec![
        format!("https://instagram.com/{}", username),
        format!("https://github.com/{}", username),
        format!("https://x.com/{}", username),
        format!("https://reddit.com/u/{}", username),
        format!("https://tiktok.com/@{}", username),
        format!("https://imgur.com/user/{}", username),
        format!("https://facebook.com/{}", username),
        format!("https://pinterest.com/{}", username),
        format!("https://t.me/{}", username),
        format!("https://www.tumblr.com/{}", username),
    ];

    let date_time = Local::now().format("%Y%m%d_%H%M%S").to_string();
    let filename = format!("logs/user_search_{}.txt", date_time);

    fs::create_dir_all("logs")
        .map_err(|e| PyErr::new::<PyException, _>(format!("Request Error: {}", e)))?;

    for site in &sites {
        let res = client
            .get(site)
            .send()
            .map_err(|e| PyErr::new::<PyException, _>(format!("Request Failed: {}", e)))?;
        let status = res.status();

        if write_to_file {
            let mut file = OpenOptions::new()
                .append(true)
                .create(true)
                .open(&filename)?;

            writeln!(file, "[{}]: {}", status, site)?;
            println!("File Written. Check logs folder.");
        } else {
            println!("[{}]: {}", status, site);
        }
    }

    Ok(())
}

#[pymodule]
fn vizhu(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(user_search, m)?)?;
    m.add_class::<SnusProps>()?;

    Ok(())
}
