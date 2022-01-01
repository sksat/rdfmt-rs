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
        .location(Location {
            path: Some("<file path>".to_string()),
            range: Some(Range { start, end: None }),
        })
        .message("<msg>".to_string());

    assert_eq!(
        r#"{"location":{"path":"<file path>","range":{"start":{"column":15,"line":14}}},"message":"<msg>","severity":"ERROR"}"#,
        serde_json::to_string(&rdjsonl).unwrap(),
    );
}
