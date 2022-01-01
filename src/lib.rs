pub mod schema;

pub type Diagnostic = schema::diagnostic::Diagnostic;
pub type DiagnosticResult = schema::diagnosticresult::DiagnosticResult;

pub type RdJsonl = Diagnostic;
pub type RdJson = DiagnosticResult;

pub type Severity = schema::diagnostic::DiagnosticSeverity;
pub type Location = schema::diagnostic::DiagnosticLocation;
pub type Range = schema::diagnostic::ReviewdogRdfRange;
pub type Position = schema::diagnostic::ReviewdogRdfPosition;
pub type Suggestion = schema::diagnostic::DiagnosticItemSuggestions;
pub type Source = schema::diagnostic::DiagnosticSource;
pub type Code = schema::diagnostic::DiagnosticCode;
