use serde::{
    de::{self},
    Deserialize, Deserializer, Serialize, Serializer,
};
use serde_with::skip_serializing_none;
use tsify::Tsify;
use wasm_bindgen::prelude::*;



#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct ValueWrapper<T> {
    #[serde(rename = "@Value")]
    pub value: T,
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "ValueWrapper")]
    pub type ValueWrapperJsType;
}
impl<T> Tsify for ValueWrapper<T> {
    type JsType = ValueWrapperJsType;
    const DECL: &'static str = "export interface ValueWrapper<T> {\n    \"@Value\": T;\n}";
}
#[wasm_bindgen(typescript_custom_section)]
const TS_APPEND_CONTENT: &'static str = "export interface ValueWrapper<T> {\n    \"@Value\": T;\n}";

#[derive(Clone, Copy, Debug, Tsify)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Default for Color {
    fn default() -> Self {
        Color {
            r: 0,
            g: 0,
            b: 0,
            a: 255,
        }
    }
}

#[skip_serializing_none]
#[derive(Clone, Copy, Debug, Serialize, Deserialize, Tsify)]
#[serde(default, rename_all = "PascalCase")]
pub struct RGBAColor {
    #[serde(deserialize_with = "deserialize_coerce_u8")]
    pub r: ValueWrapper<u8>,
    #[serde(deserialize_with = "deserialize_coerce_u8")]
    pub g: ValueWrapper<u8>,
    #[serde(deserialize_with = "deserialize_coerce_u8")]
    pub b: ValueWrapper<u8>,
    #[serde(rename = "Alpha", deserialize_with = "deserialize_coerce_u8")]
    pub a: ValueWrapper<u8>,
}

impl Default for RGBAColor {
    fn default() -> Self {
        let default_color = Color::default();
        default_color.into()
    }
}

impl From<Color> for RGBAColor {
    fn from(color: Color) -> RGBAColor {
        RGBAColor {
            r: ValueWrapper { value: color.r },
            g: ValueWrapper { value: color.g },
            b: ValueWrapper { value: color.b },
            a: ValueWrapper { value: color.a },
        }
    }
}

impl Into<Color> for RGBAColor {
    fn into(self) -> Color {
        Color {
            r: self.r.value,
            g: self.g.value,
            b: self.b.value,
            a: self.a.value,
        }
    }
}

fn deserialize_coerce_u8<'de, D>(deserializer: D) -> Result<ValueWrapper<u8>, D::Error>
where
    D: Deserializer<'de>,
{
    let string_val: ValueWrapper<String> = ValueWrapper::deserialize(deserializer)?;
    if let Ok(val) = string_val.value.parse() {
        return Ok(ValueWrapper { value: val });
    }
    if let Ok(val) = string_val.value.parse::<f64>() {
        return Ok(ValueWrapper { value: val as u8 });
    }
    Err(de::Error::custom(
        "could not parse or coerce RGBA color string to integer",
    ))
}

#[skip_serializing_none]
#[derive(Clone, Copy, Debug, Tsify)]
pub struct HexColor {
    #[tsify(type = "string")]
    pub value: Color,
}

impl Default for HexColor {
    fn default() -> Self {
        let default_color = Color::default();
        default_color.into()
    }
}

impl From<Color> for HexColor {
    fn from(color: Color) -> HexColor {
        HexColor { value: color }
    }
}

impl Into<Color> for HexColor {
    fn into(self) -> Color {
        self.value
    }
}

impl Serialize for HexColor {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        #[derive(Serialize)]
        struct Helper {
            #[serde(rename = "@Value")]
            value: String,
        }
        let mut hex = format!(
            "#{:02x}{:02x}{:02x}",
            self.value.r, self.value.g, self.value.b,
        );
        if self.value.a != 255 {
            hex.push_str(&format!("{:02x}", self.value.a));
        }
        let hex_helper = Helper { value: hex };
        hex_helper.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HexColor {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Helper {
            #[serde(rename = "@Value")]
            value: String,
        }

        let helper = Helper::deserialize(deserializer)?;

        // Parse the hex string
        let hex = helper.value.trim_start_matches('#');

        let mut color = Color::default();
        if hex.len() >= 6 {
            color.r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(color.r);
            color.g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(color.g);
            color.b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(color.b);
            if hex.len() >= 8 {
                color.a = u8::from_str_radix(&hex[6..8], 16).unwrap_or(color.a);
            }
            return Ok(HexColor { value: color });
        }
        Err(de::Error::custom(format!(
            "string does not match the #rrggbbaa format"
        )))
    }
}

impl Into<HexColor> for RGBAColor {
    fn into(self) -> HexColor {
        let color: Color = self.into();
        color.into()
    }
}

