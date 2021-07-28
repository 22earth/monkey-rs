use crate::lexer::{
    keyword::Keyword,
    punctuator::Punctuator,
    token::{Numeric, Token, TokenKind},
    Lexer,
};

use self::node::{Program, Statement};

mod ast;
mod node;

#[cfg(test)]
mod tests;

#[derive(PartialEq, PartialOrd)]
enum Precedence {
    Lowest,
    Equals,
    LessGreater,
    Sum,
    Product,
    Prefix,
    Call,
    Index,
}
type ParseError = String;
type ParseErrors = Vec<ParseError>;
pub type ParseResult<T> = Result<T, ParseError>;
pub struct Parser<'a> {
    l: Lexer<'a>,
    cur_token: Token,
    peek_token: Token,
    // errors: ParseErrors,
}

impl<'a> Parser<'a> {
    pub fn new(mut l: Lexer<'a>) -> Parser<'a> {
        let cur_token = l.next_token();
        let peek_token = l.next_token();
        Self {
            l,
            cur_token,
            peek_token,
        }
    }
    fn next_token(&mut self) {
        self.cur_token = self.peek_token.clone();
        self.peek_token = self.l.next_token();
    }
    fn parse_program(&mut self) -> Result<Program, ParseErrors> {
        let mut program = Program::new();
        let mut errors = ParseErrors::new();
        while self.cur_token.kind != TokenKind::EOF {
            match self.parse_statement() {
                Ok(stmt) => program.body.push(stmt),
                Err(err) => errors.push(err),
            };
            self.next_token();
        }
        if errors.len() > 0 {
            return Err(errors);
        }

        Ok(program)
    }
    fn parse_statement(&mut self) -> ParseResult<Statement> {
        match self.cur_token.kind {
            TokenKind::Keyword(Keyword::Let) => self.parse_let_statement(),
            // TokenKind::Keyword(Keyword::Return) => self.parse_return_statement(),
            _ => Err(format!("invalid statement token {}", self.cur_token)),
        }
    }
    fn peek_token_is(&self, t: &TokenKind) -> bool {
        self.peek_token.kind == *t
    }
    fn cur_token_is(&self, t: &TokenKind) -> bool {
        self.cur_token.kind == *t
    }
    fn parse_let_statement(&mut self) -> ParseResult<Statement> {
        // read ident
        let name = self.expect_ident()?;
        self.expect_peek(&TokenKind::punctuator(Punctuator::Assign))?;
        self.next_token();
        // TODO
        let value = match self.cur_token.kind() {
            TokenKind::NumericLiteral(Numeric::Integer(num)) => num.clone(),
            _ => return Err(format!("invalid number token {}", self.cur_token)),
        };
        if self.peek_token_is(&TokenKind::punctuator(Punctuator::Semicolon)) {
            self.next_token();
        }
        Ok(Statement::Let(Box::new(node::LetStatement {
            name,
            value: node::Expression::Integer(value),
        })))
    }
    fn expect_ident(&mut self) -> Result<String, ParseError> {
        let name = match self.peek_token.kind() {
            TokenKind::Identifier(name) => name.to_string(),
            _ => return Err(format!("invalid identifier {}", self.peek_token)),
        };

        self.next_token();
        Ok(name)
    }
    fn expect_peek(&mut self, kind: &TokenKind) -> ParseResult<()> {
        if self.peek_token_is(kind) {
            self.next_token();
            Ok(())
        } else {
            let e = format!("expected token: {} got: {}", kind, self.cur_token);
            Err(e)
        }
    }
}
