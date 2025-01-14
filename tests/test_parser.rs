use anot::{annotation::Location, extract_annotations, Annotation, FileType, SyntaxContext};
use std::path::PathBuf;

#[test]
fn test_python_annotation_extraction() {
    let content = std::fs::read_to_string("tests/data/test.py").unwrap();
    let file = PathBuf::from("tests/data/test.py");
    let annotations = extract_annotations(&content, &FileType::Python, &file).unwrap();

    assert_eq!(annotations.len(), 2);

    assert_eq!(
        annotations[0],
        Annotation {
            kind: "note".to_string(),
            content: "this experiment will be re-written later".to_string(),
            context: SyntaxContext {
                node_type: "class_definition".to_string(),
                parent_type: "module".to_string(),
                associated_name: Some("Something".to_string()),
                variable_name: None,
            },
            location: Location {
                file: file.clone(),
                line: 2,
                inline: false,
            }
        }
    );

    assert_eq!(
        annotations[1],
        Annotation {
            kind: "hypothesis".to_string(),
            content: "5 is better than 4".to_string(),
            context: SyntaxContext {
                node_type: "function_definition".to_string(),
                parent_type: "block".to_string(),
                associated_name: Some("run".to_string()),
                variable_name: None,
            },
            location: Location {
                file,
                line: 5,
                inline: true,
            }
        }
    );
}

#[test]
fn test_rust_annotation_extraction() {
    let content = std::fs::read_to_string("tests/data/test.rs").unwrap();
    let file = PathBuf::from("tests/data/test.rs");
    let annotations = extract_annotations(&content, &FileType::Rust, &file).unwrap();

    assert_eq!(annotations.len(), 2);

    assert_eq!(
        annotations[0],
        Annotation {
            kind: "todo".to_string(),
            content: "Add more fields".to_string(),
            context: SyntaxContext {
                node_type: "struct_item".to_string(),
                parent_type: "source_file".to_string(),
                associated_name: Some("Example".to_string()),
                variable_name: None,
            },
            location: Location {
                file: file.clone(),
                line: 2,
                inline: false,
            }
        }
    );

    assert_eq!(
        annotations[1],
        Annotation {
            kind: "fixme".to_string(),
            content: "This needs better error handling".to_string(),
            context: SyntaxContext {
                node_type: "function_item".to_string(),
                parent_type: "impl_item".to_string(),
                associated_name: Some("new".to_string()),
                variable_name: None,
            },
            location: Location {
                file,
                line: 9,
                inline: true,
            }
        }
    );
}

#[test]
fn test_javascript_annotation_extraction() {
    let content = std::fs::read_to_string("tests/data/test.js").unwrap();
    let file = PathBuf::from("tests/data/test.js");
    let annotations = extract_annotations(&content, &FileType::JavaScript, &file).unwrap();

    assert_eq!(annotations.len(), 2);

    assert_eq!(
        annotations[0],
        Annotation {
            kind: "todo".to_string(),
            content: "Add initialization".to_string(),
            context: SyntaxContext {
                node_type: "method_definition".to_string(),
                parent_type: "class_body".to_string(),
                associated_name: Some("constructor".to_string()),
                variable_name: None,
            },
            location: Location {
                file: file.clone(),
                line: 3,
                inline: true
            }
        }
    );

    assert_eq!(
        annotations[1],
        Annotation {
            kind: "bug".to_string(),
            content: "Sometimes fails on Safari".to_string(),
            context: SyntaxContext {
                node_type: "method_definition".to_string(),
                parent_type: "class_body".to_string(),
                associated_name: Some("render".to_string()),
                variable_name: None,
            },
            location: Location {
                file,
                line: 7,
                inline: false
            }
        }
    );
}
