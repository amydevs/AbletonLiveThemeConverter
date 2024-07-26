use std::fs;
use std::io::Write;

use clap::Parser;
use convert_case::{Case, Casing};
use proc_macro2::*;
use quick_xml::events::Event;
use quick_xml::reader::Reader;
use quote::*;
use regex::Regex;
use syn::*;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The path to the .ask theme file
    #[arg()]
    input_path: String,

    #[arg()]
    output_path: String,
}

fn main() -> core::result::Result<(), String> {
    let args = Args::parse();

    let schema = fs::read_to_string(&args.input_path).unwrap();

    let theme_class_regex = Regex::new(r"<(SkinManager|Theme)>").unwrap();

    let theme_class_string = theme_class_regex
        .captures(&schema)
        .and_then(|captures| captures.get(1).and_then(|capture| Some(capture.as_str())))
        .ok_or("Could not find SkinManager or Theme".to_string())?;

    let mut fields: Vec<Field> = vec![];

    let mut reader = Reader::from_str(&schema);
    reader.trim_text(true);
    let mut buf = Vec::new();
    let mut is_subject = false;

    let color_type = match theme_class_string {
        "SkinManager" => "RGBAColor",
        "Theme" => "HexColor",
        _ => panic!(),
    };

    let class_to_type = |class: &[u8]| match class {
        b"RemoteableInt" => Some("ValueWrapper<i32>".to_string()),
        b"RemoteableColor" => Some(color_type.to_string()),
        b"UserFloat" => Some("ValueWrapper<f32>".to_string()),
        b"RemoteableDouble" => Some("ValueWrapper<f64>".to_string()),
        b"RemoteableFloat" => Some("ValueWrapper<f32>".to_string()),
        b"VuMeterColors" => Some("Meter".to_string()),
        _ => None,
    };

    loop {
        // NOTE: this is the generic case when we don't know about the input BufRead.
        // when the input is a &str or a &[u8], we don't actually need to use another
        // buffer, we could directly call `reader.read_event()`
        match reader.read_event_into(&mut buf) {
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            // exits the loop when reaching end of file
            Ok(Event::Eof) => break,

            Ok(Event::Start(e)) => {
                if e.name().as_ref() == theme_class_string.as_bytes() {
                    is_subject = true;
                }
            }
            Ok(Event::End(e)) => {
                if e.name().as_ref() == theme_class_string.as_bytes() {
                    break;
                }
            }
            Ok(Event::Empty(e)) => {
                if is_subject {
                    let name = std::str::from_utf8(e.name().0).unwrap();
                    for attribute_wrapped in e.attributes() {
                        let attribute = attribute_wrapped.unwrap();
                        if attribute.key.as_ref() == b"Class" {
                            if let Some(class_type) = class_to_type(attribute.value.as_ref()) {
                                let converted_name: String = name.to_case(Case::Snake);
                                let converted_name_tok: TokenStream =
                                    converted_name.parse().unwrap();
                                let class_type_tok: TokenStream = class_type.parse().unwrap();
                                let field_tok: TokenStream = quote! {
                                    pub #converted_name_tok: Option<#class_type_tok>
                                };
                                let field = syn::parse::Parser::parse2(
                                    |input: parse::ParseStream<'_>| {
                                        Ok({
                                            let mut field: Field =
                                                Field::parse_named(input).unwrap();
                                            if name.contains("LCD") {
                                                let attr: Attribute =
                                                    parse_quote!(#[serde(rename = #name)]);
                                                field.attrs.push(attr);
                                            }
                                            field
                                        })
                                    },
                                    field_tok,
                                )
                                .unwrap();
                                fields.push(field);
                            } else {
                                eprintln!(
                                    "<{} /> has unknown class: {}",
                                    name,
                                    std::str::from_utf8(attribute.value.as_ref()).unwrap()
                                );
                            }
                        }
                    }
                }
            }
            _ => (),
        }
    }

    let theme_class_token_stream: TokenStream = theme_class_string.parse().unwrap();
    let theme_class_snake_token_stream: TokenStream =
        theme_class_string.to_case(Case::Snake).parse().unwrap();

    let theme_struct_tokenstream = quote! {
        use crate::common::{HexColor, Meter, ValueWrapper};
        use serde::{Deserialize, Serialize};
        use serde_with::skip_serializing_none;
        use tsify::Tsify;

        #[skip_serializing_none]
        #[derive(Debug, Serialize, Deserialize, Tsify)]
        #[serde(rename_all = "PascalCase")]
        pub struct #theme_class_token_stream {
            #( #fields ),*
        }

        #[skip_serializing_none]
        #[derive(Debug, Serialize, Deserialize, Tsify)]
        pub struct Ableton {
            #[serde(rename = "@MajorVersion")]
            pub major_version: Option<String>,
            #[serde(rename = "@MinorVersion")]
            pub minor_version: Option<String>,
            #[serde(rename = "@SchemaChangeCount")]
            pub schema_change_count: Option<String>,
            #[serde(rename = "@Creator")]
            pub creator: Option<String>,
            #[serde(rename = "@Revision")]
            pub revision: Option<String>,
            #[serde(rename = #theme_class_string)]
            pub #theme_class_snake_token_stream: Option<#theme_class_token_stream>,
        }
    };

    let file = syn::parse_file(&theme_struct_tokenstream.to_string()).unwrap();
    let prettied_file = prettyplease::unparse(&file);

    fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .append(false)
        .open(&args.output_path)
        .and_then(|mut file| file.write_all(prettied_file.as_bytes()))
        .map_err(|_| "Could not write to source file.".to_string())?;
    Ok(())
}
