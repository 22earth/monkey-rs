use super::{keyword::Keyword, punctuator::Punctuator, token::TokenKind, Lexer};

fn expect_tokens(lexer: &mut Lexer, expected: &[TokenKind]) {
    for expect in expected.iter() {
        assert_eq!(&lexer.next_token().kind(), &expect);
    }

    assert_eq!(*lexer.next_token().kind(), TokenKind::EOF);
}

#[test]
fn check_string() {
    let s = "\"bbb\"";
    let mut lexer = Lexer::new(s);

    let expected = [TokenKind::string_literal("bbb")];

    expect_tokens(&mut lexer, &expected);
}

#[test]
fn test_lexer() {
    let s = r"let five = 5;
let ten = 10;

let add = fn(x, y) {
  x + y;
};

let result = add(five, ten);
!-/*5;
5 < 10 > 5;

if (5 < 10) {
	return true;
} else {
	return false;
}

10 == 10;
10 != 9;

";
    let mut lexer = Lexer::new(s);
    let expected = [
        TokenKind::keyword(Keyword::Let),
        TokenKind::identifier("five"),
        TokenKind::punctuator(Punctuator::Assign),
        TokenKind::numeric_literal(5),
        TokenKind::punctuator(Punctuator::Semicolon),
        TokenKind::keyword(Keyword::Let),
        TokenKind::identifier("ten"),
        TokenKind::punctuator(Punctuator::Assign),
        TokenKind::numeric_literal(10),
        TokenKind::punctuator(Punctuator::Semicolon),
        TokenKind::keyword(Keyword::Let),
        TokenKind::identifier("add"),
        TokenKind::punctuator(Punctuator::Assign),
        TokenKind::keyword(Keyword::Function),
        TokenKind::punctuator(Punctuator::OpenParen),
        TokenKind::identifier("x"),
        TokenKind::punctuator(Punctuator::Comma),
        TokenKind::identifier("y"),
        TokenKind::punctuator(Punctuator::CloseParen),
        TokenKind::punctuator(Punctuator::OpenBlock),
        TokenKind::identifier("x"),
        TokenKind::punctuator(Punctuator::Add),
        TokenKind::identifier("y"),
        TokenKind::punctuator(Punctuator::Semicolon),
        TokenKind::punctuator(Punctuator::CloseBlock),
        TokenKind::punctuator(Punctuator::Semicolon),
        TokenKind::keyword(Keyword::Let),
        TokenKind::identifier("result"),
        TokenKind::punctuator(Punctuator::Assign),
        TokenKind::identifier("add"),
        TokenKind::punctuator(Punctuator::OpenParen),
        TokenKind::identifier("five"),
        TokenKind::punctuator(Punctuator::Comma),
        TokenKind::identifier("ten"),
        TokenKind::punctuator(Punctuator::CloseParen),
        TokenKind::punctuator(Punctuator::Semicolon),
        TokenKind::punctuator(Punctuator::Not),
        TokenKind::punctuator(Punctuator::Sub),
        TokenKind::punctuator(Punctuator::Div),
        TokenKind::punctuator(Punctuator::Mul),
        TokenKind::numeric_literal(5),
        TokenKind::punctuator(Punctuator::Semicolon),
        TokenKind::numeric_literal(5),
        TokenKind::punctuator(Punctuator::LessThan),
        TokenKind::numeric_literal(10),
        TokenKind::punctuator(Punctuator::GreaterThan),
        TokenKind::numeric_literal(5),
        TokenKind::punctuator(Punctuator::Semicolon),
        TokenKind::keyword(Keyword::If),
        TokenKind::punctuator(Punctuator::OpenParen),
        TokenKind::numeric_literal(5),
        TokenKind::punctuator(Punctuator::LessThan),
        TokenKind::numeric_literal(10),
        TokenKind::punctuator(Punctuator::CloseParen),
        TokenKind::punctuator(Punctuator::OpenBlock),
        TokenKind::keyword(Keyword::Return),
        TokenKind::boolean_literal(true),
        TokenKind::punctuator(Punctuator::Semicolon),
        TokenKind::punctuator(Punctuator::CloseBlock),
        TokenKind::keyword(Keyword::Else),
        TokenKind::punctuator(Punctuator::OpenBlock),
        TokenKind::keyword(Keyword::Return),
        TokenKind::boolean_literal(false),
        TokenKind::punctuator(Punctuator::Semicolon),
        TokenKind::punctuator(Punctuator::CloseBlock),
        TokenKind::numeric_literal(10),
        TokenKind::punctuator(Punctuator::Eq),
        TokenKind::numeric_literal(10),
        TokenKind::punctuator(Punctuator::Semicolon),
        TokenKind::numeric_literal(10),
        TokenKind::punctuator(Punctuator::NotEq),
        TokenKind::numeric_literal(9),
        TokenKind::punctuator(Punctuator::Semicolon),
    ];
    expect_tokens(&mut lexer, &expected);
}
