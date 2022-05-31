#[derive(Debug, PartialEq)]
pub enum ColorError {
  InvalidHexChar,
  ParseErr,
}

pub type ColorResult<T> = Result<T, ColorError>;

#[derive(Debug, PartialEq)]
pub struct Color{
  r: u8,
  g: u8,
  b: u8,
  a: u8,
}


impl Color {
  pub fn rgb(r: u8, g: u8, b: u8) -> ColorResult<Self> {
    Ok(Self {r, g, b, a: 255 })
  }
  pub fn rgba(r: u8, g: u8, b:u8, a: f32) -> ColorResult<Self> {
    let a: u8 = if a < 0.0 { 0 } else if a > 1.0 { 255 } else { (a * 255.0).round() as u8 };
    Ok(Self { r, g, b, a })
  }
  pub fn hex(color: &str) -> ColorResult<Self> {
    let color = color.trim();
    Self::validate_hex_str(color)?;
    let len = color.len();

    match len {
      4 => Self::hex_to_rgb_1(color),
      5 => Self::hex_to_rgba_1(color),
      7 => Self::hex_to_rgb_2(color),
      9 => Self::hex_to_rgba_2(color),
      _ => Err(ColorError::InvalidHexChar)
    }
  }

  pub fn to_hex(&self) -> String {
    format!("#{:02x}{:02x}{:02x}{:02x}", self.r, self.g, self.b, self.a)
  }
  pub fn to_rgb(&self) -> String {
    if self.a == 255 {
      format!("rgb({}, {}, {})", self.r, self.g, self.b)
    } else {
      format!("rgba({}, {}, {}, {})", self.r, self.g, self.b, self.a)
    }
  }
}

impl Color {
  fn validate_hex_str(hex: &str) -> ColorResult<()> {
    match &hex.len() {
      4 | 5 | 7 | 9 => {},
      _ => return Err(ColorError::InvalidHexChar),
    }

    if &hex[0..1] != "#" {
      return Err(ColorError::InvalidHexChar);
    }

    Ok(())
  }

  fn from_hex(hex: &str) -> ColorResult<u8> {
    match u8::from_str_radix(hex, 16) {
      Ok(r) => Ok(r),
      _ => Err(ColorError::ParseErr),
    }
  }

  fn hex_to_rgb_1(hex: &str) -> ColorResult<Self> {
    let r = Self::from_hex(format!("{}{}", &hex[1..2], &hex[1..2]).as_str())?;
    let g = Self::from_hex(format!("{}{}", &hex[2..3], &hex[2..3]).as_str())?;
    let b = Self::from_hex(format!("{}{}", &hex[3..4], &hex[3..4]).as_str())?;
    Ok(Self {r, g, b, a: 255})
  }
  fn hex_to_rgba_1(hex: &str) -> ColorResult<Self> {
    let r = Self::from_hex(format!("{}{}", &hex[1..2], &hex[1..2]).as_str())?;
    let g = Self::from_hex(format!("{}{}", &hex[2..3], &hex[2..3]).as_str())?;
    let b = Self::from_hex(format!("{}{}", &hex[3..4], &hex[3..4]).as_str())?;
    let a = Self::from_hex(format!("{}{}", &hex[4..5], &hex[4..5]).as_str())?;
    Ok(Self {r, g, b, a})
  }
  fn hex_to_rgb_2(hex: &str) -> ColorResult<Self> {
    let r = Self::from_hex(&hex[1..3])?;
    let g = Self::from_hex(&hex[3..5])?;
    let b = Self::from_hex(&hex[5..7])?;
    Ok(Self {r, g, b, a: 255})
  }
  fn hex_to_rgba_2(hex: &str) -> ColorResult<Self> {
    let r = Self::from_hex(&hex[1..3])?;
    let g = Self::from_hex(&hex[3..5])?;
    let b = Self::from_hex(&hex[5..7])?;
    let a = Self::from_hex(&hex[7..9])?;
    Ok(Self {r, g, b, a})
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn rgb_test() {
    let color = Color::rgb(255, 255, 255).unwrap();
    assert_eq!(Color{r: 255, g: 255, b: 255, a: 255}, color);
  }
  #[test]
  fn rgba_test() {
    let color = Color::rgba(255, 255, 255, 1.0).unwrap();
    assert_eq!(Color{r: 255, g: 255, b: 255, a: 255}, color);
  }
  #[test]
  fn hex_test() {
    let color = Color::hex("#fff").unwrap();
    assert_eq!(Color{r: 255, g: 255, b: 255, a: 255}, color);
    let color = Color::hex("#fff0").unwrap();
    assert_eq!(Color{r: 255, g: 255, b: 255, a: 0}, color);
    let color = Color::hex("#ffffff00").unwrap();
    assert_eq!(Color{r: 255, g: 255, b: 255, a: 0}, color);
  }

  #[test]
  fn to_hex_test() {
    let color = Color::hex("#fff").unwrap();
    assert_eq!("#ffffffff".to_string(), color.to_hex());
  }
  #[test]
  fn to_rgb_test() {
    let color = Color::hex("#fff").unwrap();
    assert_eq!("rgb(255, 255, 255)".to_string(), color.to_rgb());
    let color = Color::hex("#fff0").unwrap();
    assert_eq!("rgba(255, 255, 255, 0)".to_string(), color.to_rgb());
  }
}
