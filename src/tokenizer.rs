use crate::token::{Token, lookup_keyword};
//Tokenizer splits SQL input into tokens ("SELECT name" into Keyword::Select, Identifier("name"))
pub struct Tokenizer<'a> {
    input: &'a str,
    position: usize,
}

impl<'a> Tokenizer<'a> {
    // creates a new Tokenizer for the input SQL string
    pub fn new(input: &'a str) -> Self {
        Tokenizer { input, position: 0 }
    }

    pub fn next_char(&self) -> Option<char> {
        self.input[self.position..].chars().next()
    }

    pub fn advance(&mut self) {
        if let Some(ch) = self.next_char() {
            self.position += ch.len_utf8();
        }
    }

    pub fn skip_whitespace(&mut self) {
        while let Some(ch) = self.next_char() {
            if ch.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }
        //Returns the next token (keyword, identifier, number) or an error
    pub fn next_token(&mut self) -> Result<Token, String> {
    self.skip_whitespace();
    let Some(ch) = self.next_char() else { return Ok(Token::Eof) };

    match ch {
        '(' => { self.advance(); Ok(Token::LeftParentheses) }
        ')' => { self.advance(); Ok(Token::RightParentheses) }
        '+' => { self.advance(); Ok(Token::Plus) }
        '-' => { self.advance(); Ok(Token::Minus) }
        '*' => { self.advance(); Ok(Token::Star) }
        '/' => { self.advance(); Ok(Token::Divide) }
        ',' => { self.advance(); Ok(Token::Comma) }
        ';' => { self.advance(); Ok(Token::Semicolon) }
        '=' => { self.advance(); Ok(Token::Equal) }
        '!' => {
            self.advance();
            if self.next_char() == Some('=') {
                self.advance();
                Ok(Token::NotEqual)
            } else {
                Ok(Token::Invalid('!'))
            }
        }
        '<' => {
            self.advance();
            if self.next_char() == Some('=') {
                self.advance();
                Ok(Token::LessThanOrEqual)
            } else {
                Ok(Token::LessThan)
            }
        }
        '>' => {
            self.advance();
            if self.next_char() == Some('=') {
                self.advance();
                Ok(Token::GreaterThanOrEqual)
            } else {
                Ok(Token::GreaterThan)
            }
        }
        '"' | '\'' => {
            let quote = ch;
            self.advance();
            let mut result = String::new();
            while let Some(c) = self.next_char() {
                if c == quote {
                    self.advance();
                    return Ok(Token::String(result));
                }
                result.push(c);
                self.advance();
            }
            Err(format!("Unmatched {} quote", if quote == '"' { "double" } else { "single" }))
        }
        c if c.is_ascii_digit() => {
            let mut number = String::new();
            while let Some(c) = self.next_char() {
                if c.is_ascii_digit() {
                    number.push(c);
                    self.advance();
                } else {
                    break;
                }
            }
            Ok(Token::Number(number.parse::<u64>().unwrap()))
        }
        c if c.is_alphabetic() || c == '_' => {
            let mut ident = String::new();
            while let Some(c) = self.next_char() {
                if c.is_alphanumeric() || c == '_' {
                    ident.push(c);
                    self.advance();
                } else {
                    break;
                }
            }
            Ok(if let Some(keyword) = lookup_keyword(&ident) {
                match ident.as_str() {
                    "SELECT" | "CREATE" | "TABLE" | "WHERE" | "ORDER" | "BY" | "ASC" | "DESC" |
                    "FROM" | "AND" | "OR" | "NOT" | "TRUE" | "FALSE" | "PRIMARY" | "KEY" |
                    "CHECK" | "INT" | "BOOL" | "VARCHAR" | "NULL" | "UNIQUE" | "INDEX" | "ON" |
                    "FOREIGN" | "REFERENCES" => Token::Keyword(keyword),
                    _ => Token::Identifier(ident),
                }
            } else {
                Token::Identifier(ident)
            })
        }
        _ => {
            self.advance();
            Ok(Token::Invalid(ch))
        }
    }
}
}
