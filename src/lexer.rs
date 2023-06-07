use std::fs;

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub enum Token {
    // Keywords
    KwLet,
    KwFn,
    KwVoid,
    KwTrue,
    KwFalse,
    KwIf,
    KwElse,
    KwWhile,
    KwReturn,
    KwBreak,

    // Literals
    NumLiteral(String),
    StrLiteral(String),

    // Operators
    OpPlus,
    OpMinus,
    OpMult,
    OpDiv,
    OpMod,
    OpAnd,
    OpOr,
    OpNot,
    OpGt,
    OpGe,
    OpEq,
    OpNe,
    OpLt,
    OpLe,

    // Delimitters
    SemiColon,
    Colon,
    Comma,
    Assignment,
    Lparen,
    RParen,
    LSquirly,
    RSquirly,
    LBracket,
    RBracket,

    // Others
    Identifier(String),
    Comment(String),
    TokEof,
}

pub struct Lexer {
    position: usize,
    read_position: usize,
    ch: Option<u8>,
    input: Vec<u8>,
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        let mut lex = Lexer {
            position: 0,
            read_position: 0,
            ch: None,
            input: input.into_bytes(),
        };
        lex.read_char();

        lex
    }

    pub fn from_file(file_path: String) -> Lexer {
        let contents = fs::read_to_string(file_path).expect("Passed file does not exist.");
        Lexer::new(contents)
    }

    pub fn next_token(&mut self) -> Option<Token> {
        self.skip_whitespace();

        let tok = self.ch.and_then(|ch| match ch {
            b'+' => Some(Token::OpPlus),
            b'-' => Some(Token::OpMinus),
            b'*' => Some(Token::OpMult),
            b'/' => Some(Token::OpDiv),
            b'%' => Some(Token::OpMod),
            b'&' => Some(Token::OpAnd),
            b'|' => Some(Token::OpOr),
            b'>' => self.peek().map(|next_ch| {
                if next_ch == b'=' {
                    self.read_char();
                    Token::OpGe
                } else {
                    Token::OpGt
                }
            }),
            b'=' => self.peek().map(|next_ch| {
                if next_ch == b'=' {
                    self.read_char();
                    Token::OpEq
                } else {
                    Token::Assignment
                }
            }),
            b'!' => self.peek().map(|next_ch| {
                if next_ch == b'=' {
                    self.read_char();
                    Token::OpNe
                } else {
                    Token::OpNot
                }
            }),
            b'<' => self.peek().map(|next_ch| {
                if next_ch == b'=' {
                    self.read_char();
                    Token::OpLe
                } else {
                    Token::OpLt
                }
            }),
            b';' => Some(Token::SemiColon),
            b':' => Some(Token::Colon),
            b',' => Some(Token::Comma),
            b'(' => Some(Token::Lparen),
            b')' => Some(Token::RParen),
            b'{' => Some(Token::LSquirly),
            b'}' => Some(Token::RSquirly),
            b'[' => Some(Token::LBracket),
            b']' => Some(Token::RBracket),
            b'?' => Some(Token::Comment(self.read_comment())),
            b'a'..=b'z' | b'A'..=b'Z' | b'_' => {
                let ident = self.read_identifier();
                Some(match ident.as_str() {
                    "let" => Token::KwLet,
                    "fn" => Token::KwFn,
                    "void" => Token::KwVoid,
                    "true" => Token::KwTrue,
                    "false" => Token::KwFalse,
                    "if" => Token::KwIf,
                    "else" => Token::KwElse,
                    "while" => Token::KwWhile,
                    "return" => Token::KwReturn,
                    "break" => Token::KwBreak,
                    _ => Token::Identifier(ident),
                })
            }
            b'0'..=b'9' => Some(Token::NumLiteral(self.read_num())),
            _ => None,
        });

        self.read_char();
        tok
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = None;
        } else {
            self.ch = Some(self.input[self.read_position]);
        }

        self.position = self.read_position;
        self.read_position += 1;
    }

    fn peek(&mut self) -> Option<u8> {
        if self.read_position >= self.input.len() {
            return None;
        }

        Some(self.input[self.read_position])
    }

    fn skip_whitespace(&mut self) {
        while self.ch.filter(|&ch| ch.is_ascii_whitespace()).is_some() {
            self.read_char()
        }
    }

    fn read_identifier(&mut self) -> String {
        let start_pos = self.position;

        while self
            .peek()
            .filter(|&ch| ch.is_ascii_alphabetic() || ch == b'_')
            .is_some()
        {
            self.read_char();
        }

        return String::from_utf8_lossy(&self.input[start_pos..=self.position]).to_string();
    }

    fn read_num(&mut self) -> String {
        let start_pos = self.position;
        while self.peek().filter(|&ch| ch.is_ascii_digit()).is_some() {
            self.read_char();
        }

        if self.peek().filter(|&ch| ch == b'.').is_some() {
            self.read_char();
            while self.peek().filter(|&ch| ch.is_ascii_digit()).is_some() {
                self.read_char();
            }
        }

        return String::from_utf8_lossy(&self.input[start_pos..=self.position]).to_string();
    }

    fn read_comment(&mut self) -> String {
        let start_pos = self.position;

        while self.peek().filter(|&ch| ch != b'\n').is_some() {
            self.read_char();
        }

        return String::from_utf8_lossy(&self.input[start_pos..=self.position]).to_string();
    }
}

