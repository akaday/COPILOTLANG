mod lexer;
mod parser;

fn main() {
    let input = "let x: int = 10; function main() { let y: int = 20; return y + x; if (true) { return false; } else { return true; } for (let i: int = 0; i < 10; i = i + 1) { while (i < 5) { i = i + 1; } } }";
    let mut lexer = lexer::Lexer::new(input.to_string());

    let mut parser = parser::Parser::new(&mut lexer);
    let ast = parser.parse_program();
    println!("{:?}", ast);
}
