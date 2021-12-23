# Propagate error on nom.

Working on [Sōzu issue #744](https://github.com/sozu-proxy/sozu/issues/744).

What on earth did Geal mean when he wrote:

> To propagate the error, you can use a custom error type that implements
> FromExternalError with serde_json errors: https://docs.rs/nom/7.1.0/nom/combinator/fn.map_res.html
> The map_res combinator is designed for that.
>
> You will also need the [cut](https://docs.rs/nom/7.1.0/nom/combinator/fn.cut.html)
> combinator to transform the Error into a Failure,
> then the many0 combinator will return an error instead of stopping silently

Let's see what `map_res` does and how to slip in this `cut` thing.

## Parse json into a struct

What Sōzu does:

```rust
use nom::{
    bytes::streaming::is_not,
    character::streaming::char,
    combinator::{complete, cut, map_res},
    multi::many0,
    sequence::terminated,
};

pub fn parse(input: &[u8]) -> IResult<&[u8], Vec<CommandRequest>> {
    use serde_json::from_slice;
    many0(complete(terminated(
        map_res(is_not("\0"), from_slice::<CommandRequest>),
        char('\0'),
    )))(input)
}
```

I want to reproduce this using a custom struct `User` that can be serialized and deserialized:

```rust
pub fn parse_several_users(input: &str) -> IResult<&str, Vec<User>> {
    many0(complete(terminated(
        map_res(is_not("\n"), serde_json::from_str::<User>),
        nom::character::complete::char('\n'),
    )))(input)
}
```

The goal is to make a proper use of `cut` and of a custom error, so that any `serde_json` error is converted to a failure and propagated into `IResult`.
