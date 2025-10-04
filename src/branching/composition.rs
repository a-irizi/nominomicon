use nom::{
  IResult,
  branch::alt,
  bytes::complete::{tag, tag_no_case},
  character::complete::{alpha1, space0},
  sequence::tuple,
};

fn parse_alpha1(input: &str) -> IResult<&str, &str> {
  alpha1(input)
}

fn parse_space(input: &str) -> IResult<&str, &str> {
  space0(input)
}

fn parse_def_or_ghi(input: &str) -> IResult<&str, &str> {
  alt((tag("def"), tag("ghi")))(input)
}

#[test]
fn composition_works() {
  let input = "sometext ghidef";
  let (rest, parsed_alpha) = parse_alpha1(input).unwrap();
  let (rest, ..) = parse_space(rest).unwrap();
  let (rest, parsed_def_or_ghi) = parse_def_or_ghi(rest).unwrap();

  assert_eq!("sometext", parsed_alpha);
  assert_eq!("ghi", parsed_def_or_ghi);
  assert_eq!("def", rest);
}

fn parse_base(input: &str) -> IResult<&str, &str> {
  alt((tag_no_case("a"), tag_no_case("t"), tag_no_case("c"), tag_no_case("g")))(input)
}

fn parse_pair_base(input: &str) -> IResult<&str, (&str, &str)> {
  tuple((parse_base, parse_base))(input)
}

#[test]
fn tuple_works() {
  let input = "acfoo";
  let (rest, parsed) = parse_pair_base(input).unwrap();
  assert_eq!("foo", rest);
  assert!(matches!(parsed, ("a", "c")), "{parsed:?}");
}

#[test]
fn tuple_fails() {
  let input = "adfoo";
  let result = parse_pair_base(input).unwrap_err();
  assert!(
    matches!(
      result,
      nom::Err::Error(nom::error::Error { input: "dfoo", code: nom::error::ErrorKind::Tag })
    ),
    "{result:?}"
  );
}
