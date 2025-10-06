use nom::{IResult, Parser, bytes::complete::tag, multi::many0};

fn many_abc(input: &str) -> IResult<&str, Vec<&str>> {
  many0(tag("abc")).parse(input)
}

#[test]
fn many_abc_woks() {
  let input = "abcabcabcabC";
  let (rest, parsed) = many_abc(input).unwrap();
  assert_eq!("abC", rest);
  assert_eq!(vec!["abc", "abc", "abc"], parsed);

  let input = "abCabcabcabC";
  let (rest, parsed) = many_abc(input).unwrap();
  assert_eq!(input, rest);
  assert!(parsed.is_empty());
}
