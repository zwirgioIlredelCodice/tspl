use nom::{
    branch::alt,
    bytes::complete::{is_not, is_a},
    bytes::complete::{escaped, tag},
    character::{complete::char, complete::one_of},
    combinator::{eof, recognize},
    multi::{fold_many0, many1},
    sequence::{delimited, separated_pair, terminated},
    IResult,
};

use std::collections::HashMap;

fn word_parser(i: &str) -> IResult<&str, &str> {
    recognize(many1(is_not("\" \n\t\r")))(i)
}

fn string_parser(i: &str) -> IResult<&str, &str> {
    delimited(
        char('"'),
        recognize(escaped(
            recognize(many1(is_not("\"\\"))),
            '\\',
            one_of(r#""abtnvfr\"#)
        )),
        char('"'),
    )(i)
}

fn line_parser(i: &str) -> IResult<&str, (&str, &str)> {
    terminated(
        separated_pair(
            alt((word_parser, string_parser)),
            char(' '),
            alt((word_parser, string_parser)),
        ),
        is_a("\n\r"),
    )(i)
}


/// parse program in a HashMap<String, String>> ready to be used in tspl vm0
/// must contain a newline at hte end
pub fn program_parser(i: &str) -> IResult<&str, HashMap<String, String>> {
    terminated(
        fold_many0(line_parser, HashMap::new, |mut table: HashMap<String, String>, item| {
            table.insert(String::from(item.0), String::from(item.1));
            table
        }),
        eof,
    )(i)
}
