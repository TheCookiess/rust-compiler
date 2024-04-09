#![allow(dead_code, unused_mut, unused_assignments)]
use crate::lexer::*;

// union would be better ideally.

#[derive(Debug)]
pub enum StmtKind {
    Illegal,
    Exit,
    Let,
}

#[derive(Debug)]
pub enum ExprKind {
    Illegal,
    Term,
    BinExpr,
}

#[derive(Debug)]
pub enum TermKind {
    IntLit,
    Ident,
}

#[derive(Debug)]
pub enum BinExprKind {
    Add,
    Multi,
}

#[derive(Debug)]
pub struct NodeStmt {
    pub kind: StmtKind,
    pub ident: Option<Token>,
    pub expr: Option<Box<NodeExpr>>,
}

#[derive(Debug)]
pub struct NodeExpr {
    pub kind: ExprKind,
    pub term: Option<Box<NodeTerm>>,
    pub bin_expr: Option<Box<NodeBinExpr>>,
}

#[derive(Debug)]
pub struct NodeTerm {
    pub kind: TermKind,
    pub token: Token,
}

#[derive(Debug)]
pub struct NodeBinExpr {
    pub kind: BinExprKind,
    pub lhs: Box<NodeExpr>,
    pub rhs: Box<NodeExpr>,
}

#[derive(Debug)]
pub struct NodeProg {
    pub stmts: Vec<NodeStmt>,
}

pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    pub fn new(input: Vec<Token>) -> Parser {
        let mut parser = Parser {
            tokens: input,
            position: 0,
        };
        return parser;
    }

    pub fn parse_prog(&mut self) -> Result<NodeProg, &'static str> {
        let mut prog = NodeProg { stmts: vec![] };
        while self.peek(0).is_some() {
            prog.stmts.push(self.parse_stmt()?);
        }

        return Ok(prog);
    }

    // don't propogate err up (?), use .is_ok()
    fn parse_stmt(&mut self) -> Result<NodeStmt, &'static str> {
        let cur_tok = &self.tokens[self.position];
        // println!("parsing {:?}", cur_tok);
        let mut stmt = NodeStmt {
            kind: StmtKind::Illegal,
            ident: None,
            expr: None,
        };

        if cur_tok.kind == TokenKind::KeywordExit
            && self.token_equals(TokenKind::OpenParen, 1)?
            && self.token_equals(TokenKind::CloseParen, 3)?
        {
            self.consume(); // "exit"
            self.consume(); // '('
            stmt = NodeStmt {
                kind: StmtKind::Exit,
                ident: None,
                expr: Some(Box::new(self.parse_expr()?)),
            };
            self.consume(); // ')'
        } else if cur_tok.kind == TokenKind::KeywordLet
            && self.token_equals(TokenKind::Ident, 1)?
            && self.token_equals(TokenKind::Assign, 2)?
        {
            self.consume(); // "let"
            let temp_ident = Some(self.consume());
            self.consume(); // '='
            stmt = NodeStmt {
                kind: StmtKind::Let,
                ident: temp_ident,
                expr: Some(Box::new(self.parse_expr()?)),
            };
        }

        if self.token_equals(TokenKind::SemiColon, 0).is_ok() {
            self.consume();
            return Ok(stmt);
        }
        return Err("Expected ';'.");
    }

    fn parse_expr(&mut self) -> Result<NodeExpr, &'static str> {
        // should parse all expressions until none left?
        // e.g if self.peek(1) == OPERATOR >> parse self.peek(2) << rhs

        // if self.peek(1).is_some() && self.peek(1).unwrap().is_operator() {
        // parse operator & rhs
        // rhs can be another bin_expr
        // }

        let term = self.parse_term()?;
        if self.peek(0).is_some() {
            match self.peek(0).unwrap().kind {
                TokenKind::Multiply => {}
                TokenKind::Divide => {}
                TokenKind::Add => {}
                TokenKind::Subtract => {}
                _ => false,
            };
        }

        return Err("Unable to parse expression");
    }

    fn parse_term(&mut self) -> Result<NodeTerm, &'static str> {
        if self.peek(0).is_none() {
            return Err("No term to parse.");
        }

        return match self.peek(0).unwrap().kind {
            TokenKind::IntLit => Ok(NodeTerm {
                kind: TermKind::IntLit,
                token: self.consume(),
            }),
            TokenKind::Ident => Ok(NodeTerm {
                kind: TermKind::Ident,
                token: self.consume(),
            }),
            _ => Err("Unrecognized term, unable to parse."),
        };
    }

    fn parse_bin_expr(&mut self) -> Result<NodeBinExpr, &'static str> {
        return Err("Unable to parse binary expressio");
    }

    fn token_equals(&self, kind: TokenKind, offset: usize) -> Result<bool, &'static str> {
        if self.peek(offset).is_none() {
            // println!("no token found, can't eval to {:?}", kind);
            return Err("no token to evaluate");
        }
        // print!("checking {:?} == {:?}", self.peek(offset), kind);
        if self.peek(offset).is_some() && self.peek(offset).unwrap().kind == kind {
            // println!(" .. is true");
            return Ok(true);
        }
        // println!(" .. is false");
        // return Err(format!("expected '{:?}' was false", kind));
        return Err("token evaluation was false.");
    }

    fn peek(&self, offset: usize) -> Option<&Token> {
        return self.tokens.get(self.position + offset);
    }

    fn consume(&mut self) -> Token {
        let i = self.position;
        self.position += 1;
        // println!("consuming: {:?}", &self.tokens[i]);
        return self.tokens[i].clone();
    }
}

/*
return match self.peek(0).unwrap().kind {
            TokenKind::IntLit => Ok(NodeExpr {
                kind: ExprKind::Term,
                term: Some(Box::new(NodeTerm {
                    kind: TermKind::IntLit,
                    token: self.consume(),
                })),
                bin_expr: None,
            }),
            TokenKind::Ident => Ok(NodeExpr {
                kind: ExprKind::Term,
                term: Some(Box::new(NodeTerm {
                    kind: TermKind::Ident,
                    token: self.consume(),
                })),
                bin_expr: None,
            }),
            _ => Err("Unrecognized expression, unable to parse."),
        };
*/
