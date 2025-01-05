mod lexer;

fn main() {
    let input = r#"
        let x: int = 10;
        function main() {
            let y: int = 20;
            return y + x;
        }
        /* This is a multi-line comment */
        let z: int = 30;
        let str = "Hello, world!\nThis is a string with escape sequences.";
    "#;
    let mut lexer = lexer::Lexer::new(input.to_string());

    loop {
        let token = lexer.next_token();
        println!("{:?}", token);
        if token == lexer::Token::Eof {
            break;
        }
    }
}
