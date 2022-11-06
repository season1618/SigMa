use std::process::exit;
use crate::lexer::*;

use Token::*;
use OpKind::*;
use Node::*;

fn error(msg: &str) {
    println!("error: {}", msg);
    exit(256);
}

#[derive(Debug, Clone, PartialEq)]
pub enum OpKind {
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

#[derive(Debug, Clone)]
struct Operator {
    name: String,
    args: Vec<Node>,
    cont: Node,
}

impl Operator {
    fn construct(&self, cont: Node, params: Vec<Node>) -> Node {
        match cont {
            BinaryOperator { kind, lhs, rhs } => {
                BinaryOperator { kind, lhs: Box::new(self.construct(*lhs, params.clone())), rhs: Box::new(self.construct(*rhs, params.clone())) }
            },
            UnaryOperator { kind, operand } => {
                UnaryOperator { kind, operand: Box::new(self.construct(*operand, params.clone())) }
            },
            Var { name, point } => {
                for i in 0..self.args.len() {
                    match self.args[i] {
                        Var { name: ref name_, point: ref point_ } if *name_ == name => {
                            return params[i].clone();
                        },
                        _ => {}
                    }
                }
                Var { name, point }
            },
            Node::Num { val } => {
                cont
            },
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Node {
    BinaryOperator { kind: OpKind, lhs: Box<Node>, rhs: Box<Node> },
    UnaryOperator { kind: OpKind, operand: Box<Node> },
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
struct OperatorTable {
    vec: Vec<Operator>,
}

impl OperatorTable {
    fn new() -> Self {
        OperatorTable { vec: Vec::new() }
    }

    fn push(&mut self, item: Operator) {
        self.vec.push(item);
    }

    fn pop(&mut self) {
        self.vec.pop();
    }

    fn find(&mut self, name: String) -> Operator {
        for i in (0..self.vec.len()).rev() {
            match self.vec[i] {
                Operator { name: ref name_, args: ref args_, cont: ref cont_ } if *name_ == name => {
                    return self.vec[i].clone();
                },
                _ => {},
            }
        }
        println!("error: {} is undeclared.", name);
        Operator { name: String::new(), args: Vec::new(), cont: Node::Num { val: 0.0 } }
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
        self.vec.push(node);
        // match node {
        //     Node::Var { name: ref name, point: ref point } => {
        //         self.vec.push(node);
        //     },
        //     _ => {
        //     },
        // }
    }

    fn pop(&mut self) {
        self.vec.pop();
    }

    fn find(&mut self, name: String) -> Node {
        for i in (0..self.vec.len()).rev() {
            match self.vec[i] {
                Node::Var { name: ref name_, point: ref point_ } if *name_ == name => {
                    return self.vec[i].clone();
                },
                _ => {},
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

pub struct Parser {
    token_list: Vec<Token>,
    pos: usize,
    symbol_table: SymbolTable,
    op_table: OperatorTable,
}

impl Parser {
    pub fn new<'a>(token_list: &'a Vec<Token>) -> Self {
        Parser {
            token_list: token_list.clone(),
            pos: 0,
            symbol_table: SymbolTable::new(),
            op_table: OperatorTable::new(),
        }
    }

    pub fn prog(&mut self) -> Vec<Node> {
        let mut node_list: Vec<Node> = Vec::new();
        while self.pos < self.token_list.len() {
            self.stmt();
        }
        // println!("{:?}", self.symbol_table);
        node_list
    }

    fn stmt(&mut self) {
        let token = self.token_list[self.pos].clone();
        self.inc();
        match token {
            Token::Reserved(s) if &*s == "var" => {
                let var: Node;
                let name = self.next_ident();
                if self.expect("=") {
                    var = Node::Var { name: name, point: Some(Box::new(self.expr())) };
                } else {
                    var = Node::Var { name: name, point: None };
                }
                self.symbol_table.push(var);
            },
            Token::Reserved(s) if s == "op" => {
                let name = self.next_ident();
                let mut args = Vec::new();

                self.consume("(");
                loop {
                    let arg = Node::Var { name: self.next_ident(), point: None };
                    args.push(arg.clone());
                    self.symbol_table.push(arg);
                    if self.expect(",") { continue; }
                    if self.expect(")") { break; }
                }

                self.consume("{");
                let cont = self.expr();
                self.consume("}");

                let operator = Operator { name, args, cont };
                self.op_table.push(operator);
            },
            Token::Reserved(s) if &*s == "print" => {
                let ident = self.next_ident();
                self.symbol_table.find(ident).print(0);
            },
            Token::Ident(name) => {
                if self.expect("=") {
                    let value = self.expr();
                    self.symbol_table.set(name.to_string(), Node::Var { name: name.to_string(), point: Some(Box::new(value)) });
                }
            },
            _ => {
                error("expected an identifier");
            },
        }
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
        let token = self.token_list[self.pos].clone();
        self.inc();
        match token {
            Token::Reserved(tok) if &*tok == "(" => {
                let node = self.expr();
                self.consume(")");
                node
            },
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
                let node = self.symbol_table.find(ident.clone());
                match node {
                    Node::Num { ref val } if *val == 0.0 => {},
                    _ => { return node; },
                }
                let op = self.op_table.find(ident.clone());
                let mut params = Vec::new();
                self.consume("(");
                loop {
                    let param = self.expr();
                    params.push(param);
                    if self.expect(",") { continue; }
                    if self.expect(")") { break; }
                }

                op.construct(op.cont.clone(), params)
            },
            Token::Num(val) => {
                Node::Num { val: val as f32 }
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
            Reserved(symbol) if symbol == name => {
                self.pos += 1;
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