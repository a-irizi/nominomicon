use nom::{
  IResult, Parser,
  branch::alt,
  bytes::complete::{tag, tag_no_case},
};

fn parse_abc_case_or_def_no_case(input: &str) -> IResult<&str, &str> {
  alt((tag("abc"), tag_no_case("def"))).parse(input)
}

#[test]
fn it_works() {
  let input = "abcDeF";
  let (rest, parsed) = parse_abc_case_or_def_no_case(input).unwrap();
  assert_eq!("DeF", rest);
  assert_eq!("abc", parsed);

  let input = "DeFbla";
  let (rest, parsed) = parse_abc_case_or_def_no_case(input).unwrap();
  assert_eq!("bla", rest);
  assert_eq!("DeF", parsed);
}

#[test]
fn it_fails() {
  let input = "AbcDeF";
  let result = parse_abc_case_or_def_no_case(input).unwrap_err();
  assert!(
    matches!(
      result,
      nom::Err::Error(nom::error::Error { input: "AbcDeF", code: nom::error::ErrorKind::Tag })
    ),
    "{result:?}"
  );
}
