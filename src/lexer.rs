#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Let,
    Function,
    Identifier(String),
    IntLiteral(i32),
    TypeInt,
    TypeVoid,
    Plus,
    Minus,
    Star,
    Slash,
    Equals,
    Colon,
    Semicolon,
    LParen,
    RParen,
    LBrace,
    RBrace,
    Return,
    If,
    Else,
    For,
    While,
    True,
    False,
    Comma,
    Eof,
    Error(String),
}

pub struct Lexer {
    input: Vec<char>,
    position: usize,
    nesting_level: usize,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        Self {
            input: input.chars().collect(),
            position: 0,
            nesting_level: 0,
        }
    }

    fn get_char(&self) -> Option<char> {
        self.input.get(self.position).cloned()
    }

    fn advance(&mut self) {
        self.position += 1;
    }

    fn is_identifier_char(c: char) -> bool {
        c.is_alphanumeric() || c == '_'
    }

    fn is_whitespace(c: char) -> bool {
        c == ' ' || c == '\t' || c == '\n' || c == '\r'
    }

    fn is_operator(c: char) -> bool {
        matches!(c, '+' | '-' | '*' | '/' | '=' | ':' | ';' | '(' | ')' | '{' | '}' | ',')
    }

    fn is_keyword(s: &str) -> bool {
        matches!(s, "let" | "function" | "int" | "void" | "return" | "if" | "else" | "for" | "while" | "true" | "false")
    }

    pub fn next_token(&mut self) -> Token {
        while let Some(current_char) = self.get_char() {
            match current_char {
                c if Lexer::is_whitespace(c) => self.advance(),
                'l' => {
                    self.advance();
                    if self.input[self.position..self.position + 2] == ['e', 't'] {
                        self.position += 2;
                        return Token::Let;
                    }
                }
                'f' => {
                    self.advance();
                    if self.input[self.position..self.position + 7] == ['u', 'n', 'c', 't', 'i', 'o', 'n'] {
                        self.position += 7;
                        return Token::Function;
                    }
                }
                'i' => {
                    self.advance();
                    if self.input[self.position..self.position + 1] == ['f'] {
                        self.position += 1;
                        return Token::If;
                    }
                }
                'e' => {
                    self.advance();
                    if self.input[self.position..self.position + 3] == ['l', 's', 'e'] {
                        self.position += 3;
                        return Token::Else;
                    }
                }
                'w' => {
                    self.advance();
                    if self.input[self.position..self.position + 4] == ['h', 'i', 'l', 'e'] {
                        self.position += 4;
                        return Token::While;
                    }
                }
                't' => {
                    self.advance();
                    if self.input[self.position..self.position + 3] == ['r', 'u', 'e'] {
                        self.position += 3;
                        return Token::True;
                    }
                }
                'f' => {
                    self.advance();
                    if self.input[self.position..self.position + 4] == ['a', 'l', 's', 'e'] {
                        self.position += 4;
                        return Token::False;
                    }
                }
                '0'..='9' => {
                    let start = self.position;
                    while let Some(ch) = self.get_char() {
                        if ch.is_digit(10) {
                            self.advance();
                        } else {
                            break;
                        }
                    }
                    let value: i32 = self.input[start..self.position].iter().collect::<String>().parse().unwrap();
                    return Token::IntLiteral(value);
                }
                '"' => {
                    self.advance();
                    let start = self.position;
                    while let Some(ch) = self.get_char() {
                        if ch == '"' {
                            let value = self.input[start..self.position].iter().collect::<String>();
                            self.advance();
                            return Token::Identifier(value);
                        } else {
                            self.advance();
                        }
                    }
                    return Token::Error("Unterminated string literal".to_string());
                }
                '/' => {
                    self.advance();
                    if let Some(next_char) = self.get_char() {
                        if next_char == '/' {
                            while let Some(ch) = self.get_char() {
                                if ch == '\n' {
                                    break;
                                } else {
                                    self.advance();
                                }
                            }
                        } else if next_char == '*' {
                            self.advance();
                            self.nesting_level += 1;
                            while let Some(ch) = self.get_char() {
                                if ch == '*' {
                                    self.advance();
                                    if let Some(next_ch) = self.get_char() {
                                        if next_ch == '/' {
                                            self.advance();
                                            self.nesting_level -= 1;
                                            if self.nesting_level == 0 {
                                                break;
                                            }
                                        }
                                    }
                                } else if ch == '/' {
                                    self.advance();
                                    if let Some(next_ch) = self.get_char() {
                                        if next_ch == '*' {
                                            self.advance();
                                            self.nesting_level += 1;
                                        }
                                    }
                                } else {
                                    self.advance();
                                }
                            }
                        } else {
                            return Token::Slash;
                        }
                    }
                }
                c if Lexer::is_identifier_char(c) => {
                    let start = self.position;
                    while let Some(ch) = self.get_char() {
                        if Lexer::is_identifier_char(ch) {
                            self.advance();
                        } else {
                            break;
                        }
                    }
                    let value = self.input[start..self.position].iter().collect::<String>();
                    if Lexer::is_keyword(&value) {
                        match value.as_str() {
                            "let" => return Token::Let,
                            "function" => return Token::Function,
                            "int" => return Token::TypeInt,
                            "void" => return Token::TypeVoid,
                            "return" => return Token::Return,
                            "if" => return Token::If,
                            "else" => return Token::Else,
                            "for" => return Token::For,
                            "while" => return Token::While,
                            "true" => return Token::True,
                            "false" => return Token::False,
                            _ => {}
                        }
                    } else {
                        return Token::Identifier(value);
                    }
                }
                c if Lexer::is_operator(c) => {
                    self.advance();
                    match c {
                        '+' => return Token::Plus,
                        '-' => return Token::Minus,
                        '*' => return Token::Star,
                        '/' => return Token::Slash,
                        '=' => return Token::Equals,
                        ':' => return Token::Colon,
                        ';' => return Token::Semicolon,
                        '(' => return Token::LParen,
                        ')' => return Token::RParen,
                        '{' => return Token::LBrace,
                        '}' => return Token::RBrace,
                        ',' => return Token::Comma,
                        _ => {}
                    }
                }
                _ => {
                    self.advance();
                    return Token::Error(format!("Invalid character: {}", current_char));
                }
            }
        }
        Token::Eof
    }
}
