# Use map_res

What on earth did Geal mean when he wrote:

> To propagate the error, you can use a custom error type that implements FromExternalError with serde_json errors: https://docs.rs/nom/7.1.0/nom/combinator/fn.map_res.html
> The map_res combinator is designed for that. You will also need the [cut](https://docs.rs/nom/7.1.0/nom/combinator/fn.cut.html) combinator to transform the Error into a Failure, then the many0 combinator will return an error instead of stopping silently

Let's see what `map_res` does and how to slip in this `cut` thing.