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
        IMExVal::Single(x.to_digit(10).expect("Expected value to be a digit") as usize),
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
    Ok((input, QuantifiedIMExVal { val, quantifier }))
}

fn parse_inner_imex(input: &str) -> IResult<&str, IMEx> {
    let (input, imex) = many_till(parse_quantified_imex_val, char(')'))(input)?;
    Ok((input, IMEx(imex.0)))
}

pub fn parse_imex(input: &str) -> IResult<&str, IMEx> {
    let (input, imex) = all_consuming(many0(parse_quantified_imex_val))(input)?;
    Ok((input, IMEx(imex)))
}
