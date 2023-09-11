use crate::error_handling::*;
use crate::token::*;
use crate::token_type::*;

pub struct Lexer {
  source: Vec<char>,
  tokens: Vec<Token>,
  start: usize,
  current: usize,
  line: usize,
}

impl Lexer {
    pub fn new(source: String) -> Self {
      Self {
        source: source.chars().collect(),
        tokens: vec![],
        start: 0,
        current: 0,
        line: 1,
      }
    }

    pub fn eval_tokens(&mut self) -> Result<&Vec<Token>, LoxError> {
      while !self.end_of_source() {
        self.start = self.current;
        self.eval_token();
      }

      self.tokens.push(Token::eof(self.line));
    }
    
    fn end_of_source(&self) {
      self.current >= self.source.len();
    }

    fn eval_token(&mut self) {
      let c = self.eat();
      match c {
        "(" => self.add_token(TokenType::LeftParen),
        _   => {
          unreachable!("Unmatched token type to char!")
        }
      }
    }

    fn eat(&mut self) -> char {
      let result = *self.source.get(self.current).unwrap();
      self.current += 1;
      result
    }

    fn add_token() {
      todo!()
    }
}

