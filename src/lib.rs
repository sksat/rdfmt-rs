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
pub type SeverityStr = schema::diagnostic::DiagnosticSeverityVariant0;
pub type SeverityInt = schema::diagnostic::DiagnosticSeverityVariant1;

use serde::{Deserialize, Serialize};

// https://github.com/reviewdog/reviewdog/blob/v0.13.1/proto/rdf/reviewdog.proto#L69-L74
#[derive(Debug, Serialize, Deserialize)]
#[repr(i64)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Severity {
    UnknownSeverity = 0,
    Error = 1,
    Warning = 2,
    Info = 3,
}

impl Severity {
    fn to_variant_name(&self) -> String {
        serde_variant::to_variant_name(&self)
            .expect("Severity variant name")
            .to_string()
    }
}

impl From<Severity> for schema::diagnostic::DiagnosticSeverity {
    fn from(from: Severity) -> Self {
        Self::Variant0(from.to_variant_name())
    }
}
impl From<Severity> for schema::diagnosticresult::DiagnosticResultSeverity {
    fn from(from: Severity) -> Self {
        Self::Variant0(from.to_variant_name())
    }
}

impl From<Diagnostic> for schema::diagnosticresult::DiagnosticResultItemDiagnostics {
    fn from(from: Diagnostic) -> Self {
        let serialized = serde_json::to_string(&from).expect("serialize failed");
        serde_json::from_str(&serialized).expect("deserialize failed")
    }
}

impl From<Source> for schema::diagnosticresult::ReviewdogRdfSource {
    fn from(from: Source) -> Self {
        let serialized = serde_json::to_string(&from).expect("serialize failed");
        serde_json::from_str(&serialized).expect("deserialize failed")
    }
}

#[derive(Debug)]
pub struct Rule {
    source: Source,
    code: Code,
}

impl Diagnostic {
    // severity
    pub fn error() -> Self {
        Self::from_severity(Severity::Error)
    }
    pub fn warning() -> Self {
        Self::from_severity(Severity::Warning)
    }
    pub fn info() -> Self {
        Self::from_severity(Severity::Info)
    }
    pub fn from_severity(severity: Severity) -> Self {
        Self {
            severity: Some(severity.into()),
            ..Default::default()
        }
    }

    pub fn with_rule(self, rule: Rule) -> Self {
        let mut d = self;
        d.source = Some(rule.source);
        d.code = Some(rule.code);
        d
    }

    pub fn with_location(self, location: Location) -> Self {
        let mut d = self;
        d.location = Some(location);
        d
    }

    pub fn with_message(self, message: String) -> Self {
        let mut d = self;
        d.message = Some(message);
        d
    }

    pub fn with_code(self, code: Code) -> Self {
        let mut d = self;
        d.code = Some(code);
        d
    }

    pub fn with_suggest(self, suggest: String) -> Self {
        let mut d = self.clone();
        let range = self
            .location
            .expect("location is None")
            .range
            .expect("location.range is None");
        let suggest = Suggestion {
            range: Some(range),
            text: Some(suggest),
        };
        if let Some(ref mut s) = d.suggestions {
            s.push(suggest);
        } else {
            d.suggestions = Some(vec![suggest]);
        }
        d
    }
}

impl DiagnosticResult {
    // severity
    pub fn error() -> Self {
        Self::from_severity(Severity::Error)
    }
    pub fn warning() -> Self {
        Self::from_severity(Severity::Warning)
    }
    pub fn info() -> Self {
        Self::from_severity(Severity::Info)
    }
    pub fn from_severity(severity: Severity) -> Self {
        Self {
            severity: Some(severity.into()),
            ..Default::default()
        }
    }

    pub fn with_source(self, source: Source) -> Self {
        let mut d = self;
        d.source = Some(source.into());
        d
    }

    pub fn with_diagnost(self, diagnostic: Diagnostic) -> Self {
        let mut d = self;
        if let Some(ref mut ds) = d.diagnostics {
            ds.push(diagnostic.into());
        } else {
            d.diagnostics = Some(vec![diagnostic.into()]);
        }
        d
    }
}

impl Position {
    pub fn new(line: usize, column: usize) -> Self {
        let line = Some(line as i64);
        let column = Some(column as i64);
        Self { line, column }
    }
}
