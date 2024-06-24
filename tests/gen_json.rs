use rdfmt::*;

#[test]
fn gen_rdjsonl() {
    let start = Some(Position {
        line: Some(14),
        column: Some(15),
    });
    let _end = Position {
        line: Some(14),
        column: Some(18),
    };
    let rdjsonl = RdJsonl::error()
        .with_location(Location {
            path: Some("<file path>".to_string()),
            range: Some(Range { start, end: None }),
        })
        .with_message("<msg>".to_string());

    assert_eq!(
        r#"{"location":{"path":"<file path>","range":{"start":{"column":15,"line":14}}},"message":"<msg>","severity":"ERROR"}"#,
        serde_json::to_string(&rdjsonl).unwrap(),
    );
}

#[test]
fn gen_rdjson() {
    let rdjson = RdJson::warning()
        .with_source(
            Source {
                name: Some("super lint".to_string()),
                url: Some("https://example.com/url/to/super-lint".to_string()),
            },
        )
        .with_diagnost(
            Diagnostic::error()
                .with_message("<msg>".to_string())
                .with_location(Location {
                    path: Some("<file path>".to_string()),
                    range: Some(Range {
                        start: Some(Position::new(14, 15)),
                        end: None,
                    }),
                })
                .with_code(Code {
                    value: Some("RULE1".to_string()),
                    url: Some("https://example.com/url/to/super-lint/RULE1".to_string()),
                }),
        )
        .with_diagnost(
            Diagnostic::warning()
                .with_message("<msg>".to_string())
                .with_location(Location {
                    path: Some("<file path>".to_string()),
                    range: Some(Range {
                        start: Some(Position::new(14, 15)),
                        end: Some(Position::new(14, 18)),
                    }),
                })
                .with_suggest("<replacement text>".to_string()),
        );
    let json_str = serde_json::to_string_pretty(&rdjson).unwrap();

    println!(
        "diff: {}",
        prettydiff::diff_lines(RDJSON_EXAMPLE, &json_str)
    );
    assert_eq!(RDJSON_EXAMPLE, json_str);
}

const RDJSON_EXAMPLE: &str = r#"{
  "diagnostics": [
    {
      "code": {
        "url": "https://example.com/url/to/super-lint/RULE1",
        "value": "RULE1"
      },
      "location": {
        "path": "<file path>",
        "range": {
          "start": {
            "column": 15,
            "line": 14
          }
        }
      },
      "message": "<msg>",
      "severity": "ERROR"
    },
    {
      "location": {
        "path": "<file path>",
        "range": {
          "end": {
            "column": 18,
            "line": 14
          },
          "start": {
            "column": 15,
            "line": 14
          }
        }
      },
      "message": "<msg>",
      "severity": "WARNING",
      "suggestions": [
        {
          "range": {
            "end": {
              "column": 18,
              "line": 14
            },
            "start": {
              "column": 15,
              "line": 14
            }
          },
          "text": "<replacement text>"
        }
      ]
    }
  ],
  "severity": "WARNING",
  "source": {
    "name": "super lint",
    "url": "https://example.com/url/to/super-lint"
  }
}"#;
