use chrono::Local;
use pyo3::exceptions::PyException;
use pyo3::prelude::*;
use reqwest::blocking::Client;
use serde_json::{json, to_string_pretty, Value};
use std::fs::{self, OpenOptions};
use std::{fs::File, io::Write};
use thiserror::Error;

const SNUS_URL: &str = "https://api.snusbase.com/data/search";

#[derive(Error, Debug)]
enum ErrorHandling {
    #[error("Request Error: {0}")]
    ReqwestErr(#[from] reqwest::Error),

    #[error("JSON Error: {0}")]
    SerdeErr(#[from] serde_json::Error),

    #[error("IO Error: {0}")]
    IOError(#[from] std::io::Error),

    #[error("DateTime Parse Error: {0}")]
    ChronoError(#[from] chrono::ParseError),

    #[error("Formatting Error: {0}")]
    FmtError(#[from] std::fmt::Error),
}

impl From<ErrorHandling> for PyErr {
    fn from(err: ErrorHandling) -> PyErr {
        PyException::new_err(err.to_string())
    }
}

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

    fn snusbase(&self, output_to_file: bool) -> PyResult<()> {
        self.snusbase_internal(output_to_file)?;
        Ok(())
    }
}

impl SnusProps {
    fn snusbase_internal(&self, output_to_file: bool) -> Result<(), ErrorHandling> {
        let client = Client::new();

        let body = json!({
            "type": self.search_type,
            "term": self.search_param,
        });

        let response = client
            .post(SNUS_URL)
            .header("Auth", &self.api_key)
            .header("Content-Type", "application/json")
            .json(&body)
            .send()?
            .error_for_status()?;

        let response_text = response.text()?.trim().to_string();

        let json_value: Value = serde_json::from_str(&response_text)?;

        let pretty_json = to_string_pretty(&json_value)?;

        if output_to_file {
            let filename = format!("logs/snusbase_{}.txt", Local::now().format("%Y%m%d%H%M%S"));

            let mut file = File::create(&filename)?;
            file.write_all(pretty_json.as_bytes())?;

            println!("File written: {}", filename);
        } else {
            println!("{}", pretty_json);
        }

        Ok(())
    }
}

fn user_internal(username: &str, write_to_file: bool) -> Result<(), ErrorHandling> {
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

    fs::create_dir_all("logs")?;

    let date_time = Local::now().format("%Y%m%d_%H%M%S").to_string();
    let filename = format!("logs/user_search_{}.txt", date_time);

    for site in &sites {
        let res = client.get(site).send()?;
        let status = res.status();

        if write_to_file {
            let mut file = OpenOptions::new()
                .append(true)
                .create(true)
                .open(&filename)?;
            writeln!(file, "[{}]: {}", status, site)?;
        } else {
            println!("[{}]: {}", status, site);
        }
    }

    if write_to_file {
        println!("File written: {}", filename);
    }

    Ok(())
}

#[pyfunction]
fn user_search(username: &str, write_to_file: bool) -> PyResult<()> {
    user_internal(username, write_to_file)?;
    Ok(())
}

#[pymodule]
fn vizhu(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(user_search, m)?)?;
    m.add_class::<SnusProps>()?;

    Ok(())
}
