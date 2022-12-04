// use std::process::exit;
use crate::node::*;
use crate::lexer::*;

use Token::*;
use BKind::*;
use UKind::*;
use Node::*;

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
                        Var { name: ref name_, .. } if *name_ == name => {
                            return params[i].clone();
                        },
                        _ => {}
                    }
                }
                Var { name, point }
            },
            Node::Num { .. } => {
                cont
            },
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

    fn find(&mut self, name: String) -> Option<Operator> {
        for i in (0..self.vec.len()).rev() {
            match self.vec[i].clone() {
                Operator { name: name_, .. } if name_ == name => {
                    return Some(self.vec[i].clone());
                },
                _ => {},
            }
        }
        None
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

    fn find(&mut self, name: String) -> Option<Node> {
        for i in (0..self.vec.len()).rev() {
            match self.vec[i].clone() {
                Node::Var { name: name_, .. } if name_ == name => {
                    return Some(self.vec[i].clone());
                },
                _ => {},
            }
        }
        None
    }

    fn set(&mut self, name: String, node: Node) {
        for i in (0..self.vec.len()).rev() {
            match self.vec[i].clone() {
                Node::Var { name: name_, .. } if name_ == name => {
                    self.vec[i] = Node::Var { name: name, point: Some(Box::new(node)) };
                    return;
                },
                _ => {},
            }
        }
        println!("\x1b[31merror\x1b[39m: {} is undeclared.", name);
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
        match token {
            Token::Reserved(s) if s == "var" => {
                self.inc();

                loop {
                    let var: Node;
                    let name = self.next_ident();
                    if self.expect("=") {
                        var = Node::Var { name: name, point: Some(Box::new(self.expr())) };
                    } else {
                        var = Node::Var { name: name, point: None };
                    }
                    self.symbol_table.push(var);
                    
                    if self.expect(",") { continue; }
                    else { break; }
                }
            },
            Token::Reserved(s) if s == "op" => {
                self.inc();

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
            Token::Reserved(s) if s == "print" => {
                self.inc();

                let node = self.expr();
                node.print(0);
            },
            Token::Ident(name) => {
                self.inc();

                if self.expect("=") {
                    let value = self.expr();
                    self.symbol_table.set(name.to_string(), Node::Var { name: name.to_string(), point: Some(Box::new(value)) });
                }
            },
            _ => {
                self.expr();
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
        let mut node = self.power();
        loop {
            if self.expect("*") {
                node = BinaryOperator { kind: Mul, lhs: Box::new(node), rhs: Box::new(self.power()) };
                continue;
            }
            if self.expect("/") {
                node = BinaryOperator { kind: Div, lhs: Box::new(node), rhs: Box::new(self.power()) };
                continue;
            }
            return node;
        }
    }

    fn power(&mut self) -> Node {
        let mut node = self.unary();
        if self.expect("^") {
            node = Node::BinaryOperator { kind: Pow, lhs: Box::new(node), rhs: Box::new(self.power()) };
        }
        node
    }

    fn unary(&mut self) -> Node {
        if self.expect("+") { return self.unary(); }
        if self.expect("-") { return UnaryOperator { kind: Neg, operand: Box::new(self.unary()) }; }
        if self.expect("sin") { return UnaryOperator { kind: Sin, operand: Box::new(self.unary()) }; }
        if self.expect("cos") { return UnaryOperator { kind: Cos, operand: Box::new(self.unary()) }; }
        if self.expect("tan") { return UnaryOperator { kind: Tan, operand: Box::new(self.unary()) }; }
        if self.expect("exp") { return UnaryOperator { kind: Exp, operand: Box::new(self.unary()) }; }
        if self.expect("log") { return UnaryOperator { kind: Log, operand: Box::new(self.unary()) }; }
        self.prim()
    }

    fn prim(&mut self) -> Node {
        let token = self.token_list[self.pos].clone();
        self.inc();
        match token {
            Token::Reserved(tok) if tok == "(" => {
                let node = self.expr();
                self.consume(")");
                node
            },
            Token::Reserved(tok) if tok == "dif" => {
                self.consume("(");
                let lhs = self.expr();
                self.consume(",");
                let rhs = self.expr();
                self.consume(")");
                
                Node::dif(lhs.clone(), rhs.clone())
            },
            Token::Ident(ident) => {
                if let Some(node) = self.symbol_table.find(ident.clone()) {
                    return node;
                }
                if let Some(op) = self.op_table.find(ident.clone()) {
                    let mut params = Vec::new();
                    self.consume("(");
                    loop {
                        let param = self.expr();
                        params.push(param);
                        if self.expect(",") { continue; }
                        if self.expect(")") { break; }
                    }

                    return op.construct(op.cont.clone(), params);
                }
                println!("\x1b[31merror\x1b[39m: expected an identifier");
                Node::Num { val: 0.0 }
            },
            Token::Num(val) => {
                Node::Num { val: val as f32 }
            },
            _ => {
                println!("\x1b[31merror\x1b[39m: unexpected token");
                Node::Num { val: 0.0 }
            },
        }
    }

    fn inc(&mut self) {
        self.pos += 1;
    }

    fn expect(&mut self, name: &str) -> bool {
        match &self.token_list[self.pos] {
            Reserved(symbol) if symbol == name => {
                self.pos += 1;
                true
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
                println!("\x1b[31merror\x1b[39m: expected '{}'", name);
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
                println!("\x1b[31merror\x1b[39m: expected an identifier");
                String::from("_")
            }
        }
    }
}