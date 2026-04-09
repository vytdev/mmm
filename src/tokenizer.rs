use crate::{
  src_stream::SrcStream,
  token::Token,
  token::TokenType,
};


pub fn parse_next<'a>(src: &mut SrcStream<'a>) -> Token<'a> {
  // TODO: actual parsing
  skip_whitespace(src);
  if src.is_eof() {
    Token::eof(src)
  } else {
    let ch = src.peek_char();
    if ch.is_alphabetic() || ch == '_' {
      parse_name(src)
    } else if ch.is_numeric() {
      parse_number(src)
    } else if "+-*/".contains(ch) {
      parse_operator(src)
    } else {
      parse_unknown(src)
    }
  }
}


pub fn skip_whitespace(src: &mut SrcStream) {
  while !src.is_eof() {
    let ch = src.peek_char();
    if !ch.is_whitespace() { break }
    src.next_char();
  }
}


pub fn include_chars(src: &mut SrcStream, tok: &mut Token,
                     check: fn(char) -> bool)
{
  let mut blen: usize = 0;
  while !src.is_eof() {
    let ch = src.peek_char();
    if !check(ch) { break }
    blen += ch.len_utf8();
    src.next_char();
  }
  tok.slice(blen);
}


pub fn parse_unknown<'a>(src: &mut SrcStream<'a>) -> Token<'a> {
  let mut token = Token::from(src, TokenType::UNKNOWN);
  include_chars(src, &mut token, |ch| !ch.is_whitespace());
  return token;
}


pub fn parse_name<'a>(src: &mut SrcStream<'a>) -> Token<'a> {
  let mut token = Token::from(src, TokenType::IDENTIFIER);
  include_chars(src, &mut token, |ch| ch.is_alphanumeric() || ch == '_');
  return token;
}


pub fn parse_number<'a>(src: &mut SrcStream<'a>) -> Token<'a> {
  let mut token = Token::from(src, TokenType::NUMBER);
  include_chars(src, &mut token, |ch| ch.is_numeric());
  return token;
}


pub fn parse_operator<'a>(src: &mut SrcStream<'a>) -> Token<'a> {
  let mut token = Token::from(src, TokenType::UNKNOWN);
  let mut ch = src.next_char();
  token.tt = match ch {
    '+' => { TokenType::PLUS },
    '-' => { TokenType::DASH },
    '*' => { TokenType::STAR },
    '/' => { TokenType::SLASH },
    _   => { TokenType::UNKNOWN },
  };
  token.slice(1);
  return token;
}
