use regex::Regex;
use quick_xml::{de::from_str, se::to_string, DeError};


use crate::live11;
use crate::live10;


pub const LIVE_REGEX_STRING: &str = r#"MinorVersion\s*=\s*"(\S+)""#;

#[derive(Debug, PartialEq)]
pub enum LiveVersion {
    Live10,
    Live11,
}

pub fn get_live_version(xml: &str) -> Option<LiveVersion> {
    let re = Regex::new(LIVE_REGEX_STRING).unwrap();
    let caps = re.captures(xml).unwrap();
    let version = caps.get(1).unwrap().as_str();
    match version.get(..3) {
        Some("10.") => Some(LiveVersion::Live10),
        Some("11.") => Some(LiveVersion::Live11),
        _ => None,
    }
}

#[derive(Debug)]
pub enum LiveWrapper {
    Live10(live10::Ableton),
    Live11(live11::Ableton),
}

pub fn parse_ask(xml: &str, version: &LiveVersion) -> Result<LiveWrapper, DeError> {
    match version {
        LiveVersion::Live10 => Ok(LiveWrapper::Live10(from_str(xml)?)),
        LiveVersion::Live11 => Ok(LiveWrapper::Live11(from_str(xml)?)),
    }
}

pub fn generate_ask(live: &LiveWrapper) -> Result<String, DeError> {
    match live {
        LiveWrapper::Live10(live10) => to_string(live10),
        LiveWrapper::Live11(live11) => to_string(live11),
    }
}

pub fn convert(live: LiveWrapper, version: &LiveVersion) -> Result<LiveWrapper, DeError> {
    match version {
        LiveVersion::Live10 => match live {
            LiveWrapper::Live11(live11) => Ok(LiveWrapper::Live10(live11.into())),
            LiveWrapper::Live10(_) => Ok(live),
        },
        LiveVersion::Live11 => match live {
            LiveWrapper::Live10(live10) => Ok(LiveWrapper::Live11(live10.into())),
            LiveWrapper::Live11(_) => Ok(live),
        },
    }
}

pub fn convert_ask(xml: &str, version: &LiveVersion) -> Result<String, DeError> {
    let live = parse_ask(xml, version)?;
    let converted = convert(live, version)?;
    generate_ask(&converted)
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
        ];
        for xml in xmls {
            let version = super::get_live_version(xml).unwrap();
            let live = super::parse_ask(xml, &version).unwrap();
            match live {
                LiveWrapper::Live10(_) => (),
                LiveWrapper::Live11(_) => (),
            }
        }
    }
}