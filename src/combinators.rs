use api::{Parser, ParseResult, Token};

pub fn string<T: Token>(string: &[T]) -> Parser<T> {
    Parser::new(move |input, pos| {
        if string[..] == input[pos.cursor()..pos.cursor() + string.len()] {
            ParseResult::Success(string.len())
        } else {
            ParseResult::Failure
        }
    })
}

pub fn compose<'f, T: Token>(parsers: &'f [Parser<T>]) -> Parser<'f, T> {
    Parser::new(move |input, pos| {
        let mut consumed = 0;

        for parser in parsers {
            match parser.call(input, pos.advance(consumed)) {
                ParseResult::Success(c) => consumed += c,
                ParseResult::Failure => return ParseResult::Failure,
            }
        }

        ParseResult::Success(consumed)
    })
}