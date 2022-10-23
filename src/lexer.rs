use Token::*;

#[derive(Clone, PartialEq)]
pub enum Token {
    Reserved(String),
    Ident(String),
    Num(u32),
}

const KEYWORDS: [&str; 5] = ["sin", "cos", "tan", "exp", "log"];
const PUNCTS: [char; 17] = ['D', '=', '+', '-', '*', '/', '^', '.', ',', ':', ';', '(', ')', '{', '}', '[', ']'];

pub fn tokenize(code: String) -> Vec<Token> {
    let mut iter = code.chars();
    let mut itr = iter.next();
    let mut c;
    let mut token_list: Vec<Token> = Vec::new();
    while itr != None {
        c = itr.unwrap();
        if c.is_whitespace() {
            itr = iter.next();
            continue;
        }
        if PUNCTS.to_vec().iter().find(|&&x| x == c) != None {
            token_list.push(Reserved(c.to_string()));
            itr = iter.next();
            continue;
        }
        if c.is_ascii_alphabetic() {
            let mut name: String = c.to_string();
            loop {
                itr = iter.next();
                match itr {
                    Some(c) => {
                        if c.is_ascii_alphanumeric() {
                            name.push(c);
                        } else {
                            break;
                        }
                    },
                    None => {
                        break;
                    }
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
                itr = iter.next();
                match itr {
                    Some(c) => {
                        if c.is_digit(10) {
                            val = 10 * val + c.to_digit(10).unwrap();
                        } else {
                            break;
                        }
                    },
                    None => {
                        break;
                    }
                }
            }
            token_list.push(Num(val));
            continue;
        }
        break;
    }
    for token in &token_list {
        match token {
            Reserved(symbol) => {
                print!("{} ", symbol);
            },
            Ident(name) => {
                print!("{} ", name);
            },
            Num(val) => {
                print!("{} ", val);
            }
        }
    }
    token_list
}