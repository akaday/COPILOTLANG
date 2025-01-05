mod lexer;

fn main() {
    let input = "let x: int = 10;";
    let mut lexer = lexer::Lexer::new(input.to_string());
    let token = lexer.next_token();
    println!("{:?}", token);
}
