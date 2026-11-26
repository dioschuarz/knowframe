//! Repository filesystem observation and document construction.

pub mod document;
pub mod fs_walk;
pub mod scanner;

pub use document::DocumentNodeBuilder;
pub use scanner::{scan_repository, ScanOptions, ScanResult};
