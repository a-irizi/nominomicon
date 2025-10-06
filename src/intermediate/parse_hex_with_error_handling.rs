use nom::{
  IResult, Parser,
  bytes::{complete::take_while_m_n, tag},
  error::{Context, context},
  sequence::preceded,
};

fn parse_hex_segment(input: &str) -> IResult<&str, u8, nom_language::error::VerboseError<&str>> {
  take_while_m_n(2, 2, |c: char| c.is_ascii_hexdigit())
    .map_res(|parsed| u8::from_str_radix(parsed, 16))
    .parse(input)
}

fn parse_hex_rgb(
  input: &str,
) -> IResult<&str, (u8, u8, u8), nom_language::error::VerboseError<&str>> {
  preceded(
    context("start of hex color", tag("#")),
    (
      context("hex color red segment", parse_hex_segment),
      context("hex color green segment", parse_hex_segment),
      context("hex color blue segment", parse_hex_segment),
    ),
  )
  .parse(input)
}

#[cfg(test)]
mod tests {
  use nom_language::error::convert_error;

  use super::*;

  #[test]
  fn parse_hex_rgb_works() {
    let input = "#FFAA00";
    let expected = ("", (0xFF, 0xAA, 0x00));
    let actual = parse_hex_rgb(input).unwrap();
    assert_eq!(expected, actual);
    let input = "x#FFAA00";
    match parse_hex_rgb(input) {
      Err(nom::Err::Error(e) | nom::Err::Failure(e)) => {
        let user_friendly_error = convert_error(input, e);
        println!("Could not parse hex color because:\n{user_friendly_error}");
      }
      _ => unreachable!("invalid hex color should not return Ok()"),
    }

    let input = "#FFAG00";
    match parse_hex_rgb(input) {
      Err(nom::Err::Error(e) | nom::Err::Failure(e)) => {
        let user_friendly_error = convert_error(input, e);
        println!("Could not parse hex color because:\n{user_friendly_error}");
      }
      _ => unreachable!("invalid hex color should not return Ok()"),
    }
  }
}
