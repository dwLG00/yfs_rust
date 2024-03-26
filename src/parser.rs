use std::collections::HashMap;
use std::collections::HashSet;
use crate::tokenizer;

#[derive(Debug)]
#[derive(Clone)]
pub enum SemType {
    Root,
    AuxRoot,
    Statement,
    AuxExpression,
    Expression,
    AuxSubexpression,
    Subexpression,
    LitTerm,
    Literal(String),
    Term(u32)
}

/*
Rules:
A : B, A'; #root
A': B, A' | nothing; #aux-root
B : $T, DEFN, C, TERMINAL; #statement
C : E, D; #expression
D : OR, E, D | nothing; #aux-expression
E : G, F; #subexpression
F : AND, G, F | nothing; #aux-subexpression
G : $L | $T; #litterm
*/

#[derive(Debug)]
pub struct Node {
    semtype: SemType,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>
}

impl Node {
    fn fulfill_production(&mut self, tokens: &Vec<tokenizer::Token>, idx: usize) -> usize {
        let token = if idx < tokens.len() {
            Some(tokens[idx].clone())
        } else {
            None
        };

        match self.semtype {
            SemType::Root => match token {
                Some(tokenizer::Token::Term(_)) => {
                    let mut left = Node {
                        semtype: SemType::Statement,
                        left: None,
                        right: None
                    };
                    let mut right = Node {
                        semtype: SemType::AuxRoot,
                        left: None,
                        right: None
                    };
                    let new_idx = left.fulfill_production(&tokens, idx);
                    let new_new_idx = right.fulfill_production(&tokens, new_idx);
                    self.left = Some(Box::new(left));
                    self.right = Some(Box::new(right));
                    new_new_idx
                },
                None => {idx},
                _ => panic!(),
            },
            SemType::AuxRoot => match token {
                Some(tokenizer::Token::Term(_)) => {
                    let mut left = Node {
                        semtype: SemType::Statement,
                        left: None,
                        right: None
                    };
                    let mut right = Node {
                        semtype: SemType::AuxRoot,
                        left: None,
                        right: None
                    };
                    let new_idx = left.fulfill_production(&tokens, idx);
                    let new_new_idx = right.fulfill_production(&tokens, new_idx);
                    self.left = Some(Box::new(left));
                    self.right = Some(Box::new(right));
                    new_new_idx
                },
                None => {idx},
                _ => panic!()
            },
            SemType::Statement => match token {
                Some(tokenizer::Token::Term(id)) => {
                    let mut left = Node {
                        semtype: SemType::Term(id),
                        left: None,
                        right: None
                    };
                    let mut right = Node {
                        semtype: SemType::Expression,
                        left: None,
                        right: None
                    };
                    let new_new_idx = right.fulfill_production(&tokens, idx + 2);
                    self.left = Some(Box::new(left));
                    self.right = Some(Box::new(right));
                    new_new_idx
                },
                _ => panic!()
            },
            SemType::Expression => match token {
                Some(t) => match t {
                    tokenizer::Token::Term(_) | tokenizer::Token::Literal(_) => {
                        let mut left = Node {
                            semtype: SemType::Subexpression,
                            left: None,
                            right: None
                        };
                        let mut right = Node {
                            semtype: SemType::AuxExpression,
                            left: None,
                            right: None
                        };
                        let new_idx = left.fulfill_production(&tokens, idx);
                        let new_new_idx = right.fulfill_production(&tokens, new_idx);
                        self.left = Some(Box::new(left));
                        self.right = Some(Box::new(right));
                        new_new_idx
                    },
                    _ => panic!()
                },
                None => panic!()
            },
            SemType::AuxExpression => match token {
                Some(t) => match t {
                    tokenizer::Token::Terminal => {
                        idx + 1
                    },
                    tokenizer::Token::Or => {
                        let mut left = Node {
                            semtype: SemType::Subexpression,
                            left: None,
                            right: None
                        };
                        let mut right = Node {
                            semtype: SemType::AuxExpression,
                            left: None,
                            right: None
                        };
                        let new_idx = left.fulfill_production(&tokens, idx + 1);
                        let new_new_idx = right.fulfill_production(&tokens, new_idx);
                        self.left = Some(Box::new(left));
                        self.right = Some(Box::new(right));
                        new_new_idx
                    },
                    _ => panic!()
                },
                None => panic!()
            },
            SemType::Subexpression => match token {
                Some(t) => match t {
                    tokenizer::Token::Term(_) | tokenizer::Token::Literal(_) => {
                        let mut left = Node {
                            semtype: SemType::LitTerm,
                            left: None,
                            right: None
                        };
                        let mut right = Node {
                            semtype: SemType::AuxSubexpression,
                            left: None,
                            right: None
                        };
                        let new_idx = left.fulfill_production(&tokens, idx);
                        let new_new_idx = right.fulfill_production(&tokens, new_idx);
                        self.left = Some(Box::new(left));
                        self.right = Some(Box::new(right));
                        new_new_idx
                    },
                    _ => panic!()
                },
                None => panic!()
            },
            SemType::AuxSubexpression => match token {
                Some(t) => match t {
                    tokenizer::Token::Terminal | tokenizer::Token::Or => { idx },
                    tokenizer::Token::And => {
                        let mut left = Node {
                            semtype: SemType::LitTerm,
                            left: None,
                            right: None
                        };
                        let mut right = Node {
                            semtype: SemType::AuxSubexpression,
                            left: None,
                            right: None
                        };
                        let new_idx = left.fulfill_production(&tokens, idx + 1);
                        let new_new_idx = right.fulfill_production(&tokens, new_idx);
                        self.left = Some(Box::new(left));
                        self.right = Some(Box::new(right));
                        new_new_idx
                    },
                    _ => panic!()
                },
                None => panic!()
            },
            SemType::LitTerm => match token {
                Some(t) => match t {
                    tokenizer::Token::Term(id) => {
                        let mut left = Node {
                            semtype: SemType::Term(id),
                            left: None,
                            right: None
                        };
                        self.left = Some(Box::new(left));
                        idx + 1
                    },
                    tokenizer::Token::Literal(s) => {
                        let mut left = Node {
                            semtype: SemType::Literal(s.clone()),
                            left: None,
                            right: None
                        };
                        self.left = Some(Box::new(left));
                        idx + 1
                    },
                    _ => panic!()
                },
                None => panic!()
            },
            SemType::Term(_) => panic!(),
            SemType::Literal(_) => panic!()
        }
    }
}

pub fn parse(tokens: Vec<tokenizer::Token>) -> Node {
    let mut root = Node {
        semtype: SemType::Root,
        left: None,
        right: None
    };
    root.fulfill_production(&tokens, 0);
    root
}
