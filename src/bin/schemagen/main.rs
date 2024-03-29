use std::fs;
use std::io::Read;

use clap::Parser;
use quick_xml::events::Event;
use quick_xml::reader::Reader;

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
                let name = std::str::from_utf8(e.name().0).unwrap();
                if name == "Theme" || name == "SkinManager" {
                    is_subject = true;
                }
            }
            Ok(Event::End(e)) => {
                let name = std::str::from_utf8(e.name().0).unwrap();
                if name == "Theme" || name == "SkinManager" {
                    is_subject = false;
                    break;
                }
            }
            Ok(Event::Empty(e)) => {
                if is_subject {
                    let name = std::str::from_utf8(e.name().0).unwrap();
                    println!("{}", name);
                    for attribute_wrapped in e.attributes() {
                        let attribute = attribute_wrapped.unwrap();
                        let key = std::str::from_utf8(attribute.key.0).unwrap();
                        let value = std::str::from_utf8(attribute.value.as_ref()).unwrap();
                        println!("{}: {}", key, value);
                    }
    
                }
            }
            // There are several other `Event`s we do not consider here
            _ => (),
        }
        // if we don't keep a borrow elsewhere, we can clear the buffer to keep memory usage low
        buf.clear();
    }
}