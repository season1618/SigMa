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
const PUNCTS: [char; 17] = ['D', '=', '+', '-', '*', '/', '^', '.', ',', ':', ';', '(', ')', '{', '}', '[', ']'];

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
                loop {
                    self.pos += 1;
                    if self.chs[self.pos] == '\n' {
                        break;
                    }
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
            if c.is_ascii_alphabetic() {
                let mut name = c.to_string();
                loop {
                    self.pos += 1;
                    c = self.chs[self.pos];
                    if c.is_ascii_alphanumeric() {
                        name.push(c);
                    } else {
                        break;
                    }
                }
                if KEYWORDS.to_vec().iter().find(|&&x| x == name) != None {
                    token_list.push(Reserved(name));
                } else {
                    token_list.push(Ident(name));
                }
                continue;
            }
            if c.is_digit(10) {
                let mut val = c.to_digit(10).unwrap();
                loop {
                    self.pos += 1;
                    c = self.chs[self.pos];
                    if c.is_digit(10) {
                        val = 10 * val + c.to_digit(10).unwrap();
                    } else {
                        break;
                    }
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