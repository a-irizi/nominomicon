use nom::{
  IResult, Parser,
  branch::alt,
  bytes::complete::{tag, tag_no_case},
  character::complete::{i32 as nom_i32, space0},
  combinator::value,
  sequence::{delimited, separated_pair},
};

fn parse_bool(input: &str) -> IResult<&str, bool> {
  alt((value(true, tag_no_case("true")), value(false, tag_no_case("false")))).parse(input)
}

#[test]
fn it_works() {
  let input = "falsetruefoo";
  let (rest, parsed) = parse_bool(input).unwrap();
  assert!(!parsed);
  let (rest, parsed) = parse_bool(rest).unwrap();
  assert!(parsed);
  let result = parse_bool(rest).unwrap_err();
  assert!(
    matches!(
      result,
      nom::Err::Error(nom::error::Error { input: "foo", code: nom::error::ErrorKind::Tag })
    ),
    "{result:?}"
  );
}

#[derive(Debug, PartialEq, Eq)]
struct Coordinates {
  x: i32,
  y: i32,
}

fn parse_coordinates(input: &str) -> IResult<&str, Coordinates> {
  delimited(
    tag("("),
    separated_pair(
      delimited(space0, nom_i32, space0),
      tag(","),
      delimited(space0, nom_i32, space0),
    ),
    tag(")"),
  )
  .parse(input)
  .map(|(rest, (x, y))| (rest, Coordinates { x, y }))
}

#[test]
fn parse_coordinates_works() {
  let input = vec!["(   3 ,    -2)", "(3,-2)", "(3 ,-2)"];
  for input in input {
    let (rest, coordinates) = parse_coordinates(input).unwrap();
    assert_eq!(("", Coordinates { x: 3, y: -2 }), (rest, coordinates));
  }

  let input = "(3  -2)";
  let result = parse_coordinates(input).unwrap_err();
  assert!(
    matches!(
      result,
      nom::Err::Error(nom::error::Error { input: "-2)", code: nom::error::ErrorKind::Tag })
    ),
    "{result:?}"
  );
}
