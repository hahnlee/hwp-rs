use hwp::hwp::paragraph::control::table::{Cell, TableControl};
use pyo3::prelude::*;

use crate::paragraph::{to_py_paragraphs, PyParagraph};

use super::common_properties::PyCommonProperties;

#[derive(Clone)]
#[pyclass(name = "Table")]
pub struct PyTable {
    /// 개체 공통 속성
    #[pyo3(get)]
    pub common_properties: PyCommonProperties,
    /// 행 개수
    #[pyo3(get)]
    pub rows: u16,
    /// 열 개수
    #[pyo3(get)]
    pub cols: u16,
    /// row에 몇개의 column이 있는지 기록 (표준문서의 Row Size)
    #[pyo3(get)]
    pub row_count: Vec<u16>,
    #[pyo3(get)]
    pub cells: Vec<PyCell>,
}

impl PyTable {
    pub fn from_rust(table: &TableControl) -> Self {
        Self {
            common_properties: PyCommonProperties::from_rust(&table.common_properties),
            rows: table.record.rows,
            cols: table.record.cols,
            row_count: table.record.row_count.clone(),
            cells: (&table.cells)
                .into_iter()
                .map(|c| PyCell::from_rust(c))
                .collect(),
        }
    }

    pub fn to_py_any(&self) -> Py<PyAny> {
        Python::with_gil(|py| self.clone().into_py(py))
    }
}

#[derive(Clone)]
#[pyclass(name = "Cell")]
pub struct PyCell {
    /// 열 주소
    ///
    /// 0 부터 시작, 왼쪽으로 갈수록 커진다
    #[pyo3(get)]
    pub column: u16,
    /// 행 주소
    ///
    /// 0 부터 시작, 왼쪽으로 갈수록 커진다
    #[pyo3(get)]
    pub row: u16,
    /// 열의 병합 개수
    #[pyo3(get)]
    pub col_span: u16,
    /// 행의 병합 개수
    #[pyo3(get)]
    pub row_span: u16,
    /// 너비
    #[pyo3(get)]
    pub width: u32,
    /// 높이
    #[pyo3(get)]
    pub height: u32,
    #[pyo3(get)]
    pub padding: [u16; 4],
    /// 문단 리스트
    #[pyo3(get)]
    pub paragraphs: Vec<PyParagraph>,
}

impl PyCell {
    pub fn from_rust(cell: &Cell) -> Self {
        Self {
            column: cell.column,
            row: cell.row,
            col_span: cell.col_span,
            row_span: cell.row_span,
            width: cell.width,
            height: cell.height,
            padding: cell.padding,
            paragraphs: to_py_paragraphs(&cell.paragraph_list),
        }
    }

    pub fn to_py_any(&self) -> Py<PyAny> {
        Python::with_gil(|py| self.clone().into_py(py))
    }
}
