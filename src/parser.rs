use std::process::exit;
use crate::lexer::*;

use Token::*;
use BKind::*;
use UKind::*;
use Node::*;

fn error(msg: &str) {
    println!("error: {}", msg);
    exit(256);
}

#[derive(Debug, Clone, PartialEq)]
pub enum BKind {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug, Clone, PartialEq)]
pub enum UKind {
    Neg,
    Sin,
    Cos,
    Tan,
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

#[derive(Debug, Clone, PartialEq)]
pub enum Node {
    BinaryOperator { kind: BKind, lhs: Box<Node>, rhs: Box<Node> },
    UnaryOperator { kind: UKind, operand: Box<Node> },
    Var { name: String, point: Option<Box<Node>> },
    Num { val: f32 },
}

impl Node {
    fn equiv(node1: Node, node2: Node) -> bool {
        match node1 {
            Node::BinaryOperator { kind: kind1, lhs: lhs1, rhs: rhs1 } => {
                match node2 {
                    BinaryOperator { kind: kind2, lhs: lhs2, rhs: rhs2 } => {
                        kind1 == kind2 && Node::equiv(*lhs1, *lhs2) && Node::equiv(*rhs1, *rhs2)
                    },
                    _ => false
                }
            },
            Node::UnaryOperator { kind: kind1, operand: operand1 } => {
                match node2 {
                    UnaryOperator { kind: kind2, operand: operand2 } => {
                        kind1 == kind2 && Node::equiv(*operand1, *operand2)
                    },
                    _ => false
                }
            },
            Node::Var { name: name1, .. } => {
                match node2 {
                    Node::Var { name: name2, .. } => { name1 == name2 },
                    _ => false
                }
            },
            Node::Num { val: val1 } => {
                match node2 {
                    Node::Num { val: val2 } => { val1 == val2 },
                    _ => false
                }
            },
        }
    }

    fn dif(node1: Node, node2: Node) -> Node {
        if Node::equiv(node1.clone(), node2.clone()) {
            return Node::Num { val: 1.0 };
        }
        match node1 {
            Node::BinaryOperator { kind, lhs, rhs } => {
                match kind {
                    Add => Node::BinaryOperator { kind: Add, lhs: Box::new(Node::dif(*lhs, node2.clone())), rhs: Box::new(Node::dif(*rhs, node2.clone())) },
                    Sub => Node::BinaryOperator { kind: Sub, lhs: Box::new(Node::dif(*lhs, node2.clone())), rhs: Box::new(Node::dif(*rhs, node2.clone())) },
                    Mul => Node::BinaryOperator {
                        kind: Add,
                        lhs: Box::new(Node::BinaryOperator { kind: Mul, lhs: Box::new(Node::dif((*lhs).clone(), node2.clone())), rhs: Box::new((*rhs).clone()) }),
                        rhs: Box::new(Node::BinaryOperator { kind: Mul, lhs: Box::new((*lhs).clone()), rhs: Box::new(Node::dif((*rhs).clone(), node2.clone())) })
                    },
                    Div => Node::BinaryOperator {
                        kind: Div,
                        lhs: Box::new(Node::BinaryOperator {
                            kind: Sub,
                            lhs: Box::new(Node::BinaryOperator { kind: Mul, lhs: Box::new(Node::dif((*lhs).clone(), node2.clone())), rhs: Box::new((*rhs).clone()) }),
                            rhs: Box::new(Node::BinaryOperator { kind: Mul, lhs: Box::new((*lhs).clone()), rhs: Box::new(Node::dif((*rhs).clone(), node2.clone())) })
                        }),
                        rhs: Box::new(Node::BinaryOperator { kind: Mul, lhs: Box::new(node2.clone()), rhs: Box::new(node2.clone()) })
                    }
                }
            },
            Node::UnaryOperator { kind, operand } => {
                match kind {
                    Neg => Node::UnaryOperator { kind: Neg, operand: Box::new(Node::dif(*operand, node2)) },
                    Sin => Node::BinaryOperator {
                        kind: Mul,
                        lhs: Box::new(Node::dif((*operand).clone(), node2)),
                        rhs: Box::new(Node::UnaryOperator { kind: Cos, operand: Box::new((*operand).clone()) })
                    },
                    Cos => Node::UnaryOperator {
                        kind: Neg,
                        operand: Box::new(Node::BinaryOperator {
                            kind: Mul,
                            lhs: Box::new(Node::dif((*operand).clone(), node2)),
                            rhs: Box::new(Node::UnaryOperator { kind: Sin, operand: Box::new((*operand).clone()) })
                        })
                    },
                    Tan => Node::BinaryOperator {
                        kind: Div,
                        lhs: Box::new(Node::dif((*operand).clone(), node2.clone())),
                        rhs: Box::new(Node::BinaryOperator {
                            kind: Mul,
                            lhs: Box::new(Node::UnaryOperator { kind: Cos, operand: Box::new((*operand).clone()) }),
                            rhs: Box::new(Node::UnaryOperator { kind: Cos, operand: Box::new((*operand).clone()) })
                        })
                    },
                    Exp => Node::BinaryOperator {
                        kind: Mul,
                        lhs: Box::new(Node::dif((*operand).clone(), node2.clone())),
                        rhs: Box::new(Node::UnaryOperator { kind: Exp, operand: Box::new((*operand).clone()) })
                    },
                    Log => Node::BinaryOperator {
                        kind: Div,
                        lhs: Box::new(Node::dif((*operand).clone(), node2)),
                        rhs: Box::new(Node::UnaryOperator { kind: Exp, operand: Box::new((*operand).clone()) })
                    },
                }
            },
            Node::Var { name, point } => {
                match node2 {
                    Node::Var { name: name_, .. } if name == name_ => {
                        return Node::Num { val: 1.0 };
                    },
                    Node::Var { .. } => {
                        match point {
                            Some(node) => Node::dif(*node, node2.clone()),
                            None => Node::Num { val: 0.0 }
                        }
                    },
                    _ => {
                        Node::Num { val: 0.0 }
                    },
                }
            },
            Node::Num { .. } => Node::Num { val: 0.0 },
        }
    }

