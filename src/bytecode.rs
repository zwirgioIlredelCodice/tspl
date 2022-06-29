use nom::{
    branch::alt,
    bytes::complete::{escaped_transform, is_a, is_not, tag},
    character::complete::char,
    combinator::{eof, map, recognize, value},
    multi::{fold_many0, many1},
    sequence::{delimited, separated_pair, terminated},
    IResult,
};

use std::collections::HashMap;

fn word_parser(i: &str) -> IResult<&str, String> {
    map(recognize(many1(is_not("\" \t\n\r"))), |s: &str| {
        s.to_string()
    })(i)
}

fn string_parser(i: &str) -> IResult<&str, String> {
    delimited(
        char('"'),
        escaped_transform(
            recognize(many1(is_not("\"\\"))),
            '\\',
            alt((
                value("\\", tag("\\")),
                value("\"", tag("\"")),
                value("\n", tag("n")),
                value("\t", tag("t")),
                value("\r", tag("r")),
            )),
        ),
        char('"'),
    )(i)
}

fn line_parser(i: &str) -> IResult<&str, (String, String)> {
    terminated(
        separated_pair(
            alt((word_parser, string_parser)),
            char(' '),
            alt((word_parser, string_parser)),
        ),
        is_a("\n\r\0"),
    )(i)
}

/// parse program in a HashMap<String, String>> ready to be used in tspl vm0
/// must contain a newline at hte end
pub fn program_parser(i: &str) -> IResult<&str, HashMap<String, String>> {
    terminated(
        fold_many0(
            line_parser,
            HashMap::new,
            |mut table: HashMap<String, String>, item| {
                table.insert(item.0, item.1);
                table
            },
        ),
        eof,
    )(i)
}
