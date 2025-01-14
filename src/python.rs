use pyo3::prelude::*;

use crate::annotation::{Annotation, CitationContext, Location};
use crate::input::FileType;
use crate::parser;
use crate::render::{JsonAdapter, RenderAdapter, YamlAdapter};
use crate::run;

#[pyclass(name = "Location")]
#[derive(Clone)]
struct PyLocation {
    #[pyo3(get)]
    file: String,
    #[pyo3(get)]
    line: usize,
    #[pyo3(get)]
    inline: bool,
}

#[pyclass(name = "Annotation")]
#[derive(Clone)]
struct PyAnnotation {
    #[pyo3(get)]
    kind: String,
    #[pyo3(get)]
    content: String,
    #[pyo3(get)]
    location: PyLocation,
}

#[pymethods]
impl PyAnnotation {
    #[new]
    fn new(kind: String, content: String) -> Self {
        Self {
            kind,
            content,
            location: PyLocation {
                file: String::from("<string>"),
                line: 0,
                inline: false,
            },
        }
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

    let dummy_path = std::path::PathBuf::from("<string>");
    let annotations = parser::extract_annotations(content, &ft, &dummy_path)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;

    Ok(annotations
        .into_iter()
        .map(|a| PyAnnotation {
            kind: a.kind,
            content: a.content,
            location: PyLocation {
                file: a.location.file.to_string_lossy().into_owned(),
                line: a.location.line,
                inline: a.location.inline,
            },
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
            location: Location {
                file: std::path::PathBuf::from("<string>"),
                line: 0,
                inline: a.location.inline,
            },
            context: CitationContext {
                node_type: String::new(),
                parent_type: String::new(),
                associated_name: None,
                line_number: 0,
                variable_name: None,
            },
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

// @todo: remove once https://github.com/PyO3/maturin/issues/368 is resolved
#[pyfunction]
fn run_cli(args: Vec<String>) -> PyResult<()> {
    let _ = run(args);
    Ok(())
}

#[pymodule]
fn _anot(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyAnnotation>()?;
    m.add_function(wrap_pyfunction!(extract_annotations, m)?)?;
    m.add_function(wrap_pyfunction!(format_annotations, m)?)?;
    m.add_function(wrap_pyfunction!(run_cli, m)?)?;
    Ok(())
}
