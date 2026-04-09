use std::ops::Range;

#[derive(Clone, Debug, PartialEq)]
pub enum OpType<Token: PartialEq + Clone> {
	Unary(Token),
	Binary(Token)
}
impl<Token: PartialEq + Clone> OpType<Token> {
	pub fn to_token(&self) -> Token {
		match self {
			OpType::Unary(token) => token.clone(),
			OpType::Binary(token) => token.clone()
		}
	}
}

#[derive(Clone, Debug, PartialEq)]
pub enum ExpressionElement<Token: PartialEq + Clone> {
	UnaryExpression(Box<UnaryExpression<Token>>),
	BinaryExpression(Box<BinaryExpression<Token>>),
	Token(Token)
}
impl<Token: PartialEq + Clone> ExpressionElement<Token> {
	pub fn is_token(&self) -> bool {
		if let ExpressionElement::Token(_) = self {
			true
		} else {
			false
		}
	}
}

#[derive(Clone, Debug, PartialEq)]
pub struct UnaryExpression<Token: PartialEq + Clone> {
	pub operator: Token,
	pub token: ExpressionElement<Token>
}

#[derive(Clone, Debug, PartialEq)]
pub struct BinaryExpression<Token: PartialEq + Clone> {
	pub lhs: ExpressionElement<Token>,
	pub operator: Token,
	pub rhs: ExpressionElement<Token>
}

pub struct Parser<Token: PartialEq + Clone> {
	operators: Vec<Vec<OpType<Token>>>,
	open_grouper: Option<Token>,
	close_grouper: Option<Token>
}
impl<Token: PartialEq + Clone> Parser<Token> {
	pub fn new() -> Self {
		Self {
			operators: Vec::new(),
			open_grouper: None,
			close_grouper: None
		}
	}
	pub fn init(operators: Vec<Vec<OpType<Token>>>, open_grouper: Token, close_grouper: Token) -> Self {
		Self {
			operators,
			open_grouper: Some(open_grouper),
			close_grouper: Some(close_grouper)
		}
	}

	pub fn add_precedence_level(&mut self, operators: &[OpType<Token>]) {
		let operators: Vec<OpType<Token>> = operators.to_vec();
		self.operators.push(operators);
	}
	pub fn set_open_grouper(&mut self, token: Token) {
		self.open_grouper = Some(token.clone());
	}
	pub fn set_close_grouper(&mut self, token: Token) {
		self.close_grouper = Some(token.clone());
	}

	fn is_operator(&self, token: &Token) -> bool {
		// iterate through every individual operator
		for precedence in &self.operators {
			for operator in precedence {
				// unwrap operator's inner token
				let operator = match operator {
					OpType::Unary(token) => token,
					OpType::Binary(token) => token
				};

				if token == operator {
					return true;
				}
			}
		}

		false
	}

	fn find_next_group(&self, expression_list: &mut Vec<ExpressionElement<Token>>) -> Result<Option<Range<usize>>, String> {
		let open_grouper = match &self.open_grouper {
			Some(grouper) => grouper,
			None => return Ok(None)
		};
		let close_grouper = match &self.close_grouper {
			Some(grouper) => grouper,
			None => return Ok(None)
		};

		let mut i = 0;

		while i < expression_list.len() {
			match &expression_list[i] {
				ExpressionElement::Token(token) => {
					let token = token.clone();
					if token == *open_grouper {
						let start_idx: usize = i;
						i += 1;
						let mut inner_counter: usize = 1;
						loop {
							if i >= expression_list.len() {
								return Err("Could not resolve unclosed grouped expression".to_owned());
							}

							if let ExpressionElement::Token(token) = &expression_list[i] && *token == *open_grouper {
								inner_counter += 1;
							} else if let ExpressionElement::Token(token) = &expression_list[i] && *token == *close_grouper {
								inner_counter -= 1;
							}

							if inner_counter == 0 {
								break;
							}

							i += 1;
						}

						if let ExpressionElement::Token(token) = &expression_list[i] && *token != *close_grouper {
							continue;
						}

						return Ok(Some(start_idx..i + 1));
					}
				}
				_ => {
					i += 1;
					continue
				}
			}

			i += 1;
		}

		Ok(None)
	}

