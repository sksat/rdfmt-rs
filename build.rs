use std::env;
use std::fs;
use std::io::Write;
use std::path::Path;

use protobuf_codegen_pure::Customize;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = env::var("OUT_DIR").unwrap();
    let gen_dir = format!("{}/generated", out_dir);

    if Path::new(&gen_dir).exists() {
        fs::remove_dir_all(&gen_dir).unwrap();
    }
    fs::create_dir(&gen_dir).unwrap();

    // get protobuf definition from github
    let proto_path = format!("{}/reviewdog.proto", gen_dir);
    let reviewdog_proto =
        "https://raw.githubusercontent.com/reviewdog/reviewdog/v0.13.1/proto/rdf/reviewdog.proto";
    let reviewdog_proto = reqwest::blocking::get(reviewdog_proto)?.text()?;
    let mut file = fs::File::create(&proto_path)?;
    file.write_all(reviewdog_proto.as_bytes())?;

    protobuf_codegen_pure::Codegen::new()
        .customize(Customize {
            serde_derive: Some(true),
            gen_mod_rs: Some(true),
            generate_accessors: Some(false),
            ..Default::default()
        })
        .out_dir(&gen_dir)
        .input(proto_path)
        .include(gen_dir)
        .run_from_script();

    Ok(())
}
