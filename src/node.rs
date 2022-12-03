use BKind::*;
use UKind::*;
use Node::*;

#[derive(Debug, Clone, PartialEq)]
pub enum BKind {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
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
            BinaryOperator { kind: kind1, lhs: lhs1, rhs: rhs1 } => {
                match node2 {
                    BinaryOperator { kind: kind2, lhs: lhs2, rhs: rhs2 } => {
                        kind1 == kind2 && Node::equiv(*lhs1, *lhs2) && Node::equiv(*rhs1, *rhs2)
                    },
                    _ => false
                }
            },
            UnaryOperator { kind: kind1, operand: operand1 } => {
                match node2 {
                    UnaryOperator { kind: kind2, operand: operand2 } => {
                        kind1 == kind2 && Node::equiv(*operand1, *operand2)
                    },
                    _ => false
                }
            },
            Var { name: name1, .. } => {
                match node2 {
                    Var { name: name2, .. } => { name1 == name2 },
                    _ => false
                }
            },
            Num { val: val1 } => {
                match node2 {
                    Num { val: val2 } => { val1 == val2 },
                    _ => false
                }
            },
        }
    }

    pub fn dif(node1: Node, node2: Node) -> Node {
        match node1 {
            BinaryOperator { kind, lhs, rhs } => {
                match kind {
                    Add => BinaryOperator { kind: Add, lhs: Box::new(Node::dif(*lhs, node2.clone())), rhs: Box::new(Node::dif(*rhs, node2.clone())) },
                    Sub => BinaryOperator { kind: Sub, lhs: Box::new(Node::dif(*lhs, node2.clone())), rhs: Box::new(Node::dif(*rhs, node2.clone())) },
                    Mul => BinaryOperator {
                        kind: Add,
                        lhs: Box::new(BinaryOperator { kind: Mul, lhs: Box::new(Node::dif((*lhs).clone(), node2.clone())), rhs: Box::new((*rhs).clone()) }),
                        rhs: Box::new(BinaryOperator { kind: Mul, lhs: Box::new((*lhs).clone()), rhs: Box::new(Node::dif((*rhs).clone(), node2.clone())) })
                    },
                    Div => BinaryOperator {
                        kind: Div,
                        lhs: Box::new(BinaryOperator {
                            kind: Sub,
                            lhs: Box::new(BinaryOperator { kind: Mul, lhs: Box::new(Node::dif((*lhs).clone(), node2.clone())), rhs: Box::new((*rhs).clone()) }),
                            rhs: Box::new(BinaryOperator { kind: Mul, lhs: Box::new((*lhs).clone()), rhs: Box::new(Node::dif((*rhs).clone(), node2.clone())) })
                        }),
                        rhs: Box::new(BinaryOperator { kind: Pow, lhs: Box::new(node2.clone()), rhs: Box::new(Num { val: 2.0 }) })
                    },
                    Pow => BinaryOperator {
                        kind: Add,
                        lhs: Box::new(BinaryOperator {
                            kind: Mul,
                            lhs: Box::new(BinaryOperator {
                                kind: Mul,
                                lhs: Box::new((*rhs).clone()),
                                rhs: Box::new(BinaryOperator {
                                    kind: Pow,
                                    lhs: Box::new((*lhs).clone()),
                                    rhs: Box::new(BinaryOperator {
                                        kind: Sub,
                                        lhs: Box::new((*rhs).clone()),
                                        rhs: Box::new(Num { val: 1.0 })
                                    })
                                })
                            }),
                            rhs: Box::new(Node::dif((*lhs).clone(), node2.clone()))
                        }),
                        rhs: Box::new(BinaryOperator {
                            kind: Mul,
                            lhs: Box::new(BinaryOperator {
                                kind: Mul,
                                lhs: Box::new(BinaryOperator {
                                    kind: Pow,
                                    lhs: Box::new((*lhs).clone()),
                                    rhs: Box::new((*rhs).clone())
                                }),
                                rhs: Box::new(UnaryOperator {
                                    kind: Log,
                                    operand: Box::new((*lhs).clone())
                                })
                            }),
                            rhs: Box::new(Node::dif((*rhs).clone(), node2.clone()))
                        })
                    },
                }
            },
            UnaryOperator { kind, operand } => {
                match kind {
                    Neg => UnaryOperator { kind: Neg, operand: Box::new(Node::dif(*operand, node2)) },
                    Sin => BinaryOperator {
                        kind: Mul,
                        lhs: Box::new(Node::dif((*operand).clone(), node2)),
                        rhs: Box::new(UnaryOperator { kind: Cos, operand: Box::new((*operand).clone()) })
                    },
                    Cos => UnaryOperator {
                        kind: Neg,
                        operand: Box::new(BinaryOperator {
                            kind: Mul,
                            lhs: Box::new(Node::dif((*operand).clone(), node2)),
                            rhs: Box::new(UnaryOperator { kind: Sin, operand: Box::new((*operand).clone()) })
                        })
                    },
                    Tan => BinaryOperator {
                        kind: Div,
                        lhs: Box::new(Node::dif((*operand).clone(), node2.clone())),
                        rhs: Box::new(BinaryOperator {
                            kind: Pow,
                            lhs: Box::new(UnaryOperator { kind: Cos, operand: Box::new((*operand).clone()) }),
                            rhs: Box::new(Num { val: 2.0 })
                        })
                    },
                    Exp => BinaryOperator {
                        kind: Mul,
                        lhs: Box::new(Node::dif((*operand).clone(), node2.clone())),
                        rhs: Box::new(UnaryOperator { kind: Exp, operand: Box::new((*operand).clone()) })
                    },
                    Log => BinaryOperator {
                        kind: Div,
                        lhs: Box::new(Node::dif((*operand).clone(), node2)),
                        rhs: Box::new((*operand).clone())
                    },
                }
            },
            Var { name, point } => {
                match node2 {
                    Var { name: name_, .. } if name == name_ => {
                        return Num { val: 1.0 };
                    },
                    Var { .. } => {
                        match point {
                            Some(node) => Node::dif(*node, node2.clone()),
                            None => Num { val: 0.0 }
                        }
                    },
                    _ => {
                        Num { val: 0.0 }
                    },
                }
            },
            Num { .. } => Num { val: 0.0 },
        }
    }

    pub fn print(&self, indent: usize) {
        match self {
            BinaryOperator { kind, lhs, rhs } => {
                lhs.print(indent + 1);
                match kind {
                    Add => { print!(" + "); }
                    Sub => { print!(" - "); }
                    Mul => { print!(" * "); }
                    Div => { print!(" / "); }
                    Pow => { print!(" ^ "); }
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
            Num { val } => {
                print!("{}", val);
            },
        }
        if indent == 0 {
            println!();
        }
    }
}