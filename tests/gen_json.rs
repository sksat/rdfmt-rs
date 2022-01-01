use rdfmt::*;

#[test]
fn gen_json() {
    let json = RdJson {
        severity: Severity::WARNING.into(),
        source: Some(Source {
            name: "super lint".to_string(),
            url: "https://example.com/url/to/super-lint".to_string(),
            ..Default::default()
        })
        .into(),
        diagnostics: vec![Diagnostic {
            message: "<msg>".to_string(),
            location: Some(Location {
                path: "<file path>".to_string(),
                range: Some(Range {
                    start: Some(Position {
                        line: 14,
                        column: 15,
                        ..Default::default() //cached_size, unknown_fields
                    })
                    .into(),
                    end: None.into(),
                    ..Default::default() // cached_size, unknown_fields
                })
                .into(),
                ..Default::default() // cached_size, unknown_fields
            })
            .into(),
            code: Some(Code {
                value: "RULE1".to_string(),
                url: "https://example.com/url/to/super-lint/RULE1".to_string(),
                ..Default::default() // cached_size, unknown_fields
            })
            .into(),
            source: None.into(),
            original_output: "this field is optional".to_string(), //None,
            suggestions: vec![],
            severity: Severity::WARNING.into(),
            ..Default::default() // cached_size, unknown_fields
        }],
        ..Default::default() // cached_size, unknown_fields
    };
    let json = serde_json::to_string(&json).unwrap();
    println!("{}", json);
}