impl Into<RGBAColor> for HexColor {
    fn into(self) -> RGBAColor {
        let color: Color = self.into();
        color.into()
    }
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Tsify)]
#[serde(rename_all = "PascalCase")]
pub struct Meter {
    pub only_minimum_to_maximum: Option<ValueWrapper<bool>>,
    pub maximum: Option<HexColor>,
    pub above_zero_decibel: Option<HexColor>,
    pub zero_decibel: Option<HexColor>,
    /// This is always none is only_minimum_to_maximum is true.
    pub below_zero_decibel1: Option<HexColor>,
    /// This is always none is only_minimum_to_maximum is true.
    pub below_zero_decibel2: Option<HexColor>,
    pub minimum: Option<HexColor>,
}

impl Meter {
    pub fn default_standard_vu_meter() -> Self {
        Self {
            only_minimum_to_maximum: Some(ValueWrapper { value: false }),
            maximum: Some(HexColor {
                value: Color {
                    r: 255,
                    g: 10,
                    b: 10,
                    a: 255,
                },
            }),
            above_zero_decibel: Some(HexColor {
                value: Color {
                    r: 255,
                    g: 208,
                    b: 10,
                    a: 255,
                },
            }),
            zero_decibel: Some(HexColor {
                value: Color {
                    r: 198,
                    g: 248,
                    b: 100,
                    a: 255,
                },
            }),
            below_zero_decibel1: None,
            below_zero_decibel2: None,
            minimum: Some(HexColor {
                value: Color {
                    r: 10,
                    g: 248,
                    b: 100,
                    a: 255,
                },
            }),
        }
    }
    pub fn default_overlad_vu_meter() -> Self {
        Self {
            only_minimum_to_maximum: Some(ValueWrapper { value: true }),
            maximum: Some(HexColor {
                value: Color {
                    r: 255,
                    g: 10,
                    b: 10,
                    a: 255,
                },
            }),
            above_zero_decibel: Some(HexColor {
                value: Color {
                    r: 255,
                    g: 255,
                    b: 255,
                    a: 255,
                },
            }),
            zero_decibel: Some(HexColor {
                value: Color {
                    r: 255,
                    g: 255,
                    b: 255,
                    a: 255,
                },
            }),
            below_zero_decibel1: None,
            below_zero_decibel2: None,
            minimum: Some(HexColor {
                value: Color {
                    r: 175,
                    g: 10,
                    b: 10,
                    a: 255,
                },
            }),
        }
    }
    pub fn default_disabled_vu_meter() -> Self {
        Self {
            only_minimum_to_maximum: Some(ValueWrapper { value: false }),
            maximum: Some(HexColor {
                value: Color {
                    r: 255,
                    g: 10,
                    b: 10,
                    a: 255,
                },
            }),
            above_zero_decibel: Some(HexColor {
                value: Color {
                    r: 255,
                    g: 208,
                    b: 10,
                    a: 255,
                },
            }),
            zero_decibel: Some(HexColor {
                value: Color {
                    r: 130,
                    g: 130,
                    b: 130,
                    a: 255,
                },
            }),
            below_zero_decibel1: None,
            below_zero_decibel2: None,
            minimum: Some(HexColor {
                value: Color {
                    r: 110,
                    g: 110,
                    b: 110,
                    a: 255,
                },
            }),
        }
    }
    pub fn default_headphones_vu_meter() -> Self {
        Self {
            only_minimum_to_maximum: Some(ValueWrapper { value: false }),
            maximum: Some(HexColor {
                value: Color {
                    r: 165,
                    g: 165,
                    b: 241,
                    a: 255,
                },
            }),
            above_zero_decibel: Some(HexColor {
                value: Color {
                    r: 144,
                    g: 170,
                    b: 236,
                    a: 255,
                },
            }),
            zero_decibel: Some(HexColor {
                value: Color {
                    r: 144,
                    g: 170,
                    b: 236,
                    a: 255,
                },
            }),
            below_zero_decibel1: None,
            below_zero_decibel2: None,
            minimum: Some(HexColor {
                value: Color {
                    r: 10,
                    g: 255,
                    b: 255,
                    a: 255,
                },
            }),
        }
    }
    pub fn default_sends_only_vu_meter() -> Self {
        Self {
            only_minimum_to_maximum: Some(ValueWrapper { value: false }),
            maximum: Some(HexColor {
                value: Color {
                    r: 200,
                    g: 200,
                    b: 0,
                    a: 255,
                },
            }),
            above_zero_decibel: Some(HexColor {
                value: Color {
                    r: 200,
                    g: 200,
                    b: 0,
                    a: 255,
                },
            }),
            zero_decibel: Some(HexColor {
                value: Color {
                    r: 100,
                    g: 100,
                    b: 255,
                    a: 255,
                },
            }),
            below_zero_decibel1: None,
            below_zero_decibel2: None,
            minimum: Some(HexColor {
                value: Color {
                    r: 100,
                    g: 100,
                    b: 255,
                    a: 255,
                },
            }),
        }
    }
    pub fn default_bipolar_gain_reduction_vu_meter() -> Self {
        Self {
            only_minimum_to_maximum: Some(ValueWrapper { value: false }),
            maximum: Some(HexColor {
                value: Color {
                    r: 85,
                    g: 119,
                    b: 198,
                    a: 255,
                },
            }),
            above_zero_decibel: Some(HexColor {
                value: Color {
                    r: 85,
                    g: 119,
                    b: 198,
                    a: 255,
                },
            }),
            zero_decibel: Some(HexColor {
                value: Color {
                    r: 255,
                    g: 165,
                    b: 25,
                    a: 255,
                },
            }),
            below_zero_decibel1: None,
            below_zero_decibel2: None,
            minimum: Some(HexColor {
                value: Color {
                    r: 255,
                    g: 165,
                    b: 25,
                    a: 255,
                },
            }),
        }
    }
    pub fn default_orange_vu_meter() -> Self {
        Self {
            only_minimum_to_maximum: Some(ValueWrapper { value: true }),
            maximum: Some(HexColor {
                value: Color {
                    r: 255,
                    g: 165,
                    b: 25,
                    a: 255,
                },
            }),
            above_zero_decibel: Some(HexColor {
                value: Color {
                    r: 255,
                    g: 165,
                    b: 25,
                    a: 255,
                },
            }),
            zero_decibel: Some(HexColor {
                value: Color {
                    r: 255,
                    g: 165,
                    b: 25,
                    a: 255,
                },
            }),
            below_zero_decibel1: None,
            below_zero_decibel2: None,
            minimum: Some(HexColor {
                value: Color {
                    r: 255,
                    g: 165,
                    b: 25,
                    a: 255,
                },
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use quick_xml::de::from_str;

    use super::{HexColor, RGBAColor, ValueWrapper};
    #[test]
    fn value_wrapper() {
        let rgba_value: ValueWrapper<u8> = from_str(
            r#"
            <Alpha Value="90"/>
            "#,
        )
        .unwrap();
        assert_eq!(rgba_value.value, 90);
        let rgba_error: Result<ValueWrapper<u8>, quick_xml::DeError> = from_str(
            r#"
            <Alpha Value="90.0"/>
            "#,
        );
        println!("{:?}", rgba_error);
        assert!(matches!(rgba_error, Err(quick_xml::DeError::InvalidInt(_))));
    }
    #[test]
    fn rgba() {
        let rgba: RGBAColor = from_str(
            r#"
            <ControlForeground>
                <R Value="1"/>
                <G Value="2"/>
                <B Value="3"/>
                <Alpha Value="4"/>
            </ControlForeground>
            "#,
        )
        .unwrap();
        assert_eq!(rgba.r.value, 1);
        assert_eq!(rgba.g.value, 2);
        assert_eq!(rgba.b.value, 3);
        assert_eq!(rgba.a.value, 4);
        let rgba_floats: RGBAColor = from_str(
            r#"
            <ControlForeground>
                <R Value="1.0"/>
                <G Value="-0.1"/>
                <B Value="255.1"/>
                <Alpha Value="256"/>
            </ControlForeground>
            "#,
        )
        .unwrap();
        assert_eq!(rgba_floats.r.value, 1);
        assert_eq!(rgba_floats.g.value, 0);
        assert_eq!(rgba_floats.b.value, 255);
        assert_eq!(rgba_floats.a.value, 255);
        let rgba_error: Result<RGBAColor, quick_xml::DeError> = from_str(
            r#"
            <ControlForeground>
                <R Value="value"/>
            </ControlForeground>
            "#,
        );
        assert!(matches!(rgba_error, Err(quick_xml::DeError::Custom(_))));
    }
    #[test]
    fn hex_color() {
        let hex: HexColor = from_str("<ControlForeground Value=\"#01020304\"/>").unwrap();
        assert_eq!(hex.value.r, 1);
        assert_eq!(hex.value.g, 2);
        assert_eq!(hex.value.b, 3);
        assert_eq!(hex.value.a, 4);
        let hex2: HexColor = from_str("<ControlForeground Value=\"#010203\"/>").unwrap();
        assert_eq!(hex2.value.r, 1);
        assert_eq!(hex2.value.g, 2);
        assert_eq!(hex2.value.b, 3);
        assert_eq!(hex2.value.a, 255);
        let hex_error: Result<HexColor, quick_xml::DeError> =
            from_str("<ControlForeground Value=\"value\"/>");
        assert!(matches!(hex_error, Err(quick_xml::DeError::Custom(_))));
    }
}
