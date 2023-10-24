use crate::TokenType::{_Return, IntLit, Semicolon};

fn main() {
    let contents = scan_file(String::from(("test.lg")));

    let tokens = tokenize(contents);

    println!("{:#?}", tokens);
}

#[derive(Debug)]
enum TokenType {
    _Return,
    IntLit,
    Semicolon,
}

#[derive(Debug)]
struct Token {
    token_type: TokenType,
    value: Option<String>,
}

fn scan_file(filename: String) -> String {
    return std::fs::read_to_string(filename).expect("Failed to parse file!").replace("\"", "");
}


fn tokenize(str: String) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut buffer: Vec<char> = Vec::new();
    let mut next_index = 0;

    for (i, x) in str.chars().enumerate() { // Access every (c)haracter individually in the source code
        next_index = i + 1;
        let c = str.chars().nth(i).unwrap();



        if c.is_alphabetic() {
            buffer.push(c);


            while str.chars().nth(next_index).unwrap().is_alphanumeric() {
                buffer.push(str.chars().nth(next_index).unwrap());
                next_index += 1;
            }
            next_index -= 1;

            let word = String::from_iter(&buffer);
            if word == String::from("return") {
                tokens.push(Token { token_type: _Return, value: None });
                buffer.clear();
            }

            buffer.clear();

        } else if c.is_numeric() {
            buffer.push(c);

            while str.chars().nth(next_index).unwrap().is_numeric() {
                buffer.push(str.chars().nth(next_index).unwrap());
                next_index += 1;
            }
            next_index -= 1;
            tokens.push(Token {
                token_type: IntLit,
                value: Some(String::from_iter(&buffer)),
            });
            buffer.clear();

        } else if c == ';' {
            tokens.push(
                Token {
                    token_type: Semicolon,
                    value: None
                }
            );

            continue;
        } else if c.is_whitespace() {
            continue;
        } else {
            panic!("Mission Failed");
        }
    }

    return tokens;
}