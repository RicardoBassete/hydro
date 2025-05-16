use crate::tokenization::{Token, TokenType};

pub struct NodeExpr {
    pub int_lit: Token,
}
pub struct NodeExit {
    pub expr: NodeExpr,
}

pub struct Parser {
    tokens: Vec<Token>,
    current_index: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser {
            tokens: tokens,
            current_index: 0,
        }
    }

    fn parse_expr(&mut self) -> Option<NodeExpr> {
        let ahead = self.peak(1);
        if ahead.is_some() && ahead.unwrap().token_type == TokenType::IntLit {
            return Some(NodeExpr {
                int_lit: self.consume(),
            });
        } else {
            None
        }
    }

    pub fn parse(&mut self) -> Option<NodeExit> {
        let mut exit_node: Option<NodeExit> = None;
        while self.peak(1).is_some() {
            let ahead = self.peak(1).unwrap();
            if ahead.token_type == TokenType::Exit {
                self.consume();
                if let Some(node_expr) = self.parse_expr() {
                    exit_node = Some(NodeExit { expr: node_expr });
                } else {
                    eprintln!("Invalid expression");
                    return None;
                }
            }
            if self.peak(1).is_some() && self.peak(1).unwrap().token_type != TokenType::Semi {
                eprintln!("Invalid expression");
                return None;
            }
        }
        self.current_index = 0;
        exit_node
    }

    fn peak(&self, ahead: usize) -> Option<Token> {
        if self.current_index + ahead >= self.tokens.len() {
            return None;
        } else {
            return Some(self.tokens[self.current_index].clone());
        }
    }

    fn consume(&mut self) -> Token {
        let result = self.tokens[self.current_index].clone();
        self.current_index += 1;
        result
    }
}
