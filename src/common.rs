use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ValueWrapper<T> {
    #[serde(rename = "@Value")]
    pub value: T,
}

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

#[derive(Debug, Serialize, Deserialize)]
pub struct HexColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
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
            r: color.r,
            g: color.g,
            b: color.b,
            a: color.a,
        }
    }
}

impl Into<Color> for HexColor {
    fn into(self) -> Color {
        Color {
            r: self.r,
            g: self.g,
            b: self.b,
            a: self.a,
        }
    }
}