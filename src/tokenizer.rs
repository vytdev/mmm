use crate::src_stream::SrcStream;
use crate::token::{Token, TokenType};


pub fn parse_next<'a>(src: &mut SrcStream<'a>) -> Token<'a> {
  // TODO: actual parsing
  skip_whitespace(src);
  if src.is_eof() { Token::eof(src) }
  else { parse_unknown(src) }
}


pub fn skip_whitespace(src: &mut SrcStream) {
  while !src.is_eof() {
    let ch = src.peek_char();
    if !ch.is_whitespace() { break }
    src.next_char();
  }
}


pub fn parse_unknown<'a>(src: &mut SrcStream<'a>) -> Token<'a> {
  let mut token = Token::from(src, TokenType::UNKNOWN);
  let mut blen: usize = 0;
  while !src.is_eof() {
    let ch = src.peek_char();
    if ch.is_whitespace() { break }
    blen += ch.len_utf8();
    src.next_char();
  }
  token.slice(blen);
  return token;
}
