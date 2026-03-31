# generic-ast-rs
A dependency-free Rust library that turns flat streams of tokens into Abstract Syntax Trees (ASTs) without writing complex grammar files. Just provide your custom `Token` type and define your order of operations, `generic-ast` handles the rest!

## Features
 - **Generic**: Works with any custom token type that implements `PartialEq` and `Clone`.
 - **Simple precedence**: Define your order of operations just by passing arrays of operators.
 - **Recursive groupings**: Handles nested parentheses and brackets out of the box.
 - **Zero deps**: A pure, lightweight Rust implementation with no external bloat.

## Quick start!

### Example: A simple math parser
Here is how you could use `generic-ast` to parse standard mathematical expressions like `2 + 3 * 4`.

```rs
use generic_ast::{Parser, ExpressionElement, Expression};

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

fn main() {
	// create the parser
	let mut parser = Parser::new();

	// tell it what tokens to use to group
	parser.set_open_grouper(Token::OpenBracket);
	parser.set_close_grouper(Token::CloseBracket);

	// tell it the order of your operations
	parser.add_precedence_level(&[Token::Multiply, Token::Divide]);
	parser.add_precedence_level(&[Token::Add, Token::Subtract]);

	// get your tokens
	let tokens = vec![
		Token::Number(2.0),
		Token::Plus,
		Token::OpenBracket,
		Token::Number(3.0),
		Token::Multiply,
		Token::Number(4.0),
		Token::CloseBracket
	];

	let ast = parser.parse(&tokens).expect("Failed to parse syntax tree");

	print!("{:#?}", ast);
}
```

## How it works
The parser has two main steps, group resolution and operator grouping.

The former (and first step) involves locating and recursively parsing groups.
The latter (and last step) involves stepping through each precedence level and combining operators with the expression or token on their left and right into expressions, gradually generating the AST.

The final output will either be a raw token or an `Expression` with the left and right hand sides containing
