use std::env;
use std::fs;
use std::io::Write;
use std::path::Path;

fn schema_url(file: &str) -> String {
    let repository = "reviewdog/reviewdog";
    let tag = "v0.13.1";
    let dir = "proto/rdf/jsonschema";
    format!(
        "https://raw.githubusercontent.com/{}/{}/{}/{}",
        repository, tag, dir, file
    )
}

fn download_schema(dir: &str, file: &str) -> Result<(), Box<dyn std::error::Error>> {
    let url = schema_url(file);

    let file = format!("{}/{}", dir, file);
    let file = Path::new(&file);

    // skip download if file already exists
    if !file.exists() {
        let schema = reqwest::blocking::get(url)?.text()?;
        let mut file = fs::File::create(file)?;
        file.write_all(schema.as_bytes())?;
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = env::var("OUT_DIR").unwrap();

    let local_schema = env::var("CARGO_FEATURE_BUILD_WITH_LOCAL_SCHEMA");
    let schema_dir = if local_schema.is_ok() {
        "json_schema".to_string()
    } else {
        format!("{}/json_schema", out_dir)
    };

    let gen_dir = format!("{}/generated", out_dir);

    if Path::new(&schema_dir).exists() {
        // skip remvoe schema directory on docs.rs
        if local_schema.is_err() {
            fs::remove_dir_all(&schema_dir).unwrap();
        }
    }
    fs::create_dir(&schema_dir).unwrap();
    if Path::new(&gen_dir).exists() {
        fs::remove_dir_all(&gen_dir).unwrap();
    }
    fs::create_dir(&gen_dir).unwrap();

    let schema_files = vec![
        //"Code.jsonschema",
        "Diagnostic.jsonschema",
        "DiagnosticResult.jsonschema",
        //"Location.jsonschema",
        //"Position.jsonschema",
        //"Range.jsonschema",
        //"Source.jsonschema",
        //"Suggestion.jsonschema",
    ];

    let mut code = String::new();

    for file in schema_files {
        download_schema(&schema_dir, file)?;
        //let name = Path::new(&file).file_prefix().unwrap();
        let name = file.split('.').next().unwrap();

        code += &format!(
            r#"
pub mod {} {{
    use serde::{{ Serialize, Deserialize }};
    schemafy::schemafy!(root: {} "{}/{}");
}}"#,
            name.to_lowercase(),
            name,
            &schema_dir,
            file
        );
    }
    let mut file = fs::File::create(format!("{}/schema.rs", gen_dir))?;
    file.write_all(code.as_bytes())?;

    Ok(())
}