    fn print(&self, indent: usize) {
        match self {
            BinaryOperator { kind, lhs, rhs } => {
                lhs.print(indent + 1);
                match kind {
                    Add => { print!(" + "); }
                    Sub => { print!(" - "); }
                    Mul => { print!(" * "); }
                    Div => { print!(" / "); }
                }
                rhs.print(indent + 1);
            },
            UnaryOperator { kind, operand } => {
                match kind {
                    Neg => { print!("- "); }
                    Sin => { print!("sin "); }
                    Cos => { print!("cos "); }
                    Tan => { print!("tan "); }
                    Exp => { print!("exp "); }
                    Log => { print!("log "); }
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
                Operator { name: ref name_, .. } if *name_ == name => {
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
                Node::Var { name: ref name_, .. } if *name_ == name => {
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
                Node::Var { name: ref name_, .. } if *name_ == name => {
                    self.vec[i] = Node::Var { name: name, point: Some(Box::new(node)) };
                    return;
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
        match token {
            Token::Reserved(s) if &*s == "var" => {
                self.inc();

                let var: Node;
                let name = self.next_ident();
                if self.expect("=") {
                    var = Node::Var { name: name, point: Some(Box::new(self.expr())) };
                } else {
                    var = Node::Var { name: name, point: None };
                }
                self.symbol_table.push(var);
            },
            Token::Reserved(s) if &*s == "op" => {
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
            Token::Reserved(s) if &*s == "print" => {
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
        if self.expect("exp") { return UnaryOperator { kind: Exp, operand: Box::new(self.unary()) }; }
        if self.expect("log") { return UnaryOperator { kind: Log, operand: Box::new(self.unary()) }; }
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
            Token::Reserved(tok) if &*tok == "dif" => {
                self.consume("(");
                let lhs = self.expr();
                self.consume(",");
                let rhs = self.expr();
                self.consume(")");
                
                Node::dif(lhs.clone(), rhs.clone())
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
            _ => {
                println!("error: unexpected token", );
                Node::Num { val: 0.0 }
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