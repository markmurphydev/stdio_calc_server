//! Parse postfix calculator

use std::{
    iter::{Peekable, Rev},
    str::Chars,
    vec,
};

use shared::Expr;

#[derive(Debug, Clone, Copy)]
enum Token {
    Int(i64),
    Add,
    Sub,
    Mul,
    Div,
}

pub struct Parser {
    /// Iterates _backwards_ over tokens
    tokens_rev: Peekable<vec::IntoIter<Token>>,
}

struct ReverseTokenizer<'text> {
    chars: Peekable<Rev<Chars<'text>>>,
}

impl Parser {
    pub fn new(text: &str) -> anyhow::Result<Self> {
        let tokens = ReverseTokenizer::new(text)
            .tokenize()?
            .into_iter()
            .peekable();
        Ok(Self { tokens_rev: tokens })
    }

    pub fn parse(mut self) -> anyhow::Result<Option<Expr>> {
        if self.tokens_rev.peek().is_none() {
            return Ok(None);
        }

        let res = self.expr()?;
        if let Some(_remaining) = self.tokens_rev.next() {
            Err(anyhow::Error::msg(
                "Tokens remaining after expression parsed",
            ))
        } else {
            Ok(Some(res))
        }
    }

    fn expr(&mut self) -> anyhow::Result<Expr> {
        let res = match self
            .tokens_rev
            .next()
            .ok_or(anyhow::Error::msg("No next token in expr"))?
        {
            Token::Int(val) => Expr::Int(val),
            Token::Add => {
                // Tokens are in reverse order
                let second = self.expr()?;
                let first = self.expr()?;
                Expr::Add(Box::new(first), Box::new(second))
            }
            Token::Sub => {
                // Tokens are in reverse order
                let second = self.expr()?;
                let first = self.expr()?;
                Expr::Sub(Box::new(first), Box::new(second))
            }
            Token::Mul => {
                // Tokens are in reverse order
                let second = self.expr()?;
                let first = self.expr()?;
                Expr::Mul(Box::new(first), Box::new(second))
            }
            Token::Div => {
                // Tokens are in reverse order
                let second = self.expr()?;
                let first = self.expr()?;
                Expr::Div(Box::new(first), Box::new(second))
            }
        };
        Ok(res)
    }
}

impl<'text> ReverseTokenizer<'text> {
    pub fn new(text: &'text str) -> Self {
        Self {
            chars: text.chars().rev().peekable(),
        }
    }

    pub fn tokenize(mut self) -> anyhow::Result<Vec<Token>> {
        let mut tokens_rev = Vec::new();
        loop {
            self.skip_whitespace();

            let Some(char) = self.chars.peek() else {
                break;
            };

            let tok = match char {
                '+' => {
                    self.chars.next();
                    Token::Add
                }
                '-' => {
                    self.chars.next();
                    Token::Sub
                }
                '*' => {
                    self.chars.next();
                    Token::Mul
                }
                '/' => {
                    self.chars.next();
                    Token::Div
                }
                _ => self.int()?,
            };
            tokens_rev.push(tok);
        }

        Ok(tokens_rev)
    }

    fn int(&mut self) -> anyhow::Result<Token> {
        let mut int_str = String::new();
        loop {
            match self.chars.peek() {
                Some(&ch) if ch.is_ascii_digit() => int_str.push(self.chars.next().unwrap()),
                _ => break,
            }
        }

        let int = i64::from_str_radix(&int_str, 10)?;
        Ok(Token::Int(int))
    }

    fn skip_whitespace(&mut self) {
        loop {
            match self.chars.peek() {
                Some(ch) if ch.is_whitespace() => {
                    self.chars.next();
                }
                _ => return,
            }
        }
    }
}
