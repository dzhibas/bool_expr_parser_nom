use nom::{
    branch::alt,
    bytes::complete::{tag, take_until, tag_no_case},
    character::{complete::{alpha1, alphanumeric1, line_ending, multispace0, space0, char} },
    combinator::{eof, recognize, map_res, map},
    complete::take,
    multi::{many0, many0_count, many_till},
    sequence::{pair, preceded, tuple, separated_pair, delimited},
    Err, IResult,
};

use std::{collections::HashMap, error::Error};

type Pair = HashMap<String, String>;
type AppError = Box<dyn Error>;

#[derive(Debug)]
enum LogicExpr {
    And(Pair, Pair),
    Or(Pair, Pair),
}

enum ComparisonExpr {
    Eq,
    More,
    Less,
    MoreEq,
    LessEq,
    NotEq,
}

impl ComparisonExpr {
    fn from_str(expr: &str) -> Self {
        match expr {
            "==" | "=" => ComparisonExpr::Eq,
            ">" => ComparisonExpr::More,
            ">=" => ComparisonExpr::MoreEq,
            "<" => ComparisonExpr::Less,
            "<=" => ComparisonExpr::LessEq,
            "!=" => ComparisonExpr::NotEq,
            _ => unreachable!(),
        }
    }
}

fn parse_variable(input: &str) -> IResult<&str, &str> {
    recognize(pair(
        alt((alpha1, tag("_"))),
        many0_count(alt((alphanumeric1, tag("_")))),
    ))(input)
}

fn parse_variable_clean_spaces(input: &str) -> IResult<&str, &str> {
    preceded(multispace0, parse_variable)(input)
}

fn parse_equal(input: &str) -> IResult<&str, (&str, &str, &str)> {
    tuple((space0, tag("="), space0))(input)
}

fn parse_string_value(i: &str) -> IResult<&str, &str> {
    let (tail, (_, str, _)) = tuple((char('"'), take_until("\""), char('"')))(i)?;
    Ok((tail, str))
}

fn parse_assignment(input: &str) -> IResult<&str, Pair> {
    map(
        tuple((parse_variable_clean_spaces, parse_equal, parse_string_value)), 
        |(var, _, val)| {
            HashMap::from([(var.to_string(),val.to_string())])
        },
    )(input)
}

fn parse_bool_expr_and(i: &str) -> IResult<&str, LogicExpr> {
    let and = delimited(multispace0, tag_no_case("and"), multispace0);
    map(separated_pair(parse_assignment, and, parse_assignment), |(p1, p2)| 
        LogicExpr::And(p1, p2)
    ,)(i)
}

fn parse_bool_expr_or(i: &str) -> IResult<&str, LogicExpr> {
    let or = delimited(multispace0, tag_no_case("or"), multispace0);
    map(separated_pair(parse_assignment, or, parse_assignment), |(p1, p2)| 
        LogicExpr::Or(p1, p2)
    ,)(i)
}

fn parse_main(input: &str) -> IResult<&str, Vec<LogicExpr>> {
    many0(alt((parse_bool_expr_and, parse_bool_expr_or)))(input)
}

fn main() -> Result<(), AppError> {
    let content = r##"street_name ="Random this or that" OR countryCode = "123 NL 123""##;

    let res = parse_main(content)?;

    println!("Trying to parse: {}", content);
    dbg!(res);

    Ok(())
}
