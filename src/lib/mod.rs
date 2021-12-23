pub mod user;

use user::User;

use nom::{
    bytes::complete::is_not,
    // bytes::streaming::is_not,
    // character::streaming::char,
    combinator::{complete, cut, map_res},
    error::Error as NomError,
    error::{ErrorKind, FromExternalError},
    multi::many0,
    sequence::terminated,
    IResult,
};
use serde_json::from_str;

/**
 * Documentation:

pub trait nom::Error::FromExternalError<I, E> {
    fn from_external_error(input: I, kind: ErrorKind, e: E) -> Self;
}

*/

/*
// What should this look like?
enum CustomError<I, E> {
    SerdeError(String),
    NomError(I, ErrorKind),
}


impl FromExternalError<str, serde_json::Error> for CustomError<&str, serde_json::Error> {
    fn from_external_error(
        input: &str,
        kind: ErrorKind,
        serde_json_error: serde_json::Error,
    ) -> Self {
        let message: String = format!("{:?}", serde_json_error);
        CustomError::NomError(input, kind)
    }
}
*/

#[derive(Debug)]
pub struct CustomError {
    kind: ErrorKind,
    serde_json_error: Option<serde_json::Error>,
}

impl FromExternalError<&str, serde_json::Error> for CustomError {
    fn from_external_error(
        input: &str,
        kind: ErrorKind,
        serde_json_error: serde_json::Error,
    ) -> Self {
        println!("input: {}, error kind: {:?}", input, kind);
        Self {
            kind,
            serde_json_error: Some(serde_json_error),
        }
    }
}

impl nom::error::ParseError<&str> for CustomError {
    fn from_error_kind(input: &str, kind: ErrorKind) -> Self {
        println!("input: {}, error kind: {:?}", input, kind);

        Self {
            kind,
            serde_json_error: None,
        }
    }

    fn append(_input: &str, _kind: ErrorKind, other: Self) -> Self {
        other
    }
}

pub fn parse_one_user(input: &str) -> IResult<&str, User, CustomError> {
    let (i, json_data) = is_not("\n")(input)?;

    let user = match serde_json::from_str::<User>(json_data) {
        Ok(user) => user,
        Err(serde_error) => {
            return Err(nom::Err::Failure(CustomError::from_external_error(
                input,
                ErrorKind::MapRes,
                serde_error,
            )))
        }
    };

    let (next_input, _) = nom::character::complete::char('\n')(i)?;

    Ok((next_input, user))
}

pub fn parse_several_users(input: &str) -> IResult<&str, Vec<User>, CustomError> {
    many0(parse_one_user)(input)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_one_user_works() {
        let str = "{\"username\":\"Spongebob\",\"password\":\"HeyPatrick\"}\n";

        assert_eq!(
            parse_several_users(str).unwrap(),
            ("", vec![User::new("Spongebob", "HeyPatrick")])
        )
    }

    #[test]
    fn parse_several_users_works() {
        let random_users = User::create_random_users(3);
        let stringified_users = User::serialize_users(&random_users).unwrap();

        println!("{}", stringified_users);
        assert_eq!(
            parse_several_users(&stringified_users).unwrap(),
            ("", random_users)
        )
    }

    #[test]
    fn bad_input_yields_an_error() {
        let bad_users_input = r#"{"username":345,"password":"hV9StRA"}
{"username":"qETqU6t","password":"gykzW8x"}
{"username":"2vhA0B0","password":"SDGJDGk"}
"#;

        assert!(
            parse_several_users(bad_users_input).is_err(),
            // Err(nom::error::Error(serde_json::Error { err: "bla" }))
            // nom::Err::Error(nom::error::Error { input: bad_users_input, code: ErrorKind::Fail })
        );
    }
}
