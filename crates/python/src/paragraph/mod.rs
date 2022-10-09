pub mod char;
pub mod control;

use hwp::hwp::paragraph::{
    control::{paragraph_list::ParagraphList, Control},
    Paragraph,
};
use pyo3::prelude::*;
use pyo3::types::PyDict;

use self::{
    char::PyChar,
    control::{common_properties::PyCaption, table::PyTable, equation::PyEquation, footnote_endnote::PyFootnoteEndnote, header_footer::PyHeaderFooter},
};

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

        match tag {
            "paragraph" => self.find_paragraph(recursive),
            "table" => self.find_table(recursive),
            "caption" => self.find_caption(recursive),
            "equation" => self.find_equation(recursive),
            "footnote" => self.find_footnote(recursive),
            "endnote" => self.find_endnote(recursive),
            "header" => self.find_header(),
            "footer" => self.find_footer(),
            _ => vec![],
        }
    }
}

impl PyParagraph {
    pub fn from_rust(paragraph: &Paragraph) -> Self {
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

    fn find_table(&self, recursive: bool) -> Vec<Py<PyAny>> {
        let mut result = vec![];

        // TODO: (@hahnlee) 캡션, 머리말 꼬리말에도 테이블을 넣을 수 있는지 알아보기
        for control in &self.paragraph.controls {
            if let Control::Table(control) = control {
                result.push(PyTable::from_rust(control).to_py_any());

                if recursive {
                    for cell in &control.cells {
                        // TODO: (@hahnlee) 효츌적인 방법 생각하기
                        let paragraphs_list = to_py_paragraphs(&cell.paragraph_list);
                        for paragraph in paragraphs_list {
                            let controls = paragraph.find_table(recursive);
                            result = [result, controls].concat();
                        }
                    }
                }
            }
        }

        result
    }

    // 살...려줘...
    fn find_caption(&self, recursive: bool) -> Vec<Py<PyAny>> {
        let mut result = vec![];

        for control in &self.paragraph.controls {
            match control {
                Control::Table(control) => {
                    // TODO: (@hahnlee) 순서 피드백 필요
                    if control.common_properties.caption.is_some() {
                        let caption = control.common_properties.caption.as_ref().unwrap();
                        result.push(PyCaption::from_rust(caption).to_py_any());
                    }

                    if recursive {
                        for cell in &control.cells {
                            for paragraph in to_py_paragraphs(&cell.paragraph_list) {
                                result = [result, paragraph.find_caption(recursive)].concat();
                            }
                        }
                    }
                }
                // 개체 공통 속성 컨트롤
                // TODO: (@hahnlee) 더 좋은 방법 찾기
                Control::GenShapeObject(control) => {
                    if control.common_properties.caption.is_some() {
                        let caption = control.common_properties.caption.as_ref().unwrap();
                        result.push(PyCaption::from_rust(caption).to_py_any());
                    }
                }
                Control::ShapeLine(control) => {
                    if control.common_properties.caption.is_some() {
                        let caption = control.common_properties.caption.as_ref().unwrap();
                        result.push(PyCaption::from_rust(caption).to_py_any());
                    }
                }
                Control::ShapeRectangle(control) => {
                    if control.common_properties.caption.is_some() {
                        let caption = control.common_properties.caption.as_ref().unwrap();
                        result.push(PyCaption::from_rust(caption).to_py_any());
                    }
                }
                Control::ShapeEllipse(control) => {
                    if control.common_properties.caption.is_some() {
                        let caption = control.common_properties.caption.as_ref().unwrap();
                        result.push(PyCaption::from_rust(caption).to_py_any());
                    }
                }
                Control::ShapeArc(control) => {
                    if control.common_properties.caption.is_some() {
                        let caption = control.common_properties.caption.as_ref().unwrap();
                        result.push(PyCaption::from_rust(caption).to_py_any());
                    }
                }
                Control::ShapePolygon(control) => {
                    if control.common_properties.caption.is_some() {
                        let caption = control.common_properties.caption.as_ref().unwrap();
                        result.push(PyCaption::from_rust(caption).to_py_any());
                    }
                }
                Control::ShapeCurve(control) => {
                    if control.common_properties.caption.is_some() {
                        let caption = control.common_properties.caption.as_ref().unwrap();
                        result.push(PyCaption::from_rust(caption).to_py_any());
                    }
                }
                Control::Equation(control) => {
                    if control.common_properties.caption.is_some() {
                        let caption = control.common_properties.caption.as_ref().unwrap();
                        result.push(PyCaption::from_rust(caption).to_py_any());
                    }
                }
                Control::Picture(control) => {
                    if control.common_properties.caption.is_some() {
                        let caption = control.common_properties.caption.as_ref().unwrap();
                        result.push(PyCaption::from_rust(caption).to_py_any());
                    }
                }
                Control::Ole(control) => {
                    if control.common_properties.caption.is_some() {
                        let caption = control.common_properties.caption.as_ref().unwrap();
                        result.push(PyCaption::from_rust(caption).to_py_any());
                    }
                }
                Control::Container(control) => {
                    if control.common_properties.caption.is_some() {
                        let caption = control.common_properties.caption.as_ref().unwrap();
                        result.push(PyCaption::from_rust(caption).to_py_any());
                    }
                }
                // 개체 이외 컨트롤 + 문단리스트
                Control::Header(control) | Control::Footer(control) => {
                    if recursive {
                        for paragraph in to_py_paragraphs(&control.paragraph_list) {
                            result = [result, paragraph.find_caption(recursive)].concat();
                        }
                    }
                }
                Control::Footnote(control) | Control::Endnote(control) => {
                    if recursive {
                        for paragraph in to_py_paragraphs(&control.paragraph_list) {
                            result = [result, paragraph.find_caption(recursive)].concat();
                        }
                    }
                }
                // TODO: (@hahnlee) HiddenComment 같은건 어떻게 할지?
                _ => {}
            }
        }

        result
    }

    fn find_equation(&self, recursive: bool) -> Vec<Py<PyAny>> {
        let mut result = vec![];

        for control in &self.paragraph.controls {
            match control {
                Control::Equation(control) => {
                    result.push(PyEquation::from_rust(control).to_py_any());
                }
                Control::Table(control) => {
                    if recursive {
                        for cell in &control.cells {
                            for paragraph in to_py_paragraphs(&cell.paragraph_list) {
                                result = [result, paragraph.find_equation(recursive)].concat();
                            }
                        }
                    }
                    // TODO: (@hahnlee) 캡션에 있는지 알아보기
                }
                // 개체 이외 컨트롤 + 문단리스트
                Control::Header(control) | Control::Footer(control) => {
                    if recursive {
                        for paragraph in to_py_paragraphs(&control.paragraph_list) {
                            result = [result, paragraph.find_equation(recursive)].concat();
                        }
                    }
                }
                Control::Footnote(control) | Control::Endnote(control) => {
                    if recursive {
                        for paragraph in to_py_paragraphs(&control.paragraph_list) {
                            result = [result, paragraph.find_equation(recursive)].concat();
                        }
                    }
                }
                _ => {}
            }
        }

        result
    }

    fn find_footnote(&self, recursive: bool) -> Vec<Py<PyAny>> {
        let mut result = vec![];

        for control in &self.paragraph.controls {
            match control {
                Control::Table(control) => {
                    if recursive {
                        for cell in &control.cells {
                            for paragraph in to_py_paragraphs(&cell.paragraph_list) {
                                result = [result, paragraph.find_footnote(recursive)].concat();
                            }
                        }
                    }
                    // TODO: (@hahnlee) 캡션에 있는지 알아보기
                }
                Control::Footnote(control) => {
                    result.push(PyFootnoteEndnote::from_rust(control).to_py_any())
                }
                _ => {}
            }
        }

        result
    }

    fn find_endnote(&self, recursive: bool) -> Vec<Py<PyAny>> {
        let mut result = vec![];

        for control in &self.paragraph.controls {
            match control {
                Control::Table(control) => {
                    if recursive {
                        for cell in &control.cells {
                            for paragraph in to_py_paragraphs(&cell.paragraph_list) {
                                result = [result, paragraph.find_endnote(recursive)].concat();
                            }
                        }
                    }
                    // TODO: (@hahnlee) 캡션에 있는지 알아보기
                }
                Control::Endnote(control) => {
                    result.push(PyFootnoteEndnote::from_rust(control).to_py_any())
                }
                _ => {}
            }
        }

        result
    }

    fn find_header(&self) -> Vec<Py<PyAny>> {
        let mut result = vec![];

        for control in &self.paragraph.controls {
            match control {
                Control::Header(control) => {
                    result.push(PyHeaderFooter::from_rust(control).to_py_any())
                }
                _ => {}
            }
        }

        result
    }

    fn find_footer(&self) -> Vec<Py<PyAny>> {
        let mut result = vec![];

        for control in &self.paragraph.controls {
            match control {
                Control::Footer(control) => {
                    result.push(PyHeaderFooter::from_rust(control).to_py_any())
                }
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
        let py_paragraph = PyParagraph::from_rust(paragraph);
        result.push(py_paragraph.to_py_any());

        if recursive {
            let children = py_paragraph.find_paragraph(recursive);
            result = [result, children].concat();
        }
    }

    result
}

pub fn to_py_paragraphs(list: &ParagraphList) -> Vec<PyParagraph> {
    (&list.paragraphs)
        .into_iter()
        .map(|p| PyParagraph::from_rust(p))
        .collect()
}
