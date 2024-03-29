use altc::util;
use clap::Parser;
use std::{fs, io::Write, path};

pub fn parse_live_version(s: &str) -> Result<util::LiveVersion, String> {
    match s.parse::<u8>() {
        Ok(n) => match n {
            10 => Ok(util::LiveVersion::Live10),
            11 => Ok(util::LiveVersion::Live11),
            _ => Err(format!("{} is not a valid version number.", s)),
        },
        _ => Err(format!("{} is not a valid version number.", s)),
    }
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The path to the .ask theme file
    #[arg()]
    ask_path: String,
    /// The path to the .ask theme file
    #[arg()]
    output_path: Option<String>,
    /// The version of Live to that the theme file supports
    #[arg(short, long, value_parser=parse_live_version)]
    from_version: Option<util::LiveVersion>,
    /// The version of Live to convert the theme to
    #[arg(short, long, value_parser=parse_live_version)]
    to_version: util::LiveVersion,
}

fn main() {
    let args = Args::parse();
    let ask_file = match fs::read_to_string(&args.ask_path) {
        Ok(file) => file,
        Err(_) => {
            eprintln!("Could not read .ask theme file: {}", &args.ask_path);
            return;
        }
    };
    let from_version = match args.from_version {
        Some(version) => version,
        None => match util::get_live_version(&ask_file) {
            Some(version) => version,
            None => {
                eprintln!(
                    "Could not read detect Ableton Live version from .ask theme file: {}",
                    &args.ask_path
                );
                return;
            }
        },
    };
    eprintln!("Detected Ableton Live version: {}", from_version as u8);

    let parsed_ask = match util::parse_ask(&ask_file, from_version) {
        Ok(ask) => ask,
        Err(err) => {
            eprintln!("Could not parse .ask theme file: {}", &args.ask_path);
            eprintln!("Error: {}", err.to_string());
            return;
        }
    };
    let converted_ask = util::convert(parsed_ask, args.to_version);
    let generated_ask = match util::generate_ask(&converted_ask) {
        Ok(ask) => ask,
        Err(_) => {
            eprintln!("Could not generate .ask theme file: {}", &args.ask_path);
            return;
        }
    };

    let output_path = args.output_path.unwrap_or_else(|| {
        let path = path::Path::new(&args.ask_path);
        let mut output = String::new();
        if let Some(parent) = path.parent() {
            output.push_str(parent.to_str().unwrap());
            output.push_str("/");
        }
        // this should never fail, because it must be a file
        output += &format!(
            "{}.converted.ask",
            path.file_stem().unwrap().to_str().unwrap()
        );
        output
    });
    eprintln!("Writing converted .ask theme file to: {}", &output_path);

    let write_file_result = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .append(false)
        .open(&output_path)
        .and_then(|mut file| file.write_all(generated_ask.as_bytes()));

    if write_file_result.is_err() {
        eprintln!(
            "Could not write converted .ask theme file: {}",
            &output_path
        );
    }
}
