use std::fmt::{Debug, Display, Formatter};
//Defines tokens for SQL syntax such as keywords, identifiers, operators...
#[derive(PartialEq, Clone, Debug)]
pub enum Token {
    Keyword(Keyword),
    Identifier(String),
    String(String),
    Number(u64),
    Invalid(char),
    RightParentheses,
    LeftParentheses,
    GreaterThan,
    GreaterThanOrEqual,
    LessThan,
    LessThanOrEqual,
    Equal,
    NotEqual,
    Star,
    Divide,
    Minus,
    Plus,
    Comma,
    Semicolon,
    Eof,
    
}

#[derive(PartialEq, Clone, Debug)]
pub enum Keyword {
    Select,
    Create,
    Table,
    Where,
    Order,
    By,
    Asc,
    Desc,
    From,
    And,
    Or,
    Not,
    True,
    False,
    Primary,
    Key,
    Check,
    Int,
    Bool,
    Varchar,
    Null,
    Unique,
    Index,
    On,
    Foreign,
    References,
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Keyword(keyword) => write!(f, "{}", keyword),
            Token::Identifier(iden) => write!(f, "{:?}", iden),
            Token::String(str) => write!(f, "{:?}", str),
            Token::Number(num) => write!(f, "{:?}", num),
            Token::RightParentheses => write!(f, ")"),
            Token::LeftParentheses => write!(f, "("),
            Token::GreaterThan => write!(f, ">"),
            Token::GreaterThanOrEqual => write!(f, ">="),
            Token::LessThan => write!(f, "<"),
            Token::LessThanOrEqual => write!(f, "<="),
            Token::Equal => write!(f, "="),
            Token::NotEqual => write!(f, "!="),
            Token::Star => write!(f, "*"),
            Token::Divide => write!(f, "/"),
            Token::Minus => write!(f, "-"),
            Token::Plus => write!(f, "+"),
            Token::Comma => write!(f, ","),
            Token::Semicolon => write!(f, ";"),
            Token::Eof => write!(f, "Eof"),
            Token::Invalid(c) => write!(f, "{}", c),
        }
    }
}

impl Display for Keyword {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Keyword::Select => write!(f, "Select"),
            Keyword::Create => write!(f, "Create"),
            Keyword::Table => write!(f, "Table"),
            Keyword::Where => write!(f, "Where"),
            Keyword::Order => write!(f, "Order"),
            Keyword::By => write!(f, "By"),
            Keyword::Asc => write!(f, "Asc"),
            Keyword::Desc => write!(f, "Desc"),
            Keyword::From => write!(f, "From"),
            Keyword::And => write!(f, "And"),
            Keyword::Or => write!(f, "Or"),
            Keyword::Not => write!(f, "Not"),
            Keyword::True => write!(f, "True"),
            Keyword::False => write!(f, "False"),
            Keyword::Primary => write!(f, "Primary"),
            Keyword::Key => write!(f, "Key"),
            Keyword::Check => write!(f, "Check"),
            Keyword::Int => write!(f, "Int"),
            Keyword::Bool => write!(f, "Bool"),
            Keyword::Varchar => write!(f, "Varchar"),
            Keyword::Null => write!(f, "Null"),
            Keyword::Unique => write!(f, "Unique"),
            Keyword::Index => write!(f, "Index"),
            Keyword::On => write!(f, "On"),
            Keyword::Foreign => write!(f, "FOREIGN"),
            Keyword::References => write!(f, "REFERENCES"),
        }
    }
}

//Maps string identifiers to SQL keywords ("SELECT" to Keyword::Select)
pub fn lookup_keyword(ident: &str) -> Option<Keyword> {
    match ident.to_ascii_uppercase().as_str() {
        "SELECT" => Some(Keyword::Select),
        "CREATE" => Some(Keyword::Create),
        "TABLE" => Some(Keyword::Table),
        "WHERE" => Some(Keyword::Where),
        "ORDER" => Some(Keyword::Order),
        "BY" => Some(Keyword::By),
        "ASC" => Some(Keyword::Asc),
        "DESC" => Some(Keyword::Desc),
        "FROM" => Some(Keyword::From),
        "AND" => Some(Keyword::And),
        "OR" => Some(Keyword::Or),
        "NOT" => Some(Keyword::Not),
        "TRUE" => Some(Keyword::True),
        "FALSE" => Some(Keyword::False),
        "PRIMARY" => Some(Keyword::Primary),
        "KEY" => Some(Keyword::Key),
        "CHECK" => Some(Keyword::Check),
        "INT" => Some(Keyword::Int),
        "BOOL" => Some(Keyword::Bool),
        "VARCHAR" => Some(Keyword::Varchar),
        "NULL" => Some(Keyword::Null),
        "UNIQUE" => Some(Keyword::Unique),
        "INDEX" => Some(Keyword::Index),
        "ON" => Some(Keyword::On),
        "FOREIGN" => Some(Keyword::Foreign),
        "REFERENCES" => Some(Keyword::References),
        _ => None,
    }
}
