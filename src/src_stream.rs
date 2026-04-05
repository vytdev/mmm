use std::str;


const TABSTOP: usize = 8;


pub struct SrcStream<'a> {
  pub text: &'a str,
  iter: str::Chars<'a>,
  peeked: Option<char>,

  pub line: usize,
  pub col: usize,
  pub pos: usize,
  pub curr_ln: &'a str,
}


impl<'a> SrcStream<'a> {

  /**
   * Creates a new SrcStream.
   */
  pub fn new(source: &'a str) -> Self {
    SrcStream{
      text: source,
      iter: source.chars(),
      peeked: None,

      line: 1,
      col: 1,
      pos: 0,         // curr byte position in src
      curr_ln: source,
    }
  }

  /**
   * Peek to the next char.
   */
  pub fn peek_char(&mut self) -> char {
    self.peeked.unwrap_or_else(|| {
      let ch = self.iter.next().unwrap_or('\0');
      self.peeked = Some(ch);
      return ch;
    })
  }

  /**
   * Advance the lexer and get next char.
   */
  pub fn next_char(&mut self) -> char {
    if let Some(ch) = self.peeked.take() {
      self.advance(ch);
      return ch;
    }
    if let Some(ch) = self.iter.next() {
      self.advance(ch);
      return ch;
    }
    return '\0';    // sentinel
  }

  /**
   * Advance the lexer counters.
   */
  pub fn advance(&mut self, ch: char) {
    self.pos += ch.len_utf8();

    match ch {
      '\r' => {
        self.col = 1;
        self.curr_ln = &self.text[self.pos..];
      },
      '\n' => {
        self.col = 1;
        self.line += 1;
        self.curr_ln = &self.text[self.pos..];
      },
      '\t' => {
        self.col += TABSTOP - (self.col % TABSTOP);
      },
      _ => {
        self.col += 1;
      },
    }
  }

  /**
   * Check whether it's eof.
   */
  pub fn is_eof(&self) -> bool {
    self.peeked != None && self.pos >= self.text.len()
  }

}
