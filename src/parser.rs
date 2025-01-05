use crate::lexer::{Lexer, Token};

#[derive(Debug)]
pub enum ASTNode {
    Program(Vec<ASTNode>),
    Function {
        name: String,
        params: Vec<String>,
        body: Box<ASTNode>,
    },
    Let {
        name: String,
        value: Box<ASTNode>,
    },
    Return(Box<ASTNode>),
    If {
        condition: Box<ASTNode>,
        then_branch: Box<ASTNode>,
        else_branch: Option<Box<ASTNode>>,
    },
    For {
        init: Box<ASTNode>,
        condition: Box<ASTNode>,
        increment: Box<ASTNode>,
        body: Box<ASTNode>,
    },
    While {
        condition: Box<ASTNode>,
        body: Box<ASTNode>,
    },
    BinaryOp {
        op: Token,
        left: Box<ASTNode>,
        right: Box<ASTNode>,
    },
    Identifier(String),
    IntLiteral(i32),
    BoolLiteral(bool),
}

pub struct Parser<'a> {
    lexer: &'a mut Lexer,
    current_token: Token,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: &'a mut Lexer) -> Self {
        let current_token = lexer.next_token();
        Self { lexer, current_token }
    }

    fn advance(&mut self) {
        self.current_token = self.lexer.next_token();
    }

    fn parse_program(&mut self) -> ASTNode {
        let mut nodes = Vec::new();
        while self.current_token != Token::Eof {
            nodes.push(self.parse_statement());
        }
        ASTNode::Program(nodes)
    }

    fn parse_statement(&mut self) -> ASTNode {
        match self.current_token {
            Token::Let => self.parse_let(),
            Token::Function => self.parse_function(),
            Token::Return => self.parse_return(),
            Token::If => self.parse_if(),
            Token::For => self.parse_for(),
            Token::While => self.parse_while(),
            _ => self.parse_expression(),
        }
    }

    fn parse_let(&mut self) -> ASTNode {
        self.advance();
        if let Token::Identifier(name) = &self.current_token {
            self.advance();
            if self.current_token == Token::Equals {
                self.advance();
                let value = self.parse_expression();
                ASTNode::Let {
                    name: name.clone(),
                    value: Box::new(value),
                }
            } else {
                panic!("Expected '=' after let declaration");
            }
        } else {
            panic!("Expected identifier after let");
        }
    }

    fn parse_function(&mut self) -> ASTNode {
        self.advance();
        if let Token::Identifier(name) = &self.current_token {
            self.advance();
            if self.current_token == Token::LParen {
                self.advance();
                let mut params = Vec::new();
                while self.current_token != Token::RParen {
                    if let Token::Identifier(param) = &self.current_token {
                        params.push(param.clone());
                        self.advance();
                        if self.current_token == Token::Comma {
                            self.advance();
                        }
                    } else {
                        panic!("Expected identifier in function parameters");
                    }
                }
                self.advance();
                if self.current_token == Token::LBrace {
                    self.advance();
                    let body = self.parse_block();
                    ASTNode::Function {
                        name: name.clone(),
                        params,
                        body: Box::new(body),
                    }
                } else {
                    panic!("Expected '{{' after function parameters");
                }
            } else {
                panic!("Expected '(' after function name");
            }
        } else {
            panic!("Expected identifier after function");
        }
    }

    fn parse_return(&mut self) -> ASTNode {
        self.advance();
        let value = self.parse_expression();
        ASTNode::Return(Box::new(value))
    }

    fn parse_if(&mut self) -> ASTNode {
        self.advance();
        let condition = self.parse_expression();
        if self.current_token == Token::LBrace {
            self.advance();
            let then_branch = self.parse_block();
            let else_branch = if self.current_token == Token::Else {
                self.advance();
                if self.current_token == Token::LBrace {
                    self.advance();
                    Some(Box::new(self.parse_block()))
                } else {
                    panic!("Expected '{{' after else");
                }
            } else {
                None
            };
            ASTNode::If {
                condition: Box::new(condition),
                then_branch: Box::new(then_branch),
                else_branch,
            }
        } else {
            panic!("Expected '{{' after if condition");
        }
    }

    fn parse_for(&mut self) -> ASTNode {
        self.advance();
        let init = self.parse_statement();
        let condition = self.parse_expression();
        let increment = self.parse_statement();
        if self.current_token == Token::LBrace {
            self.advance();
            let body = self.parse_block();
            ASTNode::For {
                init: Box::new(init),
                condition: Box::new(condition),
                increment: Box::new(increment),
                body: Box::new(body),
            }
        } else {
            panic!("Expected '{{' after for loop");
        }
    }

    fn parse_while(&mut self) -> ASTNode {
        self.advance();
        let condition = self.parse_expression();
        if self.current_token == Token::LBrace {
            self.advance();
            let body = self.parse_block();
            ASTNode::While {
                condition: Box::new(condition),
                body: Box::new(body),
            }
        } else {
            panic!("Expected '{{' after while condition");
        }
    }

    fn parse_block(&mut self) -> ASTNode {
        let mut nodes = Vec::new();
        while self.current_token != Token::RBrace {
            nodes.push(self.parse_statement());
        }
        self.advance();
        ASTNode::Program(nodes)
    }

    fn parse_expression(&mut self) -> ASTNode {
        self.parse_precedence(0)
    }

    fn parse_precedence(&mut self, min_precedence: u8) -> ASTNode {
        let mut left = self.parse_primary();
        while let Some(op) = self.get_operator() {
            let precedence = self.get_precedence(&op);
            if precedence < min_precedence {
                break;
            }
            self.advance();
            let mut right = self.parse_primary();
            while let Some(next_op) = self.get_operator() {
                let next_precedence = self.get_precedence(&next_op);
                if next_precedence > precedence {
                    right = self.parse_precedence(precedence + 1);
                } else {
                    break;
                }
            }
            left = ASTNode::BinaryOp {
                op,
                left: Box::new(left),
                right: Box::new(right),
            };
        }
        left
    }

    fn parse_primary(&mut self) -> ASTNode {
        match &self.current_token {
            Token::Identifier(name) => {
                let name = name.clone();
                self.advance();
                ASTNode::Identifier(name)
            }
            Token::IntLiteral(value) => {
                let value = *value;
                self.advance();
                ASTNode::IntLiteral(value)
            }
            Token::True => {
                self.advance();
                ASTNode::BoolLiteral(true)
            }
            Token::False => {
                self.advance();
                ASTNode::BoolLiteral(false)
            }
            Token::LParen => {
                self.advance();
                let expr = self.parse_expression();
                if self.current_token == Token::RParen {
                    self.advance();
                    expr
                } else {
                    panic!("Expected ')' after expression");
                }
            }
            _ => panic!("Unexpected token: {:?}", self.current_token),
        }
    }

    fn get_operator(&self) -> Option<Token> {
        match self.current_token {
            Token::Plus | Token::Minus | Token::Star | Token::Slash | Token::Equals => Some(self.current_token.clone()),
            _ => None,
        }
    }

    fn get_precedence(&self, token: &Token) -> u8 {
        match token {
            Token::Equals => 1,
            Token::Plus | Token::Minus => 2,
            Token::Star | Token::Slash => 3,
            _ => 0,
        }
    }
}
