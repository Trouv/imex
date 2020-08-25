use super::{
    imex::{IMEx, IMExVal, QuantifiedIMExVal},
    quantifier::Quantifier,
};
use nom::{
    branch::alt,
    character::complete::{char, digit1, one_of},
    combinator::all_consuming,
    combinator::opt,
    multi::{many0, many_till},
    sequence::delimited,
    IResult,
};
use std::iter::once;

fn parse_finite_quantifier(input: &str) -> IResult<&str, Quantifier> {
    match opt(delimited(char('{'), digit1, char('}')))(input)? {
        (input, Some(x)) => Ok((
            input,
            Quantifier::Finite(x.parse::<usize>().expect("Expected value to be a digit")),
        )),
        (input, None) => Ok((input, Quantifier::Finite(1))),
    }
}

fn parse_infinite_quantifier(input: &str) -> IResult<&str, Quantifier> {
    let (input, _) = char('*')(input)?;
    Ok((input, Quantifier::Infinite))
}

fn parse_quantifier(input: &str) -> IResult<&str, Quantifier> {
    alt((parse_infinite_quantifier, parse_finite_quantifier))(input)
}

fn parse_single_imex_val(input: &str) -> IResult<&str, IMExVal> {
    let (input, x) = one_of("0123456789")(input)?;
    Ok((
        input,
        IMExVal::Single(once(
            x.to_digit(10).expect("Expected value to be a digit") as usize
        )),
    ))
}

fn parse_group_imex_val(input: &str) -> IResult<&str, IMExVal> {
    let (input, _) = char('(')(input)?;
    let (input, imex) = parse_inner_imex(input)?;
    Ok((input, IMExVal::Group(imex)))
}

fn parse_imex_val(input: &str) -> IResult<&str, IMExVal> {
    alt((parse_single_imex_val, parse_group_imex_val))(input)
}

fn parse_quantified_imex_val(input: &str) -> IResult<&str, QuantifiedIMExVal> {
    let (input, val) = parse_imex_val(input)?;
    let (input, quantifier) = parse_quantifier(input)?;
    Ok((input, QuantifiedIMExVal::from(val, quantifier)))
}

fn parse_inner_imex(input: &str) -> IResult<&str, IMEx> {
    let (input, imex) = many_till(parse_quantified_imex_val, char(')'))(input)?;
    Ok((input, IMEx::new(imex.0.into_iter())))
}

/// Parser combinator for parsing an [`IMEx`](../imex/struct.IMEx.html), making use of the
/// [`nom`](https://docs.rs/nom/6.0.0-alpha1/nom/index.html) library. Unless you're building your
/// own parser that incorporates IMExes using parser combinators, you may prefer to use
/// [`IMEx::from`](../imex/struct.IMEx.html#method.from), which uses this function but loses the
/// parser combinator details.
///
/// # Error
/// Results in an error if the input string is not a valid IMEx.
///
/// # Example
/// ```
/// use imex::expression::{IMEx, parsers::parse_imex};
///
/// let (remaining_input, parsed_imex) = parse_imex("12(34){56}")
///     .expect("Invalid IMEx");
/// assert_eq!(
///     parsed_imex,
///     IMEx::from("12(34){56}").expect("Invalid IMEx")
/// );
/// ```
/// Currently, this parser combinator expects to be "all consuming", which means it will fail if
/// there is any input string remaining after parsing an IMEx. This could pose compatibility issues
/// if you want to use this in your own set of parser combinators. If this is a use case for you,
/// consider contributing to this project on [github](https://github.com/Trouv/imex).
pub fn parse_imex(input: &str) -> IResult<&str, IMEx> {
    let (input, imex) = all_consuming(many0(parse_quantified_imex_val))(input)?;
    Ok((input, IMEx::new(imex.into_iter())))
}
