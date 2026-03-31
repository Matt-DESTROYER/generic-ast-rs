use generic_ast::{Parser, ExpressionElement, BinaryExpression};

#[derive(Clone, PartialEq, Debug)]
pub enum Token {
	Number(f64),
	Add,
	Subtract,
	Multiply,
	Divide,
	OpenBracket,
	CloseBracket
}

fn num(n: f64) -> ExpressionElement<Token> {
	ExpressionElement::Token(Token::Number(n))
}

fn expr(
	lhs: ExpressionElement<Token>,
	op: Token,
	rhs: ExpressionElement<Token>
) -> ExpressionElement<Token> {
	ExpressionElement::BinaryExpression(Box::new(BinaryExpression {
		lhs,
		operator: op,
		rhs
	}))
}

fn setup_math_parser() -> Parser<Token> {
	let mut parser = Parser::new();

	parser.set_open_grouper(Token::OpenBracket);
	parser.set_close_grouper(Token::CloseBracket);

	parser.add_precedence_level(&[Token::Multiply, Token::Divide]);
	parser.add_precedence_level(&[Token::Add, Token::Subtract]);

	parser
}

#[test]
fn test_standard_precedence() {
	let parser = setup_math_parser();

	// 1 + 2 * 3
	let tokens = vec![
		Token::Number(1.0),
		Token::Add,
		Token::Number(2.0),
		Token::Multiply,
		Token::Number(3.0)
	];

	let result = parser.parse(&tokens).expect("Failed to parse");

	let expected = expr(
		num(1.0),
		Token::Add,
		expr(
			num(2.0),
			Token::Multiply,
			num(3.0)
		)
	);

	assert_eq!(result, expected);
}

#[test]
fn test_grouping_overrides_precedence() {
	let parser = setup_math_parser();

	// (1 + 2) * 3
	let tokens = vec![
		Token::OpenBracket,
		Token::Number(1.0),
		Token::Add,
		Token::Number(2.0),
		Token::CloseBracket,
		Token::Multiply,
		Token::Number(3.0)
	];

	let result = parser.parse(&tokens).expect("Failed to parse");

	let expected = expr(
		expr(
			num(1.0),
			Token::Add,
			num(2.0)
		),
		Token::Multiply,
		num(3.0)
	);

	assert_eq!(result, expected);
}#[test]
fn test_left_associativity() {
	let parser = setup_math_parser();
	
	// 1 - 2 - 3
	let tokens = vec![
		Token::Number(1.0),
		Token::Subtract,
		Token::Number(2.0),
		Token::Subtract,
		Token::Number(3.0),
	];

	let result = parser.parse(&tokens).expect("Failed to parse");

	// Expected: (1 - 2) - 3
	let expected = expr(
		expr(
			num(1.0),
			Token::Subtract,
			num(2.0)
		),
		Token::Subtract,
		num(3.0)
	);

	assert_eq!(result, expected);
}

#[test]
fn test_dangling_operator_error() {
	let parser = setup_math_parser();
	
	// 1 +
	let tokens = vec![
		Token::Number(1.0),
		Token::Add,
	];

	let result = parser.parse(&tokens);

	assert!(result.is_err(), "Parser should have caught the dangling operator");
	assert_eq!(result.unwrap_err(), "Dangling operator without RHS found");
}

#[test]
fn test_unmatched_brackets_error() {
	let parser = setup_math_parser();
	
	// (1 + 2
	let tokens = vec![
		Token::OpenBracket,
		Token::Number(1.0),
		Token::Add,
		Token::Number(2.0),
	];

	let result = parser.parse(&tokens);

	assert!(result.is_err());
	assert_eq!(result.unwrap_err(), "Could not resolve unclosed grouped expression");
}
