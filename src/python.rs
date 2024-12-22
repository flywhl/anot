use pyo3::prelude::*;

use crate::annotation::Annotation;
use crate::input::FileType;
use crate::parser;
use crate::render::{JsonAdapter, RenderAdapter, YamlAdapter};

#[pyclass(name = "Annotation")]
#[derive(Clone)]
struct PyAnnotation {
    #[pyo3(get)]
    kind: String,
    #[pyo3(get)]
    content: String,
}

#[pymethods]
impl PyAnnotation {
    #[new]
    fn new(kind: String, content: String) -> Self {
        Self { kind, content }
    }
}

#[pyfunction]
fn extract_annotations(content: &str, file_type: &str) -> PyResult<Vec<PyAnnotation>> {
    let ft = match file_type {
        "py" => FileType::Python,
        "rs" => FileType::Rust,
        "js" => FileType::JavaScript,
        _ => FileType::Unknown,
    };

    let annotations = parser::extract_annotations(content, &ft)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;

    Ok(annotations
        .into_iter()
        .map(|a| PyAnnotation {
            kind: a.kind,
            content: a.content,
        })
        .collect())
}

#[pyfunction]
fn format_annotations(annotations: Vec<PyAnnotation>, format: &str) -> PyResult<String> {
    let annotations: Vec<Annotation> = annotations
        .into_iter()
        .map(|a| Annotation {
            kind: a.kind,
            content: a.content,
        })
        .collect();

    let adapter = match format {
        "json" => RenderAdapter::Json(JsonAdapter),
        "yaml" => RenderAdapter::Yaml(YamlAdapter),
        _ => {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                "Invalid format",
            ))
        }
    };

    adapter
        .format(&annotations)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))
}

#[pymodule]
fn _anot(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyAnnotation>()?;
    m.add_function(wrap_pyfunction!(extract_annotations, m)?)?;
    m.add_function(wrap_pyfunction!(format_annotations, m)?)?;
    Ok(())
}
