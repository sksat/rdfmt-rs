pub mod schema;

pub use schema::{Diagnostic, DiagnosticResult};

pub type Severity = Diagnostic::DiagnosticSeverity;
pub type Location = Diagnostic::DiagnosticLocation;
pub type Range = Diagnostic::ReviewdogRdfRange;
pub type Position = Diagnostic::ReviewdogRdfPosition;
pub type Suggestion = Diagnostic::DiagnosticItemSuggestions;
pub type Source = Diagnostic::DiagnosticSource;
pub type Code = Diagnostic::DiagnosticCode;
