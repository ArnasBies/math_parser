mod parser;
use std::io;

fn main() {
    main_loop();
}

fn main_loop() -> !{
    println!("Type exit to close the program, this is a simple math expression parser\n");
    loop{
        let mut expression = "".to_string();
        io::stdin().read_line(&mut expression).unwrap();
        if expression.to_lowercase().trim() == "exit".to_string(){
            std::process::exit(1);
        }
        println!("= {}\n", parser::evaluate(expression.trim().to_string()));
    }
}
