pub trait Token: PartialEq {}

pub struct Parser<'f, T: Token>(pub Box<'f + Fn(&[T], ParsePosition) -> ParseResult>);

impl <'f, T: Token> Parser<'f, T> {
    pub fn new<F: 'f>(closure: F) -> Parser<'f, T> 
        where F: Fn(&[T], ParsePosition) -> ParseResult {

        Parser(Box::new(closure))
    }

    #[inline]
    pub fn call(&self, input: &[T], position: ParsePosition) -> ParseResult {
        self.0(input, position)
    }

    #[inline]
    pub fn run(&self, input: &[T]) -> ParseResult {
        self.0(input, ParsePosition::new())
    }
}

#[derive(PartialEq, Debug)]
pub enum ParseResult {
    Success(usize),
    Failure
}

impl ParseResult {
    pub fn succeeded(&self) -> bool {
        match *self {
            ParseResult::Success(_) => true,
            ParseResult::Failure => false,
        }
    }
}

pub struct ParsePosition {
    cursor: usize
}

impl ParsePosition {
    pub fn new() -> ParsePosition {
        ParsePosition { cursor: 0 }
    }

    pub fn cursor(&self) -> usize {
        self.cursor
    }

    pub fn advance(&self, amount: usize) -> ParsePosition {
        ParsePosition { cursor: self.cursor + amount }
    }
}