use std::env;

use crate::lexer::Lexer;

mod lexer;

fn main() {
    if let Some(filename) = env::args().nth(1) {
        let mut lexer = Lexer::from_file(filename);
        while let Some(token) = lexer.next_token() {
            dbg!(token);
        }
    } else {
        panic!("Invalid usage. Pass a filename.");
    }
}
