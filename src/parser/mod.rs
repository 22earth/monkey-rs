use std::collections::HashMap;

use crate::lexer::{
    keyword::Keyword,
    punctuator::Punctuator,
    token::{Numeric, Token, TokenKind},
    Lexer,
};

use self::node::{Expression, Node, Program, Statement};

mod ast;
pub mod node;

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

pub fn parse(input: &str) -> Result<Node, ParseErrors> {
    let l = Lexer::new(input);
    let mut p = Parser::new(l);
    let prog = p.parse_program()?;

    Ok(Node::Program(Box::new(prog)))
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
            TokenKind::Punctuator(punc) => match punc {
                Punctuator::Add
                | Punctuator::Sub
                | Punctuator::Mul
                | Punctuator::Div
                | Punctuator::GreaterThan
                | Punctuator::Eq
                | Punctuator::NotEq
                | Punctuator::LessThan => Some(Parser::parse_infix_expression),
                Punctuator::OpenParen => Some(Parser::parse_call_expression),
                Punctuator::OpenBracket => Some(Parser::parse_index_expression),
                _ => None,
            },
            _ => None,
        }
    }

    fn parse_index_expression(
        parser: &mut Parser<'_>,
        left: Expression,
    ) -> ParseResult<Expression> {
        parser.next_token();

        let exp = node::IndexExpression {
            left,
            index: parser.parse_expression(Precedence::Lowest)?,
        };

        parser.expect_peek(&TokenKind::Punctuator(Punctuator::CloseBracket))?;

        Ok(Expression::Index(Box::new(exp)))
    }
    fn prefix_fn(&mut self) -> Option<PrefixFn> {
        match self.cur_token.kind() {
            TokenKind::Identifier(_) => Some(Parser::parse_identifier),
            TokenKind::NumericLiteral(_) => Some(Parser::parse_integer_literal),
            TokenKind::StringLiteral(_) => Some(Parser::parse_string_literal),
            TokenKind::Punctuator(Punctuator::Not) | TokenKind::Punctuator(Punctuator::Sub) => {
                Some(Parser::parse_prefix_expression)
            }
            TokenKind::BooleanLiteral(_) => Some(Parser::parse_boolean),
            TokenKind::Punctuator(Punctuator::OpenParen) => Some(Parser::parse_grouped_expression),
            TokenKind::Keyword(Keyword::If) => Some(Parser::parse_if_expression),
            TokenKind::Keyword(Keyword::Function) => Some(Parser::parse_function_literal),
            TokenKind::Punctuator(Punctuator::OpenBracket) => Some(Parser::parse_array_literal),
            TokenKind::Punctuator(Punctuator::OpenBlock) => Some(Parser::parse_hash_literal),
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
    fn cur_precedence(&self) -> Precedence {
        Precedence::token_precedence(&self.cur_token.kind())
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
    pub fn parse_program(&mut self) -> Result<Program, ParseErrors> {
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
        let value = self.parse_expression(Precedence::Lowest)?;
        if self.peek_token_is(&TokenKind::punctuator(Punctuator::Semicolon)) {
            self.next_token();
        }
        Ok(Statement::Return(Box::new(node::ReturnStatement { value })))
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
    fn parse_integer_literal(parser: &mut Parser<'_>) -> ParseResult<Expression> {
        if let TokenKind::NumericLiteral(Numeric::Integer(value)) = parser.cur_token.kind() {
            return Ok(Expression::Integer(*value));
        }

        Err(format!(
            "error parsing integer literal {}",
            parser.cur_token
        ))
    }
    fn parse_string_literal(parser: &mut Parser<'_>) -> ParseResult<Expression> {
        if let TokenKind::StringLiteral(ref s) = parser.cur_token.kind() {
            return Ok(Expression::String(s.to_string()));
        }

        Err(format!(
            "unexpected error on string parse with {}",
            parser.cur_token
        ))
    }
    fn parse_boolean(parser: &mut Parser<'_>) -> ParseResult<Expression> {
        match parser.cur_token.kind() {
            TokenKind::BooleanLiteral(v) => Ok(Expression::Boolean(*v)),
            _ => panic!("couldn't parse {:?} to boolean", parser.cur_token),
        }
    }
    fn parse_prefix_expression(parser: &mut Parser<'_>) -> ParseResult<Expression> {
        let operator = parser.cur_token.kind().clone();

        parser.next_token();

        let right = parser.parse_expression(Precedence::Prefix)?;

        Ok(Expression::Prefix(Box::new(node::PrefixExpression {
            operator,
            right,
        })))
    }
    fn parse_infix_expression(
        parser: &mut Parser<'_>,
        left: Expression,
    ) -> ParseResult<Expression> {
        let operator = parser.cur_token.kind().clone();
        let precedence = parser.cur_precedence();

        parser.next_token();

        let right = parser.parse_expression(precedence)?;

        Ok(Expression::Infix(Box::new(node::InfixExpression {
            operator,
            left,
            right,
        })))
    }
    fn parse_grouped_expression(parser: &mut Parser<'_>) -> ParseResult<Expression> {
        parser.next_token();
        let exp = parser.parse_expression(Precedence::Lowest);
        parser.expect_peek(&TokenKind::Punctuator(Punctuator::CloseParen))?;

        exp
    }
    fn parse_if_expression(parser: &mut Parser<'_>) -> ParseResult<Expression> {
        parser.expect_peek(&TokenKind::Punctuator(Punctuator::OpenParen))?;
        parser.next_token();
        let condition = parser.parse_expression(Precedence::Lowest)?;

        parser.expect_peek(&TokenKind::Punctuator(Punctuator::CloseParen))?;
        parser.expect_peek(&TokenKind::Punctuator(Punctuator::OpenBlock))?;
        let consequence = parser.parse_block_statement()?;

        let alternative = if parser.peek_token_is(&TokenKind::Keyword(Keyword::Else)) {
            parser.next_token();

            parser.expect_peek(&TokenKind::Punctuator(Punctuator::OpenBlock))?;

            let alt_block = parser.parse_block_statement()?;
            Some(alt_block)
        } else {
            None
        };

        Ok(Expression::If(Box::new(node::IfExpression {
            condition,
            consequence,
            alternative,
        })))
    }
    fn parse_block_statement(&mut self) -> ParseResult<node::BlockStatement> {
        let mut statements = Vec::new();

        self.next_token();

        while !self.cur_token_is(&TokenKind::Punctuator(Punctuator::CloseBlock))
            && !self.cur_token_is(&TokenKind::EOF)
        {
            if let Ok(stmt) = self.parse_statement() {
                statements.push(stmt);
            }
            self.next_token();
        }

        Ok(node::BlockStatement { statements })
    }
    fn parse_function_literal(parser: &mut Parser<'_>) -> ParseResult<Expression> {
        parser.expect_peek(&TokenKind::Punctuator(Punctuator::OpenParen))?;
        let parameters = parser.parse_function_parameters()?;

        parser.expect_peek(&TokenKind::Punctuator(Punctuator::OpenBlock))?;

        let body = parser.parse_block_statement()?;

        Ok(Expression::Function(Box::new(node::FunctionLiteral {
            parameters,
            body,
        })))
    }
    fn parse_function_parameters(&mut self) -> Result<Vec<node::IdentifierExpression>, ParseError> {
        let mut identifiers: Vec<node::IdentifierExpression> = Vec::new();

        if self.peek_token_is(&TokenKind::Punctuator(Punctuator::CloseParen)) {
            self.next_token();
            return Ok(identifiers);
        }

        self.next_token();

        identifiers.push(self.parse_identifier_into_identifier_expression()?);

        while self.peek_token_is(&TokenKind::Punctuator(Punctuator::Comma)) {
            self.next_token();
            self.next_token();
            identifiers.push(self.parse_identifier_into_identifier_expression()?);
        }

        self.expect_peek(&TokenKind::Punctuator(Punctuator::CloseParen))?;

        Ok(identifiers)
    }
    fn parse_identifier_into_identifier_expression(
        &mut self,
    ) -> ParseResult<node::IdentifierExpression> {
        if let TokenKind::Identifier(ref name) = self.cur_token.kind() {
            return Ok(node::IdentifierExpression {
                name: name.to_string(),
            });
        }

        Err(format!(
            "unexpected error on identifier parse with {}",
            self.cur_token
        ))
    }
    fn parse_call_expression(
        parser: &mut Parser<'_>,
        function: Expression,
    ) -> ParseResult<Expression> {
        let arguments =
            parser.parse_expression_list(TokenKind::Punctuator(Punctuator::CloseParen))?;
        Ok(Expression::Call(Box::new(node::CallExpression {
            function,
            arguments,
        })))
    }
    fn parse_expression_list(&mut self, end: TokenKind) -> ParseResult<Vec<Expression>> {
        let mut list: Vec<Expression> = Vec::new();

        if self.peek_token_is(&end) {
            self.next_token();
            return Ok(list);
        }

        self.next_token();
        list.push(self.parse_expression(Precedence::Lowest)?);

        while self.peek_token_is(&TokenKind::Punctuator(Punctuator::Comma)) {
            self.next_token();
            self.next_token();
            list.push(self.parse_expression(Precedence::Lowest)?);
        }

        self.expect_peek(&end)?;

        Ok(list)
    }
    fn parse_array_literal(parser: &mut Parser<'_>) -> ParseResult<Expression> {
        let elements =
            parser.parse_expression_list(TokenKind::Punctuator(Punctuator::CloseBracket))?;
        Ok(Expression::Array(Box::new(node::ArrayLiteral { elements })))
    }
    fn parse_hash_literal(parser: &mut Parser<'_>) -> ParseResult<Expression> {
        let mut pairs: HashMap<Expression, Expression> = HashMap::new();

        while !parser.peek_token_is(&TokenKind::Punctuator(Punctuator::CloseBlock)) {
            parser.next_token();
            let key = parser.parse_expression(Precedence::Lowest)?;

            parser.expect_peek(&TokenKind::Punctuator(Punctuator::Colon))?;
            parser.next_token();
            let value = parser.parse_expression(Precedence::Lowest)?;

            pairs.insert(key, value);

            if !parser.peek_token_is(&TokenKind::Punctuator(Punctuator::CloseBlock)) {
                parser.expect_peek(&TokenKind::Punctuator(Punctuator::Comma))?;
            }
        }

        parser.expect_peek(&TokenKind::Punctuator(Punctuator::CloseBlock))?;

        Ok(Expression::Hash(Box::new(node::HashLiteral { pairs })))
    }
}
