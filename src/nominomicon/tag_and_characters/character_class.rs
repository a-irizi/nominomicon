use nom::{IResult, character::complete::alpha0};

fn parser(input: &str) -> IResult<&str, &str> {
  alpha0(input)
}

#[test]
fn alpha0_works() {
  let input = "abc123";
  let (rest, parsed) = parser(input).unwrap();
  assert_eq!("123", rest);
  assert_eq!("abc", parsed);

  let input = "123abc";
  let (rest, parsed) = parser(input).unwrap();
  assert_eq!(input, rest);
  assert_eq!("", parsed);
}
