use std::process::exit;
// use std::slice::Iter;
use crate::lexer::*;

use Token::*;
use NodeKind::*;
use Node::*;

fn error(msg: &str) {
    println!("error: {}", msg);
    exit(256);
}

#[derive(Debug, Clone, PartialEq)]
pub enum NodeKind {
    Add,
    Sub,
    Mul,
    Div,
    Sin,
    Cos,
    Tan,
    Neg,
    Exp,
    Log,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Node {
    BinaryOperator { kind: NodeKind, lhs: Box<Node>, rhs: Box<Node> },
    UnaryOperator { kind: NodeKind, operand: Box<Node> },
    Var { name: String, point: Option<Box<Node>> },
    Num { val: f32 },
}

impl Node {
    fn print(&self, indent: usize) {
        match self {
            BinaryOperator { kind, lhs, rhs } => {
                lhs.print(indent + 1);
                match kind {
                    Add => { print!(" + "); }
                    Sub => { print!(" - "); }
                    Mul => { print!(" * "); }
                    Div => { print!(" / "); }
                    _ => { print!(""); }
                }
                rhs.print(indent + 1);
            },
            UnaryOperator { kind, operand } => {
                match kind {
                    Sin => { print!("sin "); }
                    Cos => { print!("cos "); }
                    Tan => { print!("tan "); }
                    _ => { print!(" "); }
                }
                operand.print(indent + 1);
            },
            Var { name, point } => {
                match point {
                    Some(node) => {
                        node.print(indent + 1);
                    },
                    None => {
                        print!("{}", name);
                    },
                }
            },
            Node::Num { val } => {
                print!("{}", val);
            },
        }
        if indent == 0 {
            println!();
        }
    }
}

#[derive(Debug, Clone)]
struct SymbolTable {
    vec: Vec<Node>,
}

impl SymbolTable {
    fn new() -> Self {
        SymbolTable { vec: Vec::new() }
    }

    fn push(&mut self, node: Node) {
        match node {
            Node::Var { name: ref name, point: ref point } => {
                self.vec.push(node);
            },
            _ => {
            },
        }
    }

    fn pop(&mut self) {
        self.pop();
    }

    fn find(&mut self, name: String) -> Node {
        for i in (0..self.vec.len()).rev() {
            match self.vec[i] {
                Node::Var { name: ref name_, point: ref point_ } => {
                    if *name_ == name {
                        return self.vec[i].clone();
                    }
                },
                _ => {}
            }
        }
        println!("error: {} is undeclared.", name);
        Node::Num { val: 0.0 }
    }

    fn set(&mut self, name: String, node: Node) {
        for i in (0..self.vec.len()).rev() {
            match self.vec[i] {
                Node::Var { name: ref name_, point: ref point_ } => {
                    if *name_ == name {
                        self.vec[i] = Node::Var { name: name, point: Some(Box::new(node)) };
                        return;
                    }
                },
                _ => {}
            }
        }
        println!("error: {} is undeclared.", name);
    }
}

pub struct Parser<'a> {
    token_list: &'a Vec<Token>,
    pos: usize,
    symbol_table: SymbolTable,
}

impl<'a> Parser<'a> {
    pub fn new(token_list: &'a Vec<Token>) -> Self {
        // let mut table = SymbolTable::new();
        // table.push(Node::Var { name: "abc".to_string(), point: None });
        // table.push(Node::Var { name: "def".to_string(), point: None });
        // println!("{:?}", table);
        // table.set("def".to_string(), Node::Var { name: "inf".to_string(), point: None });
        // println!("{:?}", table);
        Parser { token_list: token_list, pos: 0, symbol_table: SymbolTable::new() }
    }

    pub fn prog(&mut self) -> Vec<Node> {
        let mut node_list: Vec<Node> = Vec::new();
        while self.pos < self.token_list.len() {
            self.stmt();
            // node_list.push(self.stmt());
            // node_list.last().unwrap().print(0);
            // break;
        }
        println!("{:?}", self.symbol_table);
        node_list
    }

