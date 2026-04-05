#![allow(unused)]    // temporary

mod lexer;
mod src_stream;
mod token;
mod tokenizer;

use std::fs;
use crate::token::{Token, TokenType};
use crate::lexer::Lexer;


fn main() {
  let args: Vec<String> = std::env::args().collect();

  if args.len() < 2 {
    println!("usage: {} FILE", args[0]);
    return
  }

  let fs_read = fs::read_to_string(&args[1]);

  match fs_read {
    Ok(src) => process(src),
    Err(_) => println!("Could not read file {}", &args[1]),
  }
}


fn process(source: String) {
  let mut lexer = Lexer::new(&source);
  let mut token: &Token;

  // print tokens
  loop {
    token = lexer.consume();
    token.print();
    if token.tt == TokenType::EOF {
      break;
    }
  }
}