#[cfg(test)]
mod test {
    use super::{Lexer, Token};

    fn test(input: String, expected_tokens: Vec<Token>) {
        let mut lexer = Lexer::new(input);
        for expected_token in expected_tokens {
            let next_token = lexer
                .next_token()
                .expect("Next token is none when it should not have been.");
            println!("expected: {:?}, received {:?}", expected_token, next_token);
            assert_eq!(expected_token, next_token)
        }

        assert_eq!(lexer.next_token(), None);
    }

    #[test]
    fn test_get_next_token() {
        let input = "+-*/%&|!>>===!=<<=;:,(){}[]";

        let expected_tokens = vec![
            Token::OpPlus,
            Token::OpMinus,
            Token::OpMult,
            Token::OpDiv,
            Token::OpMod,
            Token::OpAnd,
            Token::OpOr,
            Token::OpNot,
            Token::OpGt,
            Token::OpGe,
            Token::OpEq,
            Token::OpNe,
            Token::OpLt,
            Token::OpLe,
            Token::SemiColon,
            Token::Colon,
            Token::Comma,
            Token::Lparen,
            Token::RParen,
            Token::LSquirly,
            Token::RSquirly,
            Token::LBracket,
            Token::RBracket,
        ];

        test(input.into(), expected_tokens);
    }

    #[test]
    fn test_expressions() {
        let input = "let a = 5;";
        //                 0123456789
        let expected_tokens = vec![
            Token::KwLet,
            Token::Identifier("a".into()),
            Token::Assignment,
            Token::NumLiteral("5".into()),
            Token::SemiColon,
        ];

        test(input.into(), expected_tokens);
    }

    #[test]
    fn test_comment() {
        let input = "
        let a = 5;
        ? Foo+
        let b = 10;
        ? Bar_
        ";
        //                 0123456789
        let expected_tokens = vec![
            Token::KwLet,
            Token::Identifier("a".into()),
            Token::Assignment,
            Token::NumLiteral("5".into()),
            Token::SemiColon,
            Token::Comment("? Foo+".into()),
            Token::KwLet,
            Token::Identifier("b".into()),
            Token::Assignment,
            Token::NumLiteral("10".into()),
            Token::SemiColon,
            Token::Comment("? Bar_".into()),
        ];

        test(input.into(), expected_tokens);
    }

    #[test]
    fn test_complete_lexer() {
        let input = "let five = 5;
        let ten = 10.0;
        fn add(x, y) {
            return x + y;
        }

        if (5 <= 10) {
            return add(5, 10);
        }

        if (five != ten) {
            return add(five, ten);
        }

        10 == 10.0
        9.9 < 10

        let x = 7;
        while (11 >= 10) {
            x = x + five;
            if (x > 11) {
                break;
            }
        }
        ";

        let expected_tokens = vec![
            Token::KwLet,
            Token::Identifier("five".into()),
            Token::Assignment,
            Token::NumLiteral("5".into()),
            Token::SemiColon,
            Token::KwLet,
            Token::Identifier("ten".into()),
            Token::Assignment,
            Token::NumLiteral("10.0".into()),
            Token::SemiColon,
            Token::KwFn,
            Token::Identifier("add".into()),
            Token::Lparen,
            Token::Identifier("x".into()),
            Token::Comma,
            Token::Identifier("y".into()),
            Token::RParen,
            Token::LSquirly,
            Token::KwReturn,
            Token::Identifier("x".into()),
            Token::OpPlus,
            Token::Identifier("y".into()),
            Token::SemiColon,
            Token::RSquirly,
        ];

        test(input.into(), expected_tokens)
    }
}
