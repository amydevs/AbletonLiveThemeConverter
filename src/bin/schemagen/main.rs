extern crate proc_macro;

use std::collections::HashSet;
use std::fs;

use clap::Parser;
use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use quick_xml::events::Event;
use quick_xml::reader::Reader;
use quote::quote;
use syn::{parse_macro_input, parse_quote, Field, Fields};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The path to the .ask theme file
    #[arg()]
    input_path: String,

    #[arg()]
    output_path: String,
}

fn main() {
    let args = Args::parse();

    let file = fs::read_to_string(&args.input_path).unwrap();
    let mut reader = Reader::from_str(&file);
    reader.trim_text(true);

    let mut buf = Vec::new();

    let mut is_subject = false;

    let mut classes: HashSet<String> = HashSet::new();

    // The `Reader` does not implement `Iterator` because it outputs borrowed data (`Cow`s)
    loop {
        // NOTE: this is the generic case when we don't know about the input BufRead.
        // when the input is a &str or a &[u8], we don't actually need to use another
        // buffer, we could directly call `reader.read_event()`
        match reader.read_event_into(&mut buf) {
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            // exits the loop when reaching end of file
            Ok(Event::Eof) => break,

            Ok(Event::Start(e)) => {
                if e.name().as_ref() == b"Theme" || e.name().as_ref() == b"SkinManager" {
                    let name = std::str::from_utf8(e.name().0).unwrap();
                    is_subject = true;
                }
            }
            Ok(Event::End(e)) => {
                if e.name().as_ref() == b"Theme" || e.name().as_ref() == b"SkinManager" {
                    is_subject = false;
                    break;
                }
            }
            Ok(Event::Empty(e)) => {
                if is_subject {
                    let name = std::str::from_utf8(e.name().0).unwrap();
                    // println!("{}", name);
                    for attribute_wrapped in e.attributes() {
                        let attribute = attribute_wrapped.unwrap();
                        if attribute.key.as_ref() == b"Class" {
                            let value = std::str::from_utf8(attribute.value.as_ref()).unwrap();
                            classes.insert(value.to_owned());
                            if let Some(class_type) = class_to_type(attribute.value.as_ref()) {
                                let converted_name: String = name.to_case(Case::Snake);
                                let converted_name_tok: TokenStream =
                                    converted_name.parse().unwrap();
                                let class_type_tok: TokenStream = class_type.parse().unwrap();
                                let field_tok: TokenStream = quote!(
                                    pub #converted_name_tok: Option<#class_type_tok>,
                                );
                                println!("{}", field_tok);
                            }
                        }
                    }
                }
            }
            // There are several other `Event`s we do not consider here
            _ => (),
        }
        // if we don't keep a borrow elsewhere, we can clear the buffer to keep memory usage low
        // buf.clear();
    }
    for class in classes {
        if class_to_type(class.as_bytes()).is_none() {
            eprintln!("{} is not mapped to a type!", class);
        }
    }
}

fn class_to_type(class: &[u8]) -> Option<String> {
    match class {
        b"RemoteableInt" => Some("ValueWrapper<i32>".to_string()),
        b"RemoteableColor" => Some("HexColor".to_string()),
        b"UserFloat" => Some("ValueWrapper<f32>".to_string()),
        b"RemoteableDouble" => Some("ValueWrapper<f64>".to_string()),
        b"RemoteableFloat" => Some("ValueWrapper<f32>".to_string()),
        b"VuMeterColors" => Some("Meter".to_string()),
        _ => None,
    }
}
