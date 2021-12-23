pub mod user;

use user::User;

use nom::{
    bytes::complete::is_not,
    // bytes::streaming::is_not,
    // character::streaming::char,
    combinator::{complete, cut, map_res},
    error::ErrorKind,
    multi::many0,
    sequence::terminated,
    IResult,
};
use serde_json::from_str;

pub fn parse_user_from_str(input: &str) -> IResult<&str, User> {
    Ok(("", serde_json::from_str::<User>(input).unwrap()))
}

pub fn parse_several_users(input: &str) -> IResult<&str, Vec<User>> {
    // this is as close as it gets to the syntax of Sōzu's CommandRequest parser
    
    many0(complete(terminated(
        map_res(is_not("\n"), serde_json::from_str::<User>),
        nom::character::complete::char('\n'),
    )))(input)
}

#[cfg(test)]
mod test {
    use super::*;
    // use nom::{
    //     error::{ErrorKind, VerboseError, VerboseErrorKind},
    //     Err as NomErr,
    // };

    #[test]
    fn parse_one_user_works() {
        let str = "{\"username\":\"Spongebob\",\"password\":\"HeyPatrick\"}\nbla";

        assert_eq!(
            parse_several_users(str),
            Ok(("bla", vec![User::new("Spongebob", "HeyPatrick")]))
        )
    }

    #[test]
    fn parse_several_users_works() {
        let random_users = User::create_random_users(3);
        let stringified_users = User::serialize_users(&random_users).unwrap();

        assert_eq!(
            parse_several_users(&stringified_users),
            Ok(("", random_users)),
        )
    }

    #[test]
    fn bad_input_yields_an_error() {
        let bad_users_input = r#"{"username":345,"password":"hV9StRA"}
{"username":"qETqU6t","password":"gykzW8x"}
{"username":"2vhA0B0","password":"SDGJDGk"}
"#;

        assert_eq!(
            parse_several_users(bad_users_input),
            // Err(nom::error::Error(serde_json::Error { err: "bla" }))
            Ok(("", vec![]))
        );
    }
}
