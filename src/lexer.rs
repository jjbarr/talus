use thiserror::Error;
use crate::optab::is_op;
use std::num::ParseIntError;
use std::str;
// Ideally the lexer is as dumb as possible, because then I don't have to fuck
// with it, and most of the work can be done inside the optable.  However, there
// are a few cases where the lexer actually (horror of horrors) needs
// knowledge. One is strings: [, {, and # open strings and the lexer needs to be
// aware of those. Parens are similar in that like { and #, they're not
// operators. Nothing else needs special treatment unless operators become
// harder to recognize.

#[derive(Debug, Clone, Copy)]
pub enum Token<'a> {
    Op(&'a [u8]),
    String(&'a [u8]),
    Num(i64),
    LPar,
    RPar,
    LMac,
    Comment
}

#[derive(Debug, Clone, Copy)]
enum State {
    Default,
    CommStr,
    DelimStr(&'static [u8])
}

#[derive(Debug)]
pub struct Lexer<'a> {
    src: &'a str,
    pos: usize,
    state: State
}

#[derive(Debug, Error)]
pub enum LexError<'a> {
    #[error("Missing `{}`", str::from_utf8(.0).unwrap())]
    Expected(&'static [u8]),
    #[error("At {0}: `{1}` is not an operator")]
    NoSuchOp(usize, &'a str),
    #[error("Failed to parse integer: {0}")]
    ParseIntError(#[from] ParseIntError)
}

impl<'a> Lexer<'a> {
    //Helper function to deal with lexing anywhere prior to end of string
    fn lex_non_end(&mut self)-> Result<Token<'a>, LexError<'a>> {
        let here = &self.src[self.pos..];
        match self.state {
            State::CommStr => {
                self.pos = self.src.len();
                Ok(Token::String(here.as_bytes()))
            }
            State::DelimStr(end) => {
                let c = here.as_bytes().windows(end.len())
                    .take_while(|btstr| end != *btstr).count();
                if &here.as_bytes()[c..c+end.len()] != end {
                    Err(LexError::Expected(end))
                } else {
                    Ok(Token::String(&here.as_bytes()[..c]))
                }
            }
            State::Default => {
                let mut chars = here.chars().peekable();
                let c = chars.peek().unwrap();
                if *c == '[' {
                    self.pos += 1;
                    self.state = State::DelimStr(b"]");
                    return Ok(Token::Op(b"["))
                } else if *c == '{' {
                    self.pos += 1;
                    self.state = State::DelimStr(b"}");
                    return Ok(Token::LMac)
                } else if *c == '#' {
                    self.pos += 1;
                    self.state = State::CommStr;
                    return Ok(Token::Comment)
                } else if *c == '(' {
                    self.pos += 1;
                    return Ok(Token::LPar)
                } else if *c == ')' {
                    self.pos += 1;
                    return Ok(Token::RPar)
                } else if c.is_digit(10) {
                    let len = chars.take_while(|c| c.is_digit(10)).count();
                    let num = here[..len].parse::<i64>()?;
                    self.pos += len;
                    return Ok(Token::Num(num))
                } else {
                    let maxlen = chars
                        .take_while(|c| !c.is_digit(10) && !c.is_whitespace())
                        .count();
                    let mut tok = None;
                    //try to grab an op, leftmost longest.
                    for i in 1..=maxlen {
                        let optok = here[..i].as_bytes();
                        tok = if is_op(optok) {Some((optok,i))} else {tok};
                    }
                    if let Some((optok, l)) = tok {
                        self.pos+= l;
                        Ok(Token::Op(optok))
                    } else {
                        Err(LexError::NoSuchOp(self.pos, &here[..maxlen]))
                    }
                }
            }
        }
    }
}


impl<'a> Iterator for Lexer<'a> {
    type Item = Result<Token<'a>, LexError<'a>>;
    fn next(&mut self) -> Option<Self::Item> {
        self.pos += self.src.chars().take_while(|c| c.is_whitespace()).count();
        if self.pos >= self.src.len() {
            match self.state {
                State::Default => return None,
                State::CommStr => {
                    self.state = State::Default;
                    //Technically requesting a comment and not placing one
                    //is not illegal
                    //
                    //^ case in point
                    return Some(Ok(Token::String(b"")));
                }
                State::DelimStr(s) => {
                    self.state = State::Default;
                    return Some(Err(LexError::Expected(s)));
                }
            }
        }
        Some(self.lex_non_end())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
}
