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
    let url = schema_url(&file);
    let schema = reqwest::blocking::get(url)?.text()?;
    let mut file = fs::File::create(format!("{}/{}", dir, file))?;
    file.write_all(schema.as_bytes())?;

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = env::var("OUT_DIR").unwrap();
    let schema_dir = format!("{}/json_schema", out_dir);
    let gen_dir = format!("{}/generated", out_dir);

    if Path::new(&schema_dir).exists() {
        fs::remove_dir_all(&schema_dir).unwrap();
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

    //download_schema(&schema_dir, "DiagnosticResult.jsonschema")?;

    let mut code = String::new();

    for file in schema_files {
        download_schema(&schema_dir, &file)?;
        //let name = Path::new(&file).file_prefix().unwrap();
        let name = file.split('.').next().unwrap();

        code += &format!(
            r#"
pub mod {} {{
    use serde::{{ Serialize, Deserialize }};
    schemafy::schemafy!(root: {} "{}/{}");
}}"#,
            name,
            name.to_lowercase(),
            &schema_dir,
            file
        );
    }
    let mut file = fs::File::create(format!("{}/schema.rs", gen_dir))?;
    file.write_all(code.as_bytes())?;

    Ok(())
}
