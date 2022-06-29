use nom::{
    branch::alt,
    bytes::complete::{escaped_transform, tag, is_not, is_a},
    character::{complete::char, streaming::alpha1},
    combinator::{eof, recognize, value},
    multi::{fold_many0, many1},
    sequence::{delimited, separated_pair, terminated},
    IResult,
};

use std::collections::HashMap;

fn word_parser(i: &str) -> IResult<&str, &str> {
    recognize(many1(is_not("\" \t\n\r")))(i)
}

fn string_parser(i: &str) -> IResult<&str, &str> {
    delimited(
        char('"'),
        recognize(escaped_transform(
            recognize(many1(is_not("\"\\"))),
            '\\',
            alt((
                value('\\', tag("\\")),
                value('\"', tag("\"")),
                value('\n', tag("n")),
                value('\t', tag("t")),
                value('\r', tag("r")),
            )),
        )),
        char('"'),
    )(i)
}

fn string1_parser(i: &str) -> IResult<&str, String> {
    delimited(
        char('"'),
        escaped_transform(
            alpha1,
            '\\',
            alt((
                value('\\', tag("\\")),
                value('\"', tag("\"")),
                value('\n', tag("n")),
                value('\t', tag("t")),
                value('\r', tag("r")),
            )),
        ),
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_string() {
        // https://docs.rs/nom/latest/nom/bytes/complete/fn.escaped_transform.html
        assert_eq!(string1_parser("ab\\\"cd"), Ok(("", String::from("ab\"cd"))));
    }
}