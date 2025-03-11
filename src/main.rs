//use std::collections::HashMap;
use rug::{Integer, Float};
use clap::Parser;
use std::fs;

//struct Function {
//    name: String,
//    body: (Box<Function>, Vec<String>)
//}
//
//enum Primitive {
//    Integer(Integer),
//    Float(Float),
//    Bool(bool),
//    String(String),
//}
//
//enum Union {
//    Vector(Vec<Primitive>),
//}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the file to load
    #[arg()]
    filename: String,
}

fn main() {
    let args = Args::parse();
    let target_file = args.filename;

    let contents: String = match fs::read_to_string(&target_file) {
        Ok(data) => data,
        Err(err) => {
            eprintln!("Can't open file '{}': {}", target_file, err);
            std::process::exit(0x02);
        }
    };

    let tokens = tokenize(contents);

    println!("{:?}", tokens);
}

#[derive(Debug)]
enum Token {
    OpenParen,
    CloseParen,
    Integer(Integer),
    Builtin(Builtin),
    User(String),
}

#[derive(Debug)]
enum Builtin {
    Plus,
    Minus,
    If,
    Defun,
}

fn push_word(word_buffer: &mut String, result: &mut Vec<Token>) {
    match &word_buffer[..] {
        "" => return,
        "+" => { result.push(Token::Builtin(Builtin::Plus)) }
        "-" => { result.push(Token::Builtin(Builtin::Minus)) }
        "if" => { result.push(Token::Builtin(Builtin::If)) }
        "defun" => { result.push(Token::Builtin(Builtin::Defun)) }
        _ => {
            if let Ok(integer) = Integer::from_str_radix(&word_buffer[..], 10) {
                result.push(Token::Integer(integer))
            } else {
                result.push(Token::User(word_buffer.clone()))
            }
        }
    }

    *word_buffer = String::new();
}

fn tokenize(code: String) -> Vec<Token> {
    let mut result = Vec::new();
    let mut word_buffer = String::new();
    for (i, ch) in code.chars().enumerate() {
        match ch {
            '(' => { push_word(&mut word_buffer, &mut result); result.push(Token::OpenParen) }
            ')' => { push_word(&mut word_buffer, &mut result); result.push(Token::CloseParen) }
            ' ' | '\n' => { push_word(&mut word_buffer, &mut result); }
            _ => { word_buffer.push(ch) }
        }
    }
    result
}

