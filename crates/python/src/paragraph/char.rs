use hwp::hwp::paragraph::char::Char;
use pyo3::prelude::*;

#[derive(Clone)]
#[pyclass(name = "Char")]
pub struct PyChar {
    #[pyo3(get)]
    pub kind: String,

    #[pyo3(get)]
    pub code: u16,

    #[pyo3(get)]
    pub data: Option<[u8; 12]>,
}

impl PyChar {
    pub fn from_char(char: &Char) -> Self {
        match char {
            Char::CharCode(code) => Self {
                kind: format!("char_code"),
                code: code.clone(),
                data: None,
            },
            Char::CharControl(code) => Self {
                kind: format!("char_control"),
                code: code.clone() as u16,
                data: None,
            },
            Char::InlineControl(code, data) => Self {
                kind: format!("inline_control"),
                code: code.clone(),
                data: Some(data.clone()),
            },
            Char::ExtendedControl(code, data) => Self {
                kind: format!("extended_control"),
                code: code.clone(),
                data: Some(data.clone()),
            },
        }
    }
}