    fn stmt(&mut self) {
        let token = &self.token_list[self.pos];
        self.inc();
        match token {
            Reserved(symbol) if *symbol == "var".to_string() => {
                let ident: Node;
                let name = self.next_ident();
                if self.expect("=") {
                    ident = Node::Var { name: name, point: Some(Box::new(self.expr())) };
                } else {
                    ident = Node::Var { name: name, point: None };
                }
                self.symbol_table.push(ident);
            },
            Ident(name) => {
                if self.expect("=") {
                    let value = self.expr();
                    self.symbol_table.set(name.to_string(), Node::Var { name: name.to_string(), point: Some(Box::new(value)) });
                }
            },
            _ => {
                error("expected an identifier");
            },
        };
        self.consume(";");
    }

    fn expr(&mut self) -> Node {
        self.add()
    }

    fn add(&mut self) -> Node {
        let mut node = self.mul();
        loop {
            if self.expect("+") {
                node = BinaryOperator { kind: Add, lhs: Box::new(node), rhs: Box::new(self.mul()) };
                continue;
            }
            if self.expect("-") {
                node = BinaryOperator { kind: Sub, lhs: Box::new(node), rhs: Box::new(self.mul()) };
                continue;
            }
            return node;
        }
    }

    fn mul(&mut self) -> Node {
        let mut node = self.unary();
        loop {
            if self.expect("*") {
                node = BinaryOperator { kind: Mul, lhs: Box::new(node), rhs: Box::new(self.unary()) };
                continue;
            }
            if self.expect("/") {
                node = BinaryOperator { kind: Div, lhs: Box::new(node), rhs: Box::new(self.unary()) };
                continue;
            }
            return node;
        }
    }

    fn unary(&mut self) -> Node {
        if self.expect("+") { return self.unary(); }
        if self.expect("-") { return UnaryOperator { kind: Neg, operand: Box::new(self.unary()) }; }
        if self.expect("sin") { return UnaryOperator { kind: Sin, operand: Box::new(self.unary()) }; }
        if self.expect("cos") { return UnaryOperator { kind: Cos, operand: Box::new(self.unary()) }; }
        if self.expect("tan") { return UnaryOperator { kind: Tan, operand: Box::new(self.unary()) }; }
        self.prim()
    }

    fn prim(&mut self) -> Node {
        let token = &self.token_list[self.pos];
        self.inc();
        match token {
            Token::Reserved(name) => {
                self.consume("(");
                let lhs = self.expr();
                self.consume(",");
                let rhs = self.expr();
                self.consume(")");
                if name == "exp" { return Node::BinaryOperator { kind: Exp, lhs: Box::new(lhs), rhs: Box::new(rhs) }; }
                if name == "log" { return Node::BinaryOperator { kind: Log, lhs: Box::new(lhs), rhs: Box::new(rhs) }; }
                Node::Num { val: 0.0 }
            },
            Token::Ident(ident) => {
                self.symbol_table.find(ident.to_string())
            },
            Token::Num(val) => {
                Node::Num { val: *val as f32 }
            },
        }
    }

    fn inc(&mut self) {
        self.pos += 1;
    }

    fn expect(&mut self, name: &str) -> bool {
        match &self.token_list[self.pos] {
            Reserved(symbol) => {
                if symbol == name {
                    self.pos += 1;
                    true
                } else {
                    false
                }
            },
            _ => {
                false
            },
        }
    }

    fn consume(&mut self, name: &str) {
        match &self.token_list[self.pos] {
            Reserved(symbol) => {
                self.pos += 1;
                if symbol != name {
                    println!("error: expected '{}'", name);
                }
            },
            _ => {
                println!("error: expected '{}'", name);
            },
        }
    }

    fn next_ident(&mut self) -> String {
        match &self.token_list[self.pos] {
            Ident(ident) => {
                self.pos += 1;
                ident.to_string()
            },
            _ => {
                println!("error: expected an identifier");
                String::from("_")
            }
        }
    }
}