#![no_std]
#[macro_use] extern crate alloc;

use core::str::Chars;
use alloc::string::String;
use alloc::vec::Vec;
use thiserror_no_std::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ParseError {
    #[error("unexpected end of file")]
    UnexpectedEof,
    #[error("unexpected end of file, function declaration not finished")]
    UnexpectedEofFuncDecl,
    #[error("unexpected character {0}")]
    UnexpectedCharacter(char),
    #[error("unexpected token {0:?}")]
    UnexpectedToken(Token),
}

#[derive(Debug, PartialEq)]
pub enum Token {
    Ignore(char),
    Eof,

    ModuleImport { path: String },

    FuncDecl { name: String, children: Vec<Token> },
    FuncCall { name: String },
    Loop { children: Vec<Token> },
    Expr(Expr),
}

impl Token {
    fn parse(code: &mut Chars) -> Result<Self, ParseError> {
        if let Some(c) = code.next() {
            // TODO: Potentially we should filter for } here, return an error and
            //       then catch this error in the func_decl loop
            match c {
                '+' => Ok(Token::Expr(Expr::Add)),
                '-' => Ok(Token::Expr(Expr::Sub)),
                '<' => Ok(Token::Expr(Expr::MoveLeft)),
                '>' => Ok(Token::Expr(Expr::MoveRight)),
                // Starts a function declaration
                ':' => {
                    let mut name = String::new();
                    'name_gather: loop {
                        if let Some(c) = code.next() {
                            if c == '{' {
                                // Actual function body starts now
                                break 'name_gather;
                            } else if c.is_ascii_alphanumeric() {
                                name.push(c);
                            } else {
                                return Err(ParseError::UnexpectedCharacter(c));
                            }
                        } else {
                            return Err(ParseError::UnexpectedEof);
                        }
                    }
                    let mut children = Vec::new();
                    'func_decl: loop {
                        let token = Self::parse(code)?;
                        match token {
                            Token::Ignore('}') => break 'func_decl,
                            Token::Ignore(_) => {},
                            Token::Eof => return Err(ParseError::UnexpectedEofFuncDecl),
                            _ => children.push(token),
                        }
                    }
                    Ok(Self::FuncDecl {
                        name,
                        children,
                    })
                },
                _ => Ok(Self::Ignore(c)),
            }
        } else {
            Ok(Self::Eof)
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Expr {
    Add,
    Sub,
    MoveRight,
    MoveLeft,
}

fn parse_inner(code: &mut Chars) -> Result<Vec<Token>, ParseError> {
    let mut r = Vec::new();
    loop {
        let token = Token::parse(code)?;
        match token {
            Token::FuncDecl { .. } | Token::ModuleImport { .. } => r.push(token),
            Token::Ignore(_) => {},
            Token::Eof => break,
            _ => return Err(ParseError::UnexpectedToken(token)),
        }
    }
    Ok(r)
}

pub fn parse(code: String) -> Result<Vec<Token>, ParseError> {
    parse_inner(&mut code.chars())
}

#[cfg(test)]
mod tests {
    extern crate std;
    use std::string::ToString;
    use super::*;

    #[test]
    fn func_decl() {
        let code = r#"
This function will add 2 numbers and return the value back to the stack
Another approach could be to simply leave the value in memory and hope the caller is okay with the memory pointer being shifted
:add{
    ,>,<    Read our 2 input values from the stack
    [>+<-]  Add the 2 numbers
    >.<     Push the result to the stack
    [-]     Clean up the result from memory
}"#.to_string();
        let expected = Ok(vec![Token::FuncDecl { name: "add".to_string(), children: vec![Token::Expr(Expr::MoveRight), Token::Expr(Expr::MoveLeft), Token::Expr(Expr::MoveRight), Token::Expr(Expr::Add), Token::Expr(Expr::MoveLeft), Token::Expr(Expr::Sub), Token::Expr(Expr::MoveRight), Token::Expr(Expr::MoveLeft), Token::Expr(Expr::Sub)] }]);
        assert_eq!(expected, parse(code));
    }

    #[test]
    fn nested_func_decl() {
        let code = r#"
:hello{
    ++
    :world{
        --
    }
}"#.to_string();
        let expected = Ok(vec![Token::FuncDecl {
            name: "hello".to_string(),
            children: vec![
                Token::Expr(Expr::Add),
                Token::Expr(Expr::Add),
                Token::FuncDecl {
                    name: "world".to_string(),
                    children: vec![
                        Token::Expr(Expr::Sub),
                        Token::Expr(Expr::Sub),
                    ]
                }
            ],
        }]);
        assert_eq!(expected, parse(code));
    }
}
