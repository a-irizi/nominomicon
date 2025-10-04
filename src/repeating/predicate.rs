use nom::{
  IResult,
  bytes::complete::{take_until, take_while},
  sequence::terminated,
};

fn parse_sentence(input: &str) -> IResult<&str, &str> {
  terminated(take_until("."), take_while(|c: char| c == '.' || c.is_whitespace()))(input)
}

#[test]
fn parse_sentence_works() {
  let input = "I am tom. I write Rust.";
  let (rest, parsed) = parse_sentence(input).unwrap();
  assert_eq!("I write Rust.", rest);
  assert_eq!("I am tom", parsed);

  let input = "I am tom, I write Rust.";
  let (rest, parsed) = parse_sentence(input).unwrap();
  assert_eq!("", rest);
  assert_eq!("I am tom, I write Rust", parsed);

  let input = "I am tom, I write Rust";
  let result = parse_sentence(input).unwrap_err();
  assert!(
    matches!(
      result,
      nom::Err::Error(nom::error::Error {
        input: "I am tom, I write Rust",
        code: nom::error::ErrorKind::TakeUntil
      })
    ),
    "{result:?}"
  );
}
