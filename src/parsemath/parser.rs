use super::ast::Node;
use super::token::*;
use super::tokenizer::Tokenizer;

struct Parser<'a> {
    tokenizer: Tokenizer<'a>,
    current_token: Token,
}

pub enum ParserError {
    UnableToParse(String),
    InvalidOperator(String),
}

impl<'a> Parser<'a> {
    pub fn new(expresion: &'a str) -> Result<Self, ParserError> {
        let mut lexer = Tokenizer::new(expresion);
        let cur_token = match lexer.next() {
            Some(token) => token,
            None => return Err(ParserError::InvalidOperator("Invalid Character".into())),
        };
        Ok(Parser {
            tokenizer: lexer,
            current_token: cur_token,
        })
    }

    pub fn parse(&mut self) -> Result<Node, ParserError> {
        let ast = self.generate_ast(OperPrec::DefaultZero);
        match ast {
            Ok(ast) => Ok(ast),
            Err(e) => Err(e),
        }
    }

    fn generate_ast(&mut self, oper_prec: OperPrec) -> Result<Node, ParserError> {
        let mut left_expr = self.parse_number()?;
        while oper_prec < self.current_token.get_oper_prec() {
            if self.current_token == Token::EOF {
                break;
            }
            let right_expr = self.convert_token_to_node(left_expr.clone()?);
            left_expr = right_expr;
        }
        Ok(left_expr)
    }
    fn parse_number(&mut self) -> Result<Node, ParserError> {
        let token = self.current_token.clone();
        match token {
            Token::Substract => {
                self.get_next_token()?;
                let expr = self.generate_ast(OperPrec::Negative)?;
                Ok(Node::Negative(Box::new(expr)))
            },
            Token::Num(i) => {
                self.get_next_token()?;
                Ok(Node::Number(i))
            },
            Token::LeftParen => {
                self.get_next_token()?;
                let expr = self.generate_ast(OperPrec::DefaultZero)?;
                self.check_parent(Token::RightParen)?;
                if self.current_token == Token::LeftParen {
                    let right = self.generate_ast(OperPrec::MulDiv)?;
                    return Ok(Node::Multiply(Box::new(expr), Box::new(right)));
                }
                Ok(expr)
            }
            _ => Err(ParserError::UnableToParse("Unable to parse".into())),
        }
    }
    
    fn convert_token_to_node(&mut self, left_expr: Node) -> Result<Node, ParserError> {
        match self.current_token {
            Token::Add => {
                self.get_next_token()?;
                let right_expr = self.generate_ast(OperPrec::AddSub)?;
                Ok(Node::Add(Box::new(left_expr), Box::new(right_expr)))
            },
            Token::Substract => {
                self.get_next_token()?;
                let right_expr = self.generate_ast(OperPrec::AddSub)?;
                Ok(Node::Subtract(Box::new(left_expr),Box::new(right_expr)))
            },
            Token::Multiply => {
                self.get_next_token()?;
                let right_expr = self.generate_ast(OperPrec::MulDiv)?;
                Ok(Node::Multiply(Box::new(left_expr),Box::new(right_expr)))
                },
            Token::Divide => {
                self.get_next_token()?;
                let right_expr = self.generate_ast(OperPrec::MulDiv)?;
                Ok(Node::Divide(Box::new(left_expr),Box::new(right_expr)))
            },
            Token::Caret => {
                self.get_next_token()?;
                let right_expr = self.generate_ast(OperPrec::Power)?;
                Ok(Node::Caret(Box::new(left_expr), Box::new(right_expr)))
            },
            _ => Err(ParserError::InvalidOperator(format!("Please enter valid operator {:?}", self.current_token )))
        }
    }
        
    fn check_paren(&mut self, expected:Token) -> Result<(), ParserError> {
        if expected == self.current_token {
            self.get_next_token()?;
            Ok(())
        }else{
            Err(ParserError::InvalidOperator(format!("Expected {:?} got {:?}", expected, self.current_token)))
        }
    }
    fn get_next_token(&mut self) -> Result<(), ParserError> {
        let next_token = self.tokenizer.next() {
            Some(token) => token,
            None => return Err(ParserError::InvalidOperator("Invalid character".into())),
        };
        self.current_token = next_token;
        Ok(())   
    }
}
