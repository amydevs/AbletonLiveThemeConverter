use serde::{de, Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug, Serialize, Deserialize)]
pub struct ValueWrapper<T> {
    #[serde(rename = "@Value")]
    pub value: T,
}

#[derive(Debug)]
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

#[derive(Debug, Serialize, Deserialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct RGBAColor {
    pub r: ValueWrapper<u8>,
    pub g: ValueWrapper<u8>,
    pub b: ValueWrapper<u8>,
    #[serde(rename = "Alpha")]
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

#[derive(Debug)]
pub struct HexColor {
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
        HexColor {
            value: color,
        }
    }
}

impl Into<Color> for HexColor {
    fn into(self) -> Color {
        self.value
    }
}

impl Serialize for HexColor
{
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
        Err(de::Error::custom(format!("String does not match the #rrggbbaa format")))
    }
}

#[cfg(test)]
mod tests {
    use quick_xml::de::from_str;

    use super::{ValueWrapper, RGBAColor, HexColor};
    #[test]
    fn value_wrapper() {
        let rgba_value: ValueWrapper<u8> = from_str(
            r#"
            <Alpha Value="90"/>
            "#,
        ).unwrap();
        assert_eq!(rgba_value.value, 90);
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
        ).unwrap();
        assert_eq!(rgba.r.value, 1);
        assert_eq!(rgba.g.value, 2);
        assert_eq!(rgba.b.value, 3);
        assert_eq!(rgba.a.value, 4);
    }
    #[test]
    fn hex_color() {
        let hex: HexColor = from_str(
            "<ControlForeground Value=\"#01020304\"/>",
        ).unwrap();
        assert_eq!(hex.value.r, 1);
        assert_eq!(hex.value.g, 2);
        assert_eq!(hex.value.b, 3);
        assert_eq!(hex.value.a, 4);
        let hex2: HexColor = from_str(
            "<ControlForeground Value=\"#010203\"/>",
        ).unwrap();
        assert_eq!(hex2.value.r, 1);
        assert_eq!(hex2.value.g, 2);
        assert_eq!(hex2.value.b, 3);
        assert_eq!(hex2.value.a, 255);
    }
}