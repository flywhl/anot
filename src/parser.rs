use crate::annotation::{Annotation, Location, SyntaxContext};
use crate::input::FileType;
use std::path::Path;

use anyhow::Result;
use streaming_iterator::StreamingIterator;

pub const TAG: &str = "@";

/// Extract annotations from file.
///
/// # Arguments
///
/// * `source_code` - Contents of the source code file.
/// * `file_type` - Language of the source code.
/// * `file` - Path to source file.
///
/// # Returns
/// Extracted annotations with their syntactic context.
///
/// # Errors
/// If parsing the source code fails, or the tree-sitter query is invalid.
pub fn extract_annotations(
    source_code: &str,
    file_type: &FileType,
    file: &Path,
) -> Result<Vec<Annotation>> {
    // Set up tree-sitter parser based on file type
    let mut parser = tree_sitter::Parser::new();
    let language = file_type.tree_sitter_language();
    parser.set_language(&language)?;

    // Parse the full source code
    let source = source_code.as_bytes();
    let tree = parser
        .parse(source, None)
        .ok_or_else(|| anyhow::anyhow!("Failed to parse source code"))?;

    let query = file_type.tree_sitter_query();
    let mut query_cursor = tree_sitter::QueryCursor::new();
    let mut matches = query_cursor.matches(query, tree.root_node(), source);

    let mut annotations = Vec::new();

    while let Some(match_) = matches.next() {
        for capture in match_.captures {
            let comment_node = capture.node;
            let comment_text = comment_node.utf8_text(source).unwrap_or("").trim();

            if comment_text.contains(TAG) {
                let line_number = comment_node.start_position().row + 1;
                let is_inline = is_inline_comment(comment_node);
                if let Some(annotation) = parse_annotation(
                    comment_text,
                    line_number,
                    comment_node,
                    source,
                    file_type,
                    file,
                    is_inline,
                ) {
                    annotations.push(annotation);
                }
            }
        }
    }

    Ok(annotations)
}

/// Parse annotation from line. Returns None if the syntax is wrong.
fn parse_annotation(
    line: &str,
    line_number: usize,
    comment_node: tree_sitter::Node,
    source_code: &[u8],
    file_type: &FileType,
    file: &Path,
    is_inline: bool,
) -> Option<Annotation> {
    let at_pos = line.find(TAG)?;
    let colon_pos = line[at_pos..].find(':')?;

    let kind = line[at_pos + 1..at_pos + colon_pos].trim().to_string();
    let content = line[at_pos + colon_pos + 1..].trim().to_string();
    let context = extract_context(comment_node, source_code, file_type);

    Some(Annotation {
        kind,
        content,
        context,
        location: Location {
            file: file.to_path_buf(),
            line: line_number,
            inline: is_inline,
        },
    })
}

/// Extract annotation context from tree sitter node.
/// Traverses the tree from node to prent until it finds a significant (e.g. class definition,
/// struct field, impl block.)
fn extract_context(
    comment_node: tree_sitter::Node,
    source_code: &[u8],
    file_type: &FileType,
) -> SyntaxContext {
    let mut current = comment_node;
    let mut current_kind = current.kind().to_string();
    let mut associated_name = None;
    let mut variable_name = None;
    let mut parent_type = String::new();

    // Keep track of the most significant node found
    let mut most_significant = None;

    // Traverse up until we find a significant node
    while let Some(parent) = current.parent() {
        let parent_kind = parent.kind();

        // Skip comment-related nodes
        if parent_kind.contains("comment") {
            current = parent;
            continue;
        }

        // Update most significant node based on type
        match parent_kind {
            "class_definition"
            | "class_declaration"
            | "struct_item"
            | "function_definition"
            | "function_item"
            | "function_declaration"
            | "method_definition" => {
                most_significant = Some((parent, parent_kind.to_string()));
                break;
            }
            "assignment" | "let_declaration" | "variable_declaration" => {
                if most_significant.is_none() {
                    most_significant = Some((parent, parent_kind.to_string()));
                }
            }
            "impl_item" => {
                parent_type = "impl_item".to_string();
            }
            _ => {}
        }
        current = parent;
    }

    // Use the most significant node found
    if let Some((node, kind)) = most_significant {
        current_kind = kind.clone();
        associated_name = extract_name(&node, "name", source_code);

        // If parent_type hasn't been set for `impl_item`, ensure it is correctly identified
        if parent_type.is_empty() {
            parent_type = node
                .parent()
                .map(|p| p.kind().to_string())
                .unwrap_or_else(|| "root".to_string());

            if parent_type == "declaration_list" {
                // Specifically look deeper if Rust
                if let Some(grandparent) = node.parent().and_then(|p| p.parent()) {
                    if grandparent.kind() == "impl_item" {
                        parent_type = "impl_item".to_string();
                    }
                }
            }
        }

        // Handle variable names for assignment nodes
        if matches!(
            kind.as_str(),
            "assignment" | "let_declaration" | "variable_declaration"
        ) {
            let field_name = match file_type {
                FileType::Python => "left",
                FileType::Rust => "pattern",
                FileType::JavaScript => "declarator",
            };
            variable_name = extract_name(&node, field_name, source_code);
        }
    }

    SyntaxContext {
        node_type: current_kind,
        parent_type,
        associated_name,
        variable_name,
    }
}

/// Extract associated name (e.g. variable, function, class names, etc.) from context node.
fn extract_name(node: &tree_sitter::Node, field_name: &str, source_code: &[u8]) -> Option<String> {
    node.child_by_field_name(field_name)
        .and_then(|name_node| name_node.utf8_text(source_code).ok())
        .map(|text| text.to_string())
}

/// Determine if the comment is inline or on its own
fn is_inline_comment(comment_node: tree_sitter::Node) -> bool {
    if let Some(prev_sibling) = comment_node.prev_sibling() {
        // If previous node ends on same line as our comment starts, this is an inline comment
        prev_sibling.end_position().row == comment_node.start_position().row
    } else {
        false
    }
}
