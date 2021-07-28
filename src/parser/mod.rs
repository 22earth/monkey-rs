use crate::lexer::{
    keyword::Keyword,
    punctuator::Punctuator,
    token::{Numeric, Token, TokenKind},
    Lexer,
};

use self::node::{Expression, Program, Statement};

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
impl Precedence {
    fn token_precedence(tok: &TokenKind) -> Precedence {
        match tok {
            TokenKind::Punctuator(Punctuator::Eq) => Precedence::Equals,
            TokenKind::Punctuator(Punctuator::NotEq) => Precedence::Equals,
            TokenKind::Punctuator(Punctuator::LessThan) => Precedence::LessGreater,
            TokenKind::Punctuator(Punctuator::GreaterThan) => Precedence::LessGreater,
            TokenKind::Punctuator(Punctuator::Add) => Precedence::Sum,
            TokenKind::Punctuator(Punctuator::Sub) => Precedence::Sum,
            TokenKind::Punctuator(Punctuator::Div) => Precedence::Product,
            TokenKind::Punctuator(Punctuator::Mul) => Precedence::Product,
            TokenKind::Punctuator(Punctuator::OpenParen) => Precedence::Call,
            TokenKind::Punctuator(Punctuator::OpenBracket) => Precedence::Index,
            _ => Precedence::Lowest,
        }
    }
}
type ParseError = String;
type ParseErrors = Vec<ParseError>;
type PrefixFn = fn(parser: &mut Parser<'_>) -> ParseResult<Expression>;
type InfixFn = fn(parser: &mut Parser<'_>, left: Expression) -> ParseResult<Expression>;
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
    fn infix_fn(&mut self) -> Option<InfixFn> {
        match self.peek_token.kind() {
            // TokenKind::Punctuator(Punctuator::Add) => Some(Parser::parse_infix_expression),
            _ => None,
        }
    }
    fn prefix_fn(&mut self) -> Option<PrefixFn> {
        match self.cur_token.kind() {
            TokenKind::Identifier(_) => Some(Parser::parse_identifier),
            _ => None,
        }
    }
    fn parse_identifier(parser: &mut Parser<'_>) -> ParseResult<Expression> {
        if let TokenKind::Identifier(ref name) = parser.cur_token.kind() {
            return Ok(Expression::Identifier(name.to_string()));
        }

        Err(format!(
            "unexpected error on identifier parse with {}",
            parser.cur_token
        ))
    }
    fn peek_precedence(&self) -> Precedence {
        Precedence::token_precedence(&self.peek_token.kind())
    }
    fn parse_expression(&mut self, precedence: Precedence) -> ParseResult<Expression> {
        let mut left_exp: Expression;
        if let Some(f) = self.prefix_fn() {
            left_exp = f(self)?;
        } else {
            return Err(format!(
                "no prefix parse function for {} found",
                self.cur_token
            ));
        }
        while !self.peek_token_is(&TokenKind::Punctuator(Punctuator::Semicolon))
            && precedence < self.peek_precedence()
        {
            match self.infix_fn() {
                Some(f) => {
                    self.next_token();
                    left_exp = f(self, left_exp)?;
                }
                None => return Ok(left_exp),
            }
        }
        Ok(left_exp)
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
            TokenKind::Keyword(Keyword::Return) => self.parse_return_statement(),
            _ => self.parse_expression_statement(),
            // _ => Err(format!("invalid statement token {}", self.cur_token)),
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
        let value = self.parse_expression(Precedence::Lowest)?;
        if self.peek_token_is(&TokenKind::punctuator(Punctuator::Semicolon)) {
            self.next_token();
        }
        Ok(Statement::Let(Box::new(node::LetStatement { name, value })))
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
    fn parse_return_statement(&mut self) -> ParseResult<Statement> {
        self.next_token();
        // TODO
        let value = match self.cur_token.kind() {
            TokenKind::NumericLiteral(Numeric::Integer(num)) => num.clone(),
            _ => return Err(format!("invalid number token {}", self.cur_token)),
        };
        if self.peek_token_is(&TokenKind::punctuator(Punctuator::Semicolon)) {
            self.next_token();
        }
        Ok(Statement::Return(Box::new(node::ReturnStatement {
            value: node::Expression::Integer(value),
        })))
    }
    fn parse_expression_statement(&mut self) -> ParseResult<Statement> {
        let expression = self.parse_expression(Precedence::Lowest)?;

        if self.peek_token_is(&TokenKind::Punctuator(Punctuator::Semicolon)) {
            self.next_token();
        }

        Ok(Statement::Expression(Box::new(node::ExpressionStatement {
            expression,
        })))
    }
}
