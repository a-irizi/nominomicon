use std::error::Error;

use nom::IResult;

pub fn do_nothing_parser(input: &str) -> IResult<&str, &str> {
  Ok((input, ""))
}

#[test]
fn it_works() {
  let input = "my_input";
  let (rest, parsed) = do_nothing_parser(input).unwrap();

  assert_eq!(input, rest);
  assert_eq!("", parsed);
}
