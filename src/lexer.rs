use Token::*;

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    Reserved(String),
    Ident(String),
    Num(u32),
}

pub struct Lexer {
    chs: Vec<char>,
    pos: usize,
}

const KEYWORDS: [&str; 9] = ["var", "op", "sin", "cos", "tan", "exp", "log", "dif", "print"];
const PUNCTS: [char; 16] = ['=', '+', '-', '*', '/', '^', '.', ',', ':', ';', '(', ')', '{', '}', '[', ']'];

impl Lexer {
    pub fn new(code: String) -> Self {
        Lexer {
            chs: code.chars().collect::<Vec<char>>(),
            pos: 0,
        }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut token_list = Vec::new();
        while self.pos < self.chs.len() {
            let mut c = self.chs[self.pos];
            if c == '/' && self.chs[self.pos+1] == '/' {
                while self.chs[self.pos] != '\n' {
                    self.pos += 1;
                }
                continue;
            }
            if c.is_whitespace() {
                self.pos += 1;
                continue;
            }
            if PUNCTS.to_vec().iter().find(|&&x| x == c) != None {
                token_list.push(Reserved(c.to_string()));
                self.pos += 1;
                continue;
            }
            if c.is_ascii_alphabetic() || c == '_' {
                let mut name = "".to_string();
                while c.is_ascii_alphanumeric() || c == '_' {
                    name.push(c);
                    self.pos += 1;
                    c = self.chs[self.pos];
                }
                if KEYWORDS.to_vec().iter().find(|&&x| x == name) != None {
                    token_list.push(Reserved(name));
                } else {
                    token_list.push(Ident(name));
                }
                continue;
            }
            if c.is_digit(10) {
                let mut val = 0;
                while c.is_digit(10) {
                    val = 10 * val + c.to_digit(10).unwrap();
                    self.pos += 1;
                    c = self.chs[self.pos];
                }
                token_list.push(Num(val));
                continue;
            }
            break;
        }
        // for token in &token_list {
        //     match token {
        //         Reserved(symbol) => {
        //             print!("{} ", symbol);
        //         },
        //         Ident(name) => {
        //             print!("{} ", name);
        //         },
        //         Num(val) => {
        //             print!("{} ", val);
        //         }
        //     }
        // }
        token_list
    }
}