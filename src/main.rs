mod parser;
use std::io;

fn main() {
    main_loop();
}

fn main_loop() -> !{
    loop{
        let mut expression = "".to_string();
        io::stdin().read_line(&mut expression).unwrap();
        println!("=\n{}\n", parser::evaluate(expression.trim().to_string()));
    }
}
