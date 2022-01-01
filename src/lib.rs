pub mod schema;

pub type Diagnostic = schema::diagnostic::Diagnostic;
pub type DiagnosticResult = schema::diagnosticresult::DiagnosticResult;

pub type RdJsonl = Diagnostic;
pub type RdJson = DiagnosticResult;

pub type Location = schema::diagnostic::DiagnosticLocation;
pub type Range = schema::diagnostic::ReviewdogRdfRange;
pub type Position = schema::diagnostic::ReviewdogRdfPosition;
pub type Suggestion = schema::diagnostic::DiagnosticItemSuggestions;
pub type Source = schema::diagnostic::DiagnosticSource;
pub type Code = schema::diagnostic::DiagnosticCode;

pub type SeverityImpl = schema::diagnostic::DiagnosticSeverity;
type SeverityInt = schema::diagnostic::DiagnosticSeverityVariant1;

use serde::{Deserialize, Serialize};

// https://github.com/reviewdog/reviewdog/blob/v0.13.1/proto/rdf/reviewdog.proto#L69-L74
#[derive(Debug, Serialize, Deserialize)]
#[repr(i32)]
pub enum Severity {
    UnknownSeverity = 0,
    Error = 1,
    Warning = 2,
    Info = 3,
}

impl From<Severity> for SeverityImpl {
    fn from(from: Severity) -> SeverityImpl {
        let int = from as SeverityInt;
        SeverityImpl::Variant1(int)
    }
}
