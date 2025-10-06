use nom::{
  IResult, Parser,
  bytes::complete::{tag, take_while_m_n},
};

#[derive(Debug, PartialEq)]
struct Color {
  pub r: u8,
  pub g: u8,
  pub b: u8,
}

impl Color {
  fn new(r: u8, g: u8, b: u8) -> Self {
    Self { r, g, b }
  }
}

// sample input: "FF"
// output: 255
fn parse_hex_segment(input: &str) -> IResult<&str, u8> {
  take_while_m_n(2, 2, |c: char| c.is_ascii_hexdigit())
    .map_res(|parsed| u8::from_str_radix(parsed, 16))
    .parse(input)
}

// sample input: "#FFAA00"
// output: Color {r: 255, g: 170, b: 0 }
fn parse_hex_color_no_alpha(input: &str) -> IResult<&str, Color> {
  let (rest, _) = tag("#").parse(input)?;
  let (rest, r) = parse_hex_segment(rest)?;
  let (rest, g) = parse_hex_segment(rest)?;
  let (rest, b) = parse_hex_segment(rest)?;

  Ok((rest, Color { r, g, b }))
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn parse_hex_segment_works() {
    let input = "FFAA00";
    let (rest, parsed) = parse_hex_segment(input).unwrap();
    assert_eq!("AA00", rest);
    assert_eq!(0xFF, parsed);
    let (rest, parsed) = parse_hex_segment(rest).unwrap();
    assert_eq!("00", rest);
    assert_eq!(0xAA, parsed);
    let (rest, parsed) = parse_hex_segment(rest).unwrap();
    assert_eq!("", rest);
    assert_eq!(0x00, parsed);

    let result = parse_hex_segment(rest).unwrap_err();
    assert!(matches!(
      result,
      nom::Err::Error(nom::error::Error { input: "", code: nom::error::ErrorKind::TakeWhileMN })
    ));
  }

  #[test]
  fn parse_hex_color_no_alpha_works() {
    let input = "#FFAA00";
    let (rest, parsed) = parse_hex_color_no_alpha(input).unwrap();
    assert_eq!("", rest);
    assert_eq!(Color { r: 0xFF, g: 0xAA, b: 0x00 }, parsed);

    let input = "#123456foo";
    let (rest, parsed) = parse_hex_color_no_alpha(input).unwrap();
    assert_eq!("foo", rest);
    assert_eq!(Color { r: 0x12, g: 0x34, b: 0x56 }, parsed);

    let input = "FFAA00";
    let result = parse_hex_color_no_alpha(input).unwrap_err();
    assert!(matches!(
      result,
      nom::Err::Error(nom::error::Error { input: "FFAA00", code: nom::error::ErrorKind::Tag })
    ));

    let input = "#FFAA";
    let result = parse_hex_color_no_alpha(input).unwrap_err();
    assert!(
      matches!(
        result,
        nom::Err::Error(nom::error::Error { input: "", code: nom::error::ErrorKind::TakeWhileMN })
      ),
      "{result:?}"
    );

    let input = "#FFAA0";
    let result = parse_hex_color_no_alpha(input).unwrap_err();
    assert!(
      matches!(
        result,
        nom::Err::Error(nom::error::Error { input: "0", code: nom::error::ErrorKind::TakeWhileMN })
      ),
      "{result:?}"
    );
  }
}
