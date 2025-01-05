<<<<<<< HEAD
fn main() {
    println!("Hello, world!");
=======
mod lexer;

fn main() {
    let input = "let x: int = 10; function main() { let y: int = 20; return y + x; }";
    let mut lexer = lexer::Lexer::new(input.to_string());

    loop {
        let token = lexer.next_token();
        println!("{:?}", token);
        if token == lexer::Token::Eof {
            break;
        }
    }
>>>>>>> f318f4ac4247bda0151968c691b2e5a12c2878a7
}
