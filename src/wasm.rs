use crate::util::{
    parse_ask as parse_ask_internal,
    convert_ask as convert_ask_internal,
    LiveVersion,
    LiveWrapper,
};
use wasm_bindgen::prelude::*;


#[wasm_bindgen]
pub fn parse_ask(xml: &str, version: LiveVersion) -> Result<LiveWrapper, JsError> {
    Ok(parse_ask_internal(xml, version)?)
}

#[wasm_bindgen]
pub fn convert_ask(
    xml: &str,
    from_version: LiveVersion,
    to_version: LiveVersion,
) -> Result<String, JsError> {
    Ok(convert_ask_internal(xml, from_version, to_version)?)
}