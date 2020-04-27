use std::io;
mod lexer;

fn main() {
    println!("rust/mnky 1.0");

    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("failed to read line");

        lexer::Lexer::new(input);
    }
}
