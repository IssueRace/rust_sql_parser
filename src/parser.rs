//Parser converts tokens into a Statement 

use crate::statement::{Statement, Expression, UnaryOperator, BinaryOperator, TableColumn, DBType, Constraint};
use crate::token::{Token, Keyword};
use crate::tokenizer::Tokenizer;

pub struct Parser<'a> {
    tokenizer: Tokenizer<'a>,
    current_token: Token,
}

impl<'a> Parser<'a> {
    // initializes parser with a tokenizer and first token 
    pub fn new(mut tokenizer: Tokenizer<'a>) -> Result<Self, String> {
        let current_token = tokenizer.next_token()?;
        Ok(Self { tokenizer, current_token })
    }
    // parses SELECT statements (columns, FROM, WHERE, ORDER BY)/
    pub fn parse_select(&mut self) -> Result<Statement, String> {
    self.advance()?; 
    let mut columns = Vec::new();
    if self.current_token == Token::Star {
        columns.push(Expression::Wildcard);
        self.advance()?;
    } else {
        loop {
            columns.push(self.parse_expression(0)?);
            if self.current_token == Token::Comma {
                self.advance()?;
            } else {
                break;
            }
        }
    }
    if self.current_token != Token::Keyword(Keyword::From) {
        return Err("Expected FROM".to_string());
    }
    self.advance()?;
    let from = match &self.current_token {
        Token::Identifier(table) => {
            let table_name = table.clone();
            self.advance()?;
            table_name
        }
        _ => return Err("Expected table name after FROM".to_string()),
    };
    let r#where = if self.current_token == Token::Keyword(Keyword::Where) {
        self.advance()?;
        Some(self.parse_expression(0)?)
    } else {
        None
    };
    let orderby = if self.current_token == Token::Keyword(Keyword::Order) {
        self.advance()?;
        if self.current_token != Token::Keyword(Keyword::By) {
            return Err("Expected BY after ORDER".to_string());
        }
        self.advance()?;
        let mut orderby = Vec::new();
        loop {
            orderby.push(self.parse_expression(0)?);
            if self.current_token == Token::Comma {
                self.advance()?;
            } else {
                break;
            }
        }
        orderby
    } else {
        Vec::new()
    };
    if self.current_token != Token::Semicolon {
        return Err("Expected ';'".to_string());
    }
    self.advance()?;
    Ok(Statement::Select { columns, from, r#where, orderby })
}

    fn advance(&mut self) -> Result<(), String> {
        self.current_token = self.tokenizer.next_token()?;
        Ok(())
    }

    fn get_infix_precedence(&self) -> u32 {
        match self.current_token {
            Token::Plus | Token::Minus => 25,
            Token::Star | Token::Divide => 30,
            Token::GreaterThan | Token::LessThan | Token::Equal | Token::NotEqual |
            Token::GreaterThanOrEqual | Token::LessThanOrEqual => 20,
            Token::Keyword(Keyword::Or) => 15,
            Token::Keyword(Keyword::And) => 10,
            Token::Keyword(Keyword::Asc) | Token::Keyword(Keyword::Desc) => 5,
            Token::RightParentheses | Token::Comma | Token::Semicolon | Token::Eof => 0,
            _ => 0,
        }
    }

    fn parse_prefix(&mut self) -> Result<Expression, String> {
        let token = self.current_token.clone();
        self.advance()?;
        match token {
            Token::Number(n) => Ok(Expression::Number(n)),
            Token::Identifier(id) => Ok(Expression::Identifier(id)),
            Token::String(s) => Ok(Expression::String(s)),
            Token::Keyword(Keyword::True) => Ok(Expression::Bool(true)),
            Token::Keyword(Keyword::False) => Ok(Expression::Bool(false)),
            Token::LeftParentheses => {
                let expr = self.parse_expression(0)?;
                if self.current_token != Token::RightParentheses {
                    return Err("Expected ')'".to_string());
                }
                self.advance()?;
                Ok(expr)
            }
            Token::Minus => Ok(Expression::UnaryOperation {
                operand: Box::new(self.parse_expression(100)?),
                operator: UnaryOperator::Minus,
            }),
            Token::Plus => Ok(Expression::UnaryOperation {
                operand: Box::new(self.parse_expression(100)?),
                operator: UnaryOperator::Plus,
            }),
            Token::Keyword(Keyword::Not) => Ok(Expression::UnaryOperation {
                operand: Box::new(self.parse_expression(100)?),
                operator: UnaryOperator::Not,
            }),
            _ => Err(format!("Unexpected token: {:?}", token)),
        }
    }

    fn parse_infix(&mut self, left: Expression) -> Result<Expression, String> {
        let token = self.current_token.clone();
        let precedence = self.get_infix_precedence();
        self.advance()?;
        match token {
            Token::Plus => Ok(Expression::BinaryOperation {
                left_operand: Box::new(left),
                operator: BinaryOperator::Plus,
                right_operand: Box::new(self.parse_expression(precedence)?),
            }),
            Token::Minus => Ok(Expression::BinaryOperation {
                left_operand: Box::new(left),
                operator: BinaryOperator::Minus,
                right_operand: Box::new(self.parse_expression(precedence)?),
            }),
            Token::Star => Ok(Expression::BinaryOperation {
                left_operand: Box::new(left),
                operator: BinaryOperator::Multiply,
                right_operand: Box::new(self.parse_expression(precedence)?),
            }),
            Token::Divide => Ok(Expression::BinaryOperation {
                left_operand: Box::new(left),
                operator: BinaryOperator::Divide,
                right_operand: Box::new(self.parse_expression(precedence)?),
            }),
            Token::GreaterThan => Ok(Expression::BinaryOperation {
                left_operand: Box::new(left),
                operator: BinaryOperator::GreaterThan,
                right_operand: Box::new(self.parse_expression(precedence)?),
            }),
            Token::LessThan => Ok(Expression::BinaryOperation {
                left_operand: Box::new(left),
                operator: BinaryOperator::LessThan,
                right_operand: Box::new(self.parse_expression(precedence)?),
            }),
            Token::Equal => Ok(Expression::BinaryOperation {
                left_operand: Box::new(left),
                operator: BinaryOperator::Equal,
                right_operand: Box::new(self.parse_expression(precedence)?),
            }),
            Token::NotEqual => Ok(Expression::BinaryOperation {
                left_operand: Box::new(left),
                operator: BinaryOperator::NotEqual,
                right_operand: Box::new(self.parse_expression(precedence)?),
            }),
            Token::GreaterThanOrEqual => Ok(Expression::BinaryOperation {
                left_operand: Box::new(left),
                operator: BinaryOperator::GreaterThanOrEqual,
                right_operand: Box::new(self.parse_expression(precedence)?),
            }),
            Token::LessThanOrEqual => Ok(Expression::BinaryOperation {
                left_operand: Box::new(left),
                operator: BinaryOperator::LessThanOrEqual,
                right_operand: Box::new(self.parse_expression(precedence)?),
            }),
            Token::Keyword(Keyword::And) => Ok(Expression::BinaryOperation {
                left_operand: Box::new(left),
                operator: BinaryOperator::And,
                right_operand: Box::new(self.parse_expression(precedence)?),
            }),
            Token::Keyword(Keyword::Or) => Ok(Expression::BinaryOperation {
                left_operand: Box::new(left),
                operator: BinaryOperator::Or,
                right_operand: Box::new(self.parse_expression(precedence)?),
            }),
            Token::Keyword(Keyword::Asc) => Ok(Expression::UnaryOperation {
                operand: Box::new(left),
                operator: UnaryOperator::Asc,
            }),
            Token::Keyword(Keyword::Desc) => Ok(Expression::UnaryOperation {
                operand: Box::new(left),
                operator: UnaryOperator::Desc,
            }),
            _ => Err(format!("Unexpected operator: {:?}", token)),
        }
    }
// parses expressions (salary + 1111, age > 21) with operator precedence 
    pub fn parse_expression(&mut self, rbp: u32) -> Result<Expression, String> {
        let mut left = self.parse_prefix()?;
        while rbp < self.get_infix_precedence() {
            left = self.parse_infix(left)?;
        }
        Ok(left)
    }

    fn parse_column_definition(&mut self) -> Result<TableColumn, String> {
    let column_name = if let Token::Identifier(name) = &self.current_token {
        let name = name.clone();
        self.advance()?;
        name
    } else {
        return Err("Expected column name".to_string());
    };
    let column_type = match &self.current_token {
        Token::Keyword(Keyword::Int) => { self.advance()?; DBType::Int },
        Token::Keyword(Keyword::Bool) => { self.advance()?; DBType::Bool },
        Token::Keyword(Keyword::Varchar) => {
            self.advance()?;
            if self.current_token != Token::LeftParentheses {
                return Err("Expected '(' after VARCHAR".to_string());
            }
            self.advance()?;
            let length = if let Token::Number(n) = self.current_token {
                let len = n as usize;
                self.advance()?;
                len
            } else {
                return Err("Expected number for VARCHAR length".to_string());
            };
            if self.current_token != Token::RightParentheses {
                return Err("Expected ')' after VARCHAR length".to_string());
            }
            self.advance()?;
            DBType::Varchar(length)
        },
        _ => return Err("Expected type (INT, BOOL, VARCHAR)".to_string()),
    };
    let mut constraints = Vec::new();
    while matches!(self.current_token, Token::Keyword(Keyword::Primary | Keyword::Not | Keyword::Check | Keyword::Foreign)) {
        match self.current_token {
            Token::Keyword(Keyword::Primary) => {
                self.advance()?;
                if self.current_token != Token::Keyword(Keyword::Key) {
                    return Err("Expected KEY after PRIMARY".to_string());
                }
                self.advance()?;
                constraints.push(Constraint::PrimaryKey);
            }
            Token::Keyword(Keyword::Not) => {
                self.advance()?;
                if self.current_token != Token::Keyword(Keyword::Null) {
                    return Err("Expected NULL after NOT".to_string());
                }
                self.advance()?;
                constraints.push(Constraint::NotNull);
            }
            Token::Keyword(Keyword::Check) => {
                self.advance()?;
                if self.current_token != Token::LeftParentheses {
                    return Err("Expected '(' after CHECK".to_string());
                }
                self.advance()?;
                let expr = self.parse_expression(0)?;
                if self.current_token != Token::RightParentheses {
                    return Err("Expected ')' after CHECK expression".to_string());
                }
                self.advance()?;
                constraints.push(Constraint::Check(expr));
            }
            Token::Keyword(Keyword::Foreign) => {
                self.advance()?;
                if self.current_token != Token::Keyword(Keyword::Key) {
                    return Err("Expected KEY after FOREIGN".to_string());
                }
                self.advance()?;
                if self.current_token != Token::LeftParentheses {
                    return Err("Expected '(' after FOREIGN KEY".to_string());
                }
                self.advance()?;
                let fk_column = if let Token::Identifier(name) = &self.current_token {
                    let name = name.clone();
                    self.advance()?;
                    name
                } else {
                    return Err("Expected column name in FOREIGN KEY".to_string());
                };
                if self.current_token != Token::RightParentheses {
                    return Err("Expected ')' after column name".to_string());
                }
                self.advance()?;
                if self.current_token != Token::Keyword(Keyword::References) {
                    return Err("Expected REFERENCES after FOREIGN KEY".to_string());
                }
                self.advance()?;
                let ref_table = if let Token::Identifier(name) = &self.current_token {
                    let name = name.clone();
                    self.advance()?;
                    name
                } else {
                    return Err("Expected table name in REFERENCES".to_string());
                };
                if self.current_token != Token::LeftParentheses {
                    return Err("Expected '(' after table name".to_string());
                }
                self.advance()?;
                let ref_column = if let Token::Identifier(name) = &self.current_token {
                    let name = name.clone();
                    self.advance()?;
                    name
                } else {
                    return Err("Expected column name in REFERENCES".to_string());
                };
                if self.current_token != Token::RightParentheses {
                    return Err("Expected ')' after reference column".to_string());
                }
                self.advance()?;
                constraints.push(Constraint::ForeignKey {
                    column: fk_column,
                    references_table: ref_table,
                    references_column: ref_column,
                });
            }
            _ => unreachable!(),
        }
    }
    Ok(TableColumn { column_name, column_type, constraints })
}
// Parses CREATE TABLE statement (table name, columns, constraints)
    pub fn parse_create_table(&mut self) -> Result<Statement, String> {
    self.advance()?;
    let table_name = if let Token::Identifier(name) = &self.current_token {
        let name = name.clone();
        self.advance()?;
        name
    } else {
        return Err("Expected table name".to_string());
    };
    if self.current_token != Token::LeftParentheses {
        return Err("Expected '(' after table name".to_string());
    }
    self.advance()?;
    let mut column_list = Vec::new();
    if self.current_token != Token::RightParentheses {
        loop {
            column_list.push(self.parse_column_definition()?);
            if self.current_token == Token::Comma {
                self.advance()?;
            } else if self.current_token == Token::RightParentheses {
                break;
            } else {
                return Err("Expected ',' or ')'".to_string());
            }
        }
    }
    self.advance()?;
    if self.current_token != Token::Semicolon {
        return Err("Expected ';'".to_string());
    }
    self.advance()?;
    Ok(Statement::CreateTable { table_name, column_list })
}
// Parses CREATE INDEX and CREATE UNIQUE INDEX statements
    pub fn parse_create_index(&mut self, is_unique: bool) -> Result<Statement, String> {
        self.advance()?; 
        let index_name = match &self.current_token {
            Token::Identifier(name) => name.clone(),
            _ => return Err("Expected index name".to_string()),
        };
        self.advance()?;
        if self.current_token != Token::Keyword(Keyword::On) {
            return Err("Expected ON after index name".to_string());
        }
        self.advance()?;
        let table_name = match &self.current_token {
            Token::Identifier(name) => name.clone(),
            _ => return Err("Expected table name".to_string()),
        };
        self.advance()?;
        if self.current_token != Token::LeftParentheses {
            return Err("Expected '(' after table name".to_string());
        }
        self.advance()?;
        let mut columns = Vec::new();
        loop {
            match &self.current_token {
                Token::Identifier(col) => {
                    columns.push(col.clone());
                    self.advance()?;
                }
                _ => return Err("Expected column name".to_string()),
            }
            if self.current_token == Token::RightParentheses {
                self.advance()?;
                break;
            }
            if self.current_token != Token::Comma {
                return Err("Expected ',' or ')' after column name".to_string());
            }
            self.advance()?;
        }
        if self.current_token != Token::Semicolon {
            return Err("Expected ';' after index definition".to_string());
        }
        self.advance()?;
        if is_unique {
            Ok(Statement::CreateUniqueIndex {
                index_name,
                table_name,
                columns,
            })
        } else {
            Ok(Statement::CreateIndex {
                index_name,
                table_name,
                columns,
            })
        }
    }
// (!)Entry point for parsing 
// Decides which statement to parse (SELECT, CREATE, etc)
    pub fn parse(&mut self) -> Result<Statement, String> {
    match self.current_token {
        Token::Keyword(Keyword::Select) => self.parse_select(),
        Token::Keyword(Keyword::Create) => {
            self.advance()?;
            match self.current_token {
                Token::Keyword(Keyword::Table) => self.parse_create_table(),
                Token::Keyword(Keyword::Index) => self.parse_create_index(false),
                Token::Keyword(Keyword::Unique) => {
                    self.advance()?;
                    if self.current_token == Token::Keyword(Keyword::Index) {
                        self.parse_create_index(true)
                    } else {
                        Err("Expected INDEX after UNIQUE".to_string())
                    }
                }
                _ => Err("Expected TABLE, INDEX, or UNIQUE after CREATE".to_string()),
            }
        }
        _ => Err("Expected SELECT or CREATE".to_string()),
    }
    }
}
