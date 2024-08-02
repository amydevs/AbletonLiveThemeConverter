use std::{env, fs, path};

use schemars::schema_for;
use serde_json::to_string;

fn main() {
    let args: Vec<String> = env::args().collect();
    let command = args[1].as_str();
    if command == "generate-json-schema" {
        let output_file =  path::Path::new(args[2].as_str());
        fs::create_dir_all(output_file.parent().unwrap()).unwrap();
        let schema = schema_for!(altc::util::LiveWrapper);
        fs::write(output_file, to_string(&schema).unwrap()).unwrap();
    }
}
