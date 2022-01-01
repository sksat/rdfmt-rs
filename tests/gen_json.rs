use rdfmt::*;

#[test]
fn gen_rdjsonl() {
    let start = Some(Position {
        line: Some(14),
        column: Some(15),
    });
    let end = Position {
        line: Some(14),
        column: Some(18),
    };
    let rdjsonl = RdJsonl {
        code: None,
        source: None,
        original_output: None,
        suggestions: None,
        severity: Some(Severity::Error.into()),
        message: Some("<msg>".to_string()),
        location: Some(Location {
            path: Some("<file path>".to_string()),
            range: Some(Range { start, end: None }),
        }),
    };

    assert_eq!(
        r#"{"location":{"path":"<file path>","range":{"start":{"column":15,"line":14}}},"message":"<msg>","severity":"ERROR"}"#,
        serde_json::to_string(&rdjsonl).unwrap(),
    );
}
