use anot::{extract_annotations, Annotation, FileType};

#[test]
fn test_basic_annotation_extraction() {
    let content = std::fs::read_to_string("tests/data/basic_test.py").unwrap();
    let annotations = extract_annotations(&content, &FileType::Python).unwrap();
    
    assert_eq!(annotations.len(), 2);
    
    assert_eq!(annotations[0], Annotation {
        kind: "note".to_string(),
        content: "this experiment will be re-written later".to_string(),
    });
    
    assert_eq!(annotations[1], Annotation {
        kind: "hypothesis".to_string(),
        content: "5 is better than 4".to_string(),
    });
}
