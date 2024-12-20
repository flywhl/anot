use crate::annotation::Annotation;
use crate::error::AnnotError;

pub enum RenderAdapter {
    Json(JsonAdapter),
    Yaml(YamlAdapter),
}

impl RenderAdapter {
    pub fn format(&self, annotations: &[Annotation]) -> Result<String, AnnotError> {
        match self {
            RenderAdapter::Json(adapter) => adapter.format(annotations),
            RenderAdapter::Yaml(adapter) => adapter.format(annotations),
        }
    }
}

pub struct JsonAdapter;

impl JsonAdapter {
    pub fn format(&self, annotations: &[Annotation]) -> Result<String, AnnotError> {
        serde_json::to_string_pretty(annotations)
            .map_err(|e| AnnotError::Serialization(e.to_string()))
    }
}

pub struct YamlAdapter;

impl YamlAdapter {
    pub fn format(&self, annotations: &[Annotation]) -> Result<String, AnnotError> {
        serde_yaml::to_string(annotations).map_err(|e| AnnotError::Serialization(e.to_string()))
    }
}
