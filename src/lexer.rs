use std::{
  collections::VecDeque,
};

use crate::{
  src_stream::SrcStream,
  token::Token,
  token::UNKNOWN_TOKEN,
  tokenizer::parse_next,
};


pub struct Lexer<'a> {
  src: SrcStream<'a>,
  queue: VecDeque<Token<'a>>,
  curr: Option<Token<'a>>,
}


impl<'a> Lexer<'a> {

  /**
   * Creates a new lexer.
   */
  pub fn new(source: &'a str) -> Self {
    Lexer{
      src: SrcStream::new(source),
      queue: VecDeque::new(),
      curr: None,
    }
  }

  /**
   * Push new token.
   */
  pub fn push(&mut self, token: Token<'a>) {
    self.queue.push_back(token)
  }

  /**
   * Peek next token.
   */
  pub fn peek(&mut self) -> &Token<'a> {
    self.produce_if_empty();
    self.queue.front().unwrap_or(&UNKNOWN_TOKEN)
  }

  /**
   * Consume next token.
   */
  pub fn consume(&mut self) -> &Token<'a> {
    self.produce_if_empty();
    self.curr = self.queue.pop_front();
    self.curr.as_ref().unwrap_or(&UNKNOWN_TOKEN)
  }

  /**
   * The last consumed token.
   */
  pub fn current(&self) -> &Token<'a> {
    self.curr.as_ref().unwrap_or(&UNKNOWN_TOKEN)
  }

  /**
   * Produce a token if queue is empty.
   */
  fn produce_if_empty(&mut self) {
    if self.queue.is_empty() {
      let token = parse_next(&mut self.src);
      self.push(token);
    }
  }

}
