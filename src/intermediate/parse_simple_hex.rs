use nom::{
  IResult, Parser,
  bytes::complete::{tag, take_while_m_n},
  sequence::preceded,
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

// sample input: "#FFAA00"
// output: Color {r: 255, g: 170, b: 0 }
fn parse_hex_color_no_alpha_2(input: &str) -> IResult<&str, Color> {
  preceded(tag("#"), (parse_hex_segment, parse_hex_segment, parse_hex_segment))
    .map(|(r, g, b)| Color { r, g, b })
    .parse(input)
}

fn slices_around_successful_parse<'input>(
  input: &'input str,
  mut parser: impl Parser<&'input str, Output = Color, Error = nom::error::Error<&'input str>>,
) -> IResult<(&'input str, &'input str), Color> {
  let mut latest_error =
    nom::Err::Error(nom::error::Error { input, code: nom::error::ErrorKind::Fail });

  for (idx, _) in input.char_indices() {
    let input2 = &input[idx..];
    match parser.parse(input2) {
      Ok((rest, color)) => return Ok(((&input[..idx], rest), color)),
      Err(e) => latest_error = e,
    }
  }

  Err(latest_error.map(|e| nom::error::Error { input: (e.input, ""), code: e.code }))
}

// sample input: "\n☀️\n#FFAA00\n🌙\n"
// output: Color {r: 255, g: 170, b: 0 }
// rest: ("\n☀️\n", "\n🌙\n")
fn parse_first_encountered_hex_color_no_alpha(input: &str) -> IResult<(&str, &str), Color> {
  {
    let mut parser = parse_hex_color_no_alpha_2;
    let mut latest_error =
      nom::Err::Error(nom::error::Error { input, code: nom::error::ErrorKind::Fail });

    for idx in input.char_indices().filter_map(|(idx, c)| if c == '#' { Some(idx) } else { None }) {
      let input2 = &input[idx..];
      match parser.parse(input2) {
        Ok((rest, color)) => return Ok(((&input[..idx], rest), color)),
        Err(e) => latest_error = e,
      }
    }

    Err(latest_error.map(|e| nom::error::Error { input: (e.input, ""), code: e.code }))
  }
}

#[cfg(test)]
mod tests {
  use rstest::rstest;

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

  #[rstest]
  #[case::parse_hex_color_no_alpha(parse_hex_color_no_alpha)]
  #[case::parse_hex_color_no_alpha(parse_hex_color_no_alpha_2)]
  fn parse_hex_color_no_alpha_works(#[case] parser: impl Fn(&str) -> IResult<&str, Color>) {
    let input = "#FFAA00";
    let (rest, parsed) = parser(input).unwrap();
    assert_eq!("", rest);
    assert_eq!(Color { r: 0xFF, g: 0xAA, b: 0x00 }, parsed);

    let input = "#123456foo";
    let (rest, parsed) = parser(input).unwrap();
    assert_eq!("foo", rest);
    assert_eq!(Color { r: 0x12, g: 0x34, b: 0x56 }, parsed);

    let input = "FFAA00";
    let result = parser(input).unwrap_err();
    assert!(matches!(
      result,
      nom::Err::Error(nom::error::Error { input: "FFAA00", code: nom::error::ErrorKind::Tag })
    ));

    let input = "#FFAA";
    let result = parser(input).unwrap_err();
    assert!(
      matches!(
        result,
        nom::Err::Error(nom::error::Error { input: "", code: nom::error::ErrorKind::TakeWhileMN })
      ),
      "{result:?}"
    );

    let input = "#FFAA0";
    let result = parser(input).unwrap_err();
    assert!(
      matches!(
        result,
        nom::Err::Error(nom::error::Error { input: "0", code: nom::error::ErrorKind::TakeWhileMN })
      ),
      "{result:?}"
    );
  }

  #[test]
  fn parse_first_encountered_hex_color_no_alpha_works() {
    let input = "\n☀️\n#FFA#\n#FFAA00\n🌙\n";
    let ((prefix, postfix), color) = parse_first_encountered_hex_color_no_alpha(input).unwrap();
    assert_eq!("\n☀️\n#FFA#\n", prefix);
    assert_eq!("\n🌙\n", postfix);
    assert_eq!(Color { r: 0xFF, g: 0xAA, b: 0x00 }, color);

    let input = "\n☀️\n#FFA#\n\n🌙\n#FFAA00";
    let ((prefix, postfix), color) = parse_first_encountered_hex_color_no_alpha(input).unwrap();
    assert_eq!("\n☀️\n#FFA#\n\n🌙\n", prefix);
    assert_eq!("", postfix);
    assert_eq!(Color { r: 0xFF, g: 0xAA, b: 0x00 }, color);

    let input = "#FFAA00\n☀️\n#FFA#\n\n🌙\n";
    let ((prefix, postfix), color) = parse_first_encountered_hex_color_no_alpha(input).unwrap();
    assert_eq!("", prefix);
    assert_eq!("\n☀️\n#FFA#\n\n🌙\n", postfix);
    assert_eq!(Color { r: 0xFF, g: 0xAA, b: 0x00 }, color);

    let input = "#FFAA0G\n☀️\n#FFA#\n\n🌙\n";
    let result = parse_first_encountered_hex_color_no_alpha(input).unwrap_err();
    assert!(
      matches!(
        result,
        nom::Err::Error(nom::error::Error {
          input: ("\n\n🌙\n", ""),
          code: nom::error::ErrorKind::TakeWhileMN
        })
      ),
      "{result:?}"
    );

    let input = "#FFAA0G\n☀️\n#FFA\n\n🌙\n";
    let result = parse_first_encountered_hex_color_no_alpha(input).unwrap_err();
    assert!(
      matches!(
        result,
        nom::Err::Error(nom::error::Error {
          input: ("A\n\n🌙\n", ""),
          code: nom::error::ErrorKind::TakeWhileMN
        })
      ),
      "{result:?}"
    );
  }
}
