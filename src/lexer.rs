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
        match self.eval_token() {
          Ok(_) => {},
          Err(e) => {
            e.report("".to_string())
          }
        }
      }

      self.tokens.push(Token::eof(self.line));

      Ok(&self.tokens)
    }

    fn end_of_source(&self) -> bool {
      self.current >= self.source.len()
    }

    fn eval_token(&mut self) -> Result<(), LoxError>{
      let c = self.eat();
      match c {
        '(' => self.add_token(TokenType::LeftParen),
        ')' => self.add_token(TokenType::RightParen),
        '{' => self.add_token(TokenType::LeftBrace),
        '}' => self.add_token(TokenType::RightBrace),
        ',' => self.add_token(TokenType::Comma),
        '.' => self.add_token(TokenType::Dot),
        '-' => self.add_token(TokenType::Minus),
        '+' => self.add_token(TokenType::Plus),
        ';' => self.add_token(TokenType::Semicolon),
        '/' => self.add_token(TokenType::Slash),
        '*' => self.add_token(TokenType::Star),
        _   => {
          return Err(LoxError::error(
            self.line,
            "Unexpected character".to_string()
          ));
        }
      }

      Ok(())
    }

    fn eat(&mut self) -> char {
      let result = *self.source.get(self.current).unwrap();
      self.current += 1;
      result
    }

    fn add_token(&mut self, ttype: TokenType) {
      self.add_token_object(ttype, None)
    }

    fn add_token_object(&mut self, ttype: TokenType, literal: Option<Object>) {
      let lexeme: String = self.source[self.start..self.current].iter().collect();
      self.tokens.push(Token::new(ttype, lexeme, literal, self.line))
    }
}

