use clap::Args;
use schemars::schema_for;
use serde_json::to_string;
use std::{fs, path};

#[derive(Args, Debug)]
pub struct GenerateJsonSchemaArgs {
    /// The path to output the json-schema to
    #[arg()]
    output_path: String,
}

pub fn run(args: GenerateJsonSchemaArgs) {
    let output_file =  path::Path::new(args.output_path.as_str());
    fs::create_dir_all(output_file.parent().unwrap()).unwrap();
    let schema = schema_for!(altc::util::LiveWrapper);
    fs::write(output_file, to_string(&schema).unwrap()).unwrap();
}