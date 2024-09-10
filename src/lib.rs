pub mod annotation;
pub mod input;
pub mod parser;
pub mod output_adapter;
pub mod cli;
pub mod error;

// Re-export main components for easier use
pub use annotation::Annotation;
pub use input::{read_file, determine_file_type, FileType};
pub use parser::extract_annotations;
pub use output_adapter::OutputAdapter;
pub use cli::{Cli, OutputFormat, parse_args};
pub use error::AnnotError;
