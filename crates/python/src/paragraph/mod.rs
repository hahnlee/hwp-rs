pub mod char;

use hwp::hwp::paragraph::{
    control::{paragraph_list::ParagraphList, Control},
    Paragraph,
};
use pyo3::prelude::*;
use pyo3::types::PyDict;

use self::char::PyChar;

#[derive(Clone)]
#[pyclass(name = "Paragraph")]
pub struct PyParagraph {
    paragraph: Paragraph,
}

#[pymethods]
impl PyParagraph {
    #[getter]
    fn chars(&self) -> Vec<Py<PyAny>> {
        let mut chars = Vec::new();
        for char in &self.paragraph.char_list.chars {
            chars.push(Python::with_gil(|py| {
                Py::new(py, PyChar::from_char(char)).unwrap().into_py(py)
            }));
        }
        chars
    }

    fn __str__(&self) -> String {
        self.paragraph.to_string()
    }

    #[args(kwargs = "**")]
    pub fn find_all(&self, tag: &str, kwargs: Option<&PyDict>) -> Vec<Py<PyAny>> {
        let recursive = if kwargs.is_some() {
            let option = kwargs.unwrap().get_item("recursive");
            if option.is_some() {
                option.unwrap().is_true().unwrap_or_else(|_| true)
            } else {
                true
            }
        } else {
            true
        };

        if tag == "paragraph" {
            return self.find_paragraph(recursive);
        }
        vec![]
    }
}

impl PyParagraph {
    pub fn from_paragraph(paragraph: &Paragraph) -> Self {
        Self {
            paragraph: paragraph.clone(),
        }
    }

    pub fn to_py_any(&self) -> Py<PyAny> {
        Python::with_gil(|py| self.clone().into_py(py))
    }

    fn find_paragraph(&self, recursive: bool) -> Vec<Py<PyAny>> {
        let mut result = vec![];

        for control in &self.paragraph.controls {
            match control {
                Control::Table(control) => {
                    for cell in &control.cells {
                        result = concat_paragraph_in_list(result, &cell.paragraph_list, recursive);
                    }

                    // TODO: (@hahnlee) 순서 피드백 필요
                    if control.common_properties.caption.is_some() {
                        let caption = control.common_properties.caption.as_ref().unwrap();
                        result =
                            concat_paragraph_in_list(result, &caption.paragraph_list, recursive);
                    }
                }
                // 개체 공통 속성 컨트롤
                // TODO: (@hahnlee) 더 좋은 방법 찾기
                Control::GenShapeObject(control) => {
                    if control.common_properties.caption.is_some() {
                        let caption = control.common_properties.caption.as_ref().unwrap();
                        result =
                            concat_paragraph_in_list(result, &caption.paragraph_list, recursive);
                    }
                }
                Control::ShapeLine(control) => {
                    if control.common_properties.caption.is_some() {
                        let caption = control.common_properties.caption.as_ref().unwrap();
                        result =
                            concat_paragraph_in_list(result, &caption.paragraph_list, recursive);
                    }
                }
                Control::ShapeRectangle(control) => {
                    if control.common_properties.caption.is_some() {
                        let caption = control.common_properties.caption.as_ref().unwrap();
                        result =
                            concat_paragraph_in_list(result, &caption.paragraph_list, recursive);
                    }
                }
                Control::ShapeEllipse(control) => {
                    if control.common_properties.caption.is_some() {
                        let caption = control.common_properties.caption.as_ref().unwrap();
                        result =
                            concat_paragraph_in_list(result, &caption.paragraph_list, recursive);
                    }
                }
                Control::ShapeArc(control) => {
                    if control.common_properties.caption.is_some() {
                        let caption = control.common_properties.caption.as_ref().unwrap();
                        result =
                            concat_paragraph_in_list(result, &caption.paragraph_list, recursive);
                    }
                }
                Control::ShapePolygon(control) => {
                    if control.common_properties.caption.is_some() {
                        let caption = control.common_properties.caption.as_ref().unwrap();
                        result =
                            concat_paragraph_in_list(result, &caption.paragraph_list, recursive);
                    }
                }
                Control::ShapeCurve(control) => {
                    if control.common_properties.caption.is_some() {
                        let caption = control.common_properties.caption.as_ref().unwrap();
                        result =
                            concat_paragraph_in_list(result, &caption.paragraph_list, recursive);
                    }
                }
                Control::Equation(control) => {
                    if control.common_properties.caption.is_some() {
                        let caption = control.common_properties.caption.as_ref().unwrap();
                        result =
                            concat_paragraph_in_list(result, &caption.paragraph_list, recursive);
                    }
                }
                Control::Picture(control) => {
                    if control.common_properties.caption.is_some() {
                        let caption = control.common_properties.caption.as_ref().unwrap();
                        result =
                            concat_paragraph_in_list(result, &caption.paragraph_list, recursive);
                    }
                }
                Control::Ole(control) => {
                    if control.common_properties.caption.is_some() {
                        let caption = control.common_properties.caption.as_ref().unwrap();
                        result =
                            concat_paragraph_in_list(result, &caption.paragraph_list, recursive);
                    }
                }
                Control::Container(control) => {
                    if control.common_properties.caption.is_some() {
                        let caption = control.common_properties.caption.as_ref().unwrap();
                        result =
                            concat_paragraph_in_list(result, &caption.paragraph_list, recursive);
                    }
                }
                // 개체 이외 컨트롤 + 문단리스트
                Control::Header(control) | Control::Footer(control) => {
                    result = concat_paragraph_in_list(result, &control.paragraph_list, recursive);
                }
                Control::Footnote(control) | Control::Endnote(control) => {
                    result = concat_paragraph_in_list(result, &control.paragraph_list, recursive);
                }
                // TODO: (@hahnlee) HiddenComment 같은건 어떻게 할지?
                _ => {}
            }
        }

        result
    }
}

fn concat_paragraph_in_list(
    mut result: Vec<Py<PyAny>>,
    list: &ParagraphList,
    recursive: bool,
) -> Vec<Py<PyAny>> {
    for paragraph in &list.paragraphs {
        let py_paragraph = PyParagraph::from_paragraph(paragraph);
        result.push(py_paragraph.to_py_any());

        if recursive {
            let children = py_paragraph.find_paragraph(recursive);
            result = [result, children].concat();
        }
    }

    result
}
