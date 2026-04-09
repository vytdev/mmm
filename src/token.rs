use crate::{
  src_stream::SrcStream,
};


#[derive(PartialEq, Eq)]
pub enum TokenType {
  UNKNOWN,
  EOF,
  IDENTIFIER,
  NUMBER,
}


pub struct Token<'a> {
  pub tt: TokenType,
  ln_text: &'a str,
  pub text: &'a str,
  pub line: usize,
  pub col: usize,
}


impl<'a> Token<'a> {

  /**
   * Creates a new token from the current state of the lexer.
   */
  pub fn from(src: &SrcStream<'a>, tt: TokenType) -> Self {
    Token{
      tt: tt,
      ln_text: src.curr_ln,
      text: &src.text[src.pos..],
      line: src.line,
      col: src.col,
    }
  }

  /**
   * Slice the token to appropriate length, in bytes.
   */
  pub fn slice(&mut self, len: usize) {
    self.text = &self.text[..len];
  }

  /**
   * Print this token. Assuming the token doesn't span multiple
   * lines.
   */
  pub fn print(&self) {
    let mut line: &str = self.ln_text;
    if let Some(pos) = self.ln_text.find('\n') {
      line = &self.ln_text[..pos];
    }

    let spaces = " ".repeat(self.col - 1);
    let mark = "^".repeat(self.text.chars().count());

    println!("{:4}| {}", self.line, line);
    println!("    | {}{}", spaces, mark);
  }

  /**
   * Makes an EOF token.
   */
  pub fn eof(src: &SrcStream<'a>) -> Self {
    Token::from(src, TokenType::EOF)
  }
}


/**
 * Sentinel unknown token.
 */
pub static UNKNOWN_TOKEN: Token = Token{
  tt: TokenType::UNKNOWN,
  ln_text: "",
  text: "",
  line: 0,
  col: 0,
};
