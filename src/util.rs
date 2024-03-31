use std::io::BufRead;

use quick_xml::de::from_str;
use quick_xml::DeError;
use quick_xml::{de::from_reader, se::Serializer};
use regex::Regex;
use serde::Serialize;
use wasm_bindgen::prelude::*;

use crate::live10;
use crate::live11;
use crate::live12;

pub const LIVE_REGEX_STRING: &str = r#"MinorVersion\s*=\s*"(\S+)""#;

#[wasm_bindgen]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LiveVersion {
    Live10 = 10,
    Live11 = 11,
    Live12 = 12,
}

impl LiveVersion {
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            10 => Some(LiveVersion::Live10),
            11 => Some(LiveVersion::Live11),
            12 => Some(LiveVersion::Live12),
            _ => None,
        }
    }
    pub fn to_u8(&self) -> u8 {
        *self as u8
    }
}

#[wasm_bindgen()]
pub fn get_live_version(xml: &str) -> Option<LiveVersion> {
    let re = Regex::new(LIVE_REGEX_STRING).unwrap();
    let caps = re.captures(xml)?;
    let version = caps.get(1)?.as_str();
    match version.get(..3) {
        Some("10.") => Some(LiveVersion::Live10),
        Some("11.") => Some(LiveVersion::Live11),
        Some("12.") => Some(LiveVersion::Live12),
        _ => None,
    }
}

#[derive(Debug)]
pub enum LiveWrapper {
    Live10(live10::Ableton),
    Live11(live11::Ableton),
    Live12(live12::Ableton),
}

pub fn parse_ask(xml: &str, version: LiveVersion) -> Result<LiveWrapper, DeError> {
    match version {
        LiveVersion::Live10 => Ok(LiveWrapper::Live10(from_str(xml)?)),
        LiveVersion::Live11 => Ok(LiveWrapper::Live11(from_str(xml)?)),
        LiveVersion::Live12 => Ok(LiveWrapper::Live12(from_str(xml)?)),
    }
}

pub fn parse_ask_from_reader(
    reader: impl BufRead,
    version: LiveVersion,
) -> Result<LiveWrapper, DeError> {
    match version {
        LiveVersion::Live10 => Ok(LiveWrapper::Live10(from_reader(reader)?)),
        LiveVersion::Live11 => Ok(LiveWrapper::Live11(from_reader(reader)?)),
        LiveVersion::Live12 => Ok(LiveWrapper::Live12(from_reader(reader)?)),
    }
}

pub fn generate_ask(live: &LiveWrapper) -> Result<String, DeError> {
    let mut buffer = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n".to_string();
    let mut ser = Serializer::new(&mut buffer);
    ser.indent('\t', 1);
    match live {
        LiveWrapper::Live10(live10) => live10.serialize(ser)?,
        LiveWrapper::Live11(live11) => live11.serialize(ser)?,
        LiveWrapper::Live12(live12) => live12.serialize(ser)?,
    }
    Ok(buffer)
}

pub fn convert(from: LiveWrapper, to_version: LiveVersion) -> LiveWrapper {
    match to_version {
        LiveVersion::Live10 => match from {
            LiveWrapper::Live10(_) => from,
            LiveWrapper::Live11(live11) => LiveWrapper::Live10(live11.into()),
            LiveWrapper::Live12(live12) => {
                let live11: live11::Ableton = live12.into();
                LiveWrapper::Live10(live11.into())
            }
        },
        LiveVersion::Live11 => match from {
            LiveWrapper::Live10(live10) => LiveWrapper::Live11(live10.into()),
            LiveWrapper::Live11(_) => from,
            LiveWrapper::Live12(live12) => LiveWrapper::Live11(live12.into()),
        },
        LiveVersion::Live12 => match from {
            LiveWrapper::Live10(live10) => {
                let live11: live11::Ableton = live10.into();
                LiveWrapper::Live12(live11.into())
            }
            LiveWrapper::Live11(live11) => LiveWrapper::Live12(live11.into()),
            LiveWrapper::Live12(_) => from,
        },
    }
}

fn convert_ask_internal(
    xml: &str,
    from_version: LiveVersion,
    to_version: LiveVersion,
) -> Result<String, DeError> {
    let live = parse_ask(xml, from_version)?;
    let converted = convert(live, to_version);
    generate_ask(&converted)
}

#[cfg(not(target_arch = "wasm32"))]
pub fn convert_ask(
    xml: &str,
    from_version: LiveVersion,
    to_version: LiveVersion,
) -> Result<String, DeError> {
    convert_ask_internal(xml, from_version, to_version)
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn convert_ask(
    xml: &str,
    from_version: LiveVersion,
    to_version: LiveVersion,
) -> Result<String, JsError> {
    Ok(convert_ask_internal(xml, from_version, to_version)?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_live_version() {
        let mut xml = "MinorVersion=\"10.0\"";
        assert_eq!(super::get_live_version(xml), Some(LiveVersion::Live10));
        xml = "MinorVersion=\"11.0\"";
        assert_eq!(super::get_live_version(xml), Some(LiveVersion::Live11));
    }

    #[test]
    fn parse_generate_ask() {
        let xmls = [
            include_str!("../test_themes/blank_10.ask"),
            include_str!("../test_themes/blank_11.ask"),
            include_str!("../test_themes/blank_12.ask"),
        ];
        for xml in xmls {
            let version = super::get_live_version(xml).unwrap();
            let live = super::parse_ask(xml, version).unwrap();
            match live {
                LiveWrapper::Live10(_) => (),
                LiveWrapper::Live11(_) => (),
                LiveWrapper::Live12(_) => (),
            }
        }
    }
}
