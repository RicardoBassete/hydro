use std::{env, fs, process::exit};

mod tokenization;

use tokenization::Tokenizer;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Incorrect usage");
        eprintln!("correct usage is: tests <input.hy>");
        exit(1);
    }

    let input_path = &args[1];

    let contents = match fs::read_to_string(input_path) {
        Ok(file_contents) => file_contents,
        Err(_) => {
            eprintln!("Cannot read file");
            exit(1)
        }
    };

    let mut tokenizer = Tokenizer::new(contents);
    let tokens = tokenizer.tokenize();
    let asm = tokenizer.to_asm(tokens);

    println!("{asm}");

    match fs::write("out.asm", asm) {
        Err(err) => {
            eprintln!("{err}");
            exit(1);
        }
        _ => {}
    };

    match std::process::Command::new("nasm")
        .args(["-felf64", "out.asm"])
        .output()
    {
        Err(err) => {
            eprintln!("{err}");
            exit(1);
        }
        _ => {}
    };
    match std::process::Command::new("ld")
        .args(["-o", "out", "out.o"])
        .output()
    {
        Err(err) => {
            eprintln!("{err}");
            exit(1);
        }
        _ => {}
    };
}