	fn generate_expressions_at_level(&self, expression_list: &mut Vec<ExpressionElement<Token>>, operators: &[OpType<Token>]) -> Result<(), String> {
		if expression_list.len() < 1 { return Ok(()) }

		let mut i = 0;
		while i < expression_list.len() {
			// extract the current token
			if let ExpressionElement::Token(token) = &expression_list[i] {
				let token = token.clone();

				// if a valid operator is found
				// move the surrounding elements (tokens _OR_ expressions) into a single expression
				let unary_context_guard = if i == 0 {
					true
				} else if let ExpressionElement::Token(token) = &expression_list[i - 1] {
					self.is_operator(token)
				} else {
					false
				};
				if operators.contains(&OpType::Unary(token.clone())) && unary_context_guard {
					if i == expression_list.len() - 1 {
						return Err("Dangling unary operator without RHS found".to_owned())
					}
					let slice: Vec<ExpressionElement<Token>> = expression_list.drain(i..i+2).collect();
					let expression = ExpressionElement::UnaryExpression(Box::new(UnaryExpression {
						operator: token,
						token: slice[1].clone()
					}));
					expression_list.insert(i, expression);
				} else if operators.contains(&OpType::Binary(token.clone())) {
					if i == 0 {
						return Err("Dangling binary operator without LHS found".to_owned())
					} else if i == expression_list.len() - 1 {
						return Err("Dangling binary operator without RHS found".to_owned());
					}
					i -= 1;

					let slice: Vec<ExpressionElement<Token>> = expression_list.drain(i..i+3).collect();
					let expression = ExpressionElement::BinaryExpression(Box::new(BinaryExpression {
						lhs: slice[0].clone(),
						operator: token,
						rhs: slice[2].clone()
					}));
					expression_list.insert(i, expression);
				}
			}

			i += 1;
		}

		Ok(())
	}

	fn internal_parse(&self, expression_list: &mut Vec<ExpressionElement<Token>>) -> Result<ExpressionElement<Token>, String> {
		// handle groups by finding each group and parsing them first
		loop {
			let next_group = match self.find_next_group(expression_list) {
				Ok(range) => match range {
					Some(range) => range,
					None => break
				},
				Err(err) => return Err(err)
			};
			let start = next_group.start;

			if next_group.end - next_group.start < 3 {
				return Err("Found parenthesis without internal expression".to_owned());
			}
			
			// include grouper operators to remove from tokens
			let grouped_tokens: Vec<ExpressionElement<Token>> = expression_list.drain(next_group).collect();
			// remove grouper operators when evaluating inner expression
			let mut grouped_tokens: Vec<ExpressionElement<Token>> = grouped_tokens[1..&grouped_tokens.len() - 1].to_vec();

			match self.internal_parse(&mut grouped_tokens) {
				Ok(expression) => {
					expression_list.insert(start, expression);
				},
				Err(err) => return Err(err)
			};
		}

		// create AST by going through each precedence level and generating expressions where operators are found
		for precedence in &self.operators {
			self.generate_expressions_at_level(expression_list, &precedence)?;
		}

		Ok(expression_list[0].clone())
	}

	pub fn parse(&self, tokens: &[Token]) -> Result<ExpressionElement<Token>, String> {
		if tokens.len() == 0 { return Err("No tokens provided".to_owned()) }

		let mut expression_list: Vec<ExpressionElement<Token>> = tokens.iter()
			.map(|token| ExpressionElement::Token(token.clone())).collect();

		self.internal_parse(&mut expression_list)
	}
}
