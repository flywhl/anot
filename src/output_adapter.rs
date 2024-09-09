use crate::annotation::Annotation;
use serde_json;
use serde_yaml;

pub enum OutputAdapter {
    Json(JsonAdapter),
    Yaml(YamlAdapter),
}

impl OutputAdapter {
    pub fn format(&self, annotations: &[Annotation]) -> String {
        match self {
            OutputAdapter::Json(adapter) => adapter.format(annotations),
            OutputAdapter::Yaml(adapter) => adapter.format(annotations),
        }
    }
}

pub struct JsonAdapter;

impl JsonAdapter {
    pub fn format(&self, annotations: &[Annotation]) -> String {
        serde_json::to_string_pretty(annotations).unwrap_or_else(|_| "[]".to_string())
    }
}

pub struct YamlAdapter;

impl YamlAdapter {
    pub fn format(&self, annotations: &[Annotation]) -> String {
        serde_yaml::to_string(annotations).unwrap_or_else(|_| "[]".to_string())
    }
}
