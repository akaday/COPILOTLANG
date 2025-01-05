use lexer::{Lexer, Token};

#[test]
fn test_multiline_comment_integration() {
    let input = "/* This is a multi-line comment */";
    let mut lexer = Lexer::new(input.to_string());
    let token = lexer.next_token();
    assert_eq!(token, Token::MultiLineComment(" This is a multi-line comment ".to_string()));
}

#[test]
fn test_unterminated_multiline_comment_integration() {
    let input = "/* This is an unterminated multi-line comment";
    let mut lexer = Lexer::new(input.to_string());
    let token = lexer.next_token();
    assert_eq!(token, Token::Error("Unterminated multi-line comment".to_string()));
}

#[test]
fn test_string_literal_with_escape_sequences_integration() {
    let input = r#""Hello, world!\nThis is a string with escape sequences.""#;
    let mut lexer = Lexer::new(input.to_string());
    let token = lexer.next_token();
    assert_eq!(token, Token::StringLiteral("Hello, world!\nThis is a string with escape sequences.".to_string()));
}

#[test]
fn test_unterminated_string_literal_integration() {
    let input = r#""This is an unterminated string literal"#;
    let mut lexer = Lexer::new(input.to_string());
    let token = lexer.next_token();
    assert_eq!(token, Token::Error("Unterminated string literal".to_string()));
}

#[test]
fn test_invalid_escape_sequence_integration() {
    let input = r#""This string contains an invalid escape sequence: \x""#;
    let mut lexer = Lexer::new(input.to_string());
    let token = lexer.next_token();
    assert_eq!(token, Token::Error("Invalid escape sequence".to_string()));
}
