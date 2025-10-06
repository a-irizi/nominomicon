use nom::{IResult, bytes::complete::tag};

fn parse_input(input: &str) -> IResult<&str, &str> {
  tag("abc")(input)
}

#[test]
fn tag_works() {
  let input = "abcWorld";
  let (leftover, parsed) = parse_input(input).unwrap();
  assert_eq!("abc", parsed);
  assert_eq!("World", leftover);

  let result = parse_input("defWorld");
  assert!(result.is_err());
  let error = result.unwrap_err();
  assert!(matches!(error, nom::Err::Error(_)));
  match error {
    nom::Err::Incomplete(_) => todo!(),
    nom::Err::Error(e) => {
      assert!(matches!(e.code, nom::error::ErrorKind::Tag));
      assert!(matches!(e.input, "defWorld"));
    }
    nom::Err::Failure(_) => todo!(),
  }
}

#[test]
fn tag_no_case_works() {
  use nom::bytes::complete::tag_no_case;

  let tag_no_case = tag_no_case::<&str, &str, nom::error::Error<&str>>;

  let input = "abcWorld";
  let (leftover, parsed) = tag_no_case("abc")(input).unwrap();
  assert_eq!(leftover, "World");
  assert_eq!(parsed, "abc");

  let input = "AbCWorld";
  let (leftover, parsed) = tag_no_case("aBc")(input).unwrap();
  assert_eq!(leftover, "World");
  assert_eq!(parsed, "AbC");

  let input = "AdCWorld";
  let result = tag_no_case("aBc")(input).unwrap_err();
  assert!(matches!(
    result,
    nom::Err::Error(nom::error::Error { input: i, code: nom::error::ErrorKind::Tag })  if i == input
  ));
}
