use std::process::exit;

#[derive(PartialEq, Eq)]
pub enum TokenType {
    Exit,
    IntLit,
    Semi,
}

pub struct Token {
    pub token_type: TokenType,
    pub value: Option<String>,
}

impl Token {
    fn new(token_type: TokenType, value: Option<String>) -> Token {
        return Token {
            token_type: token_type,
            value: value,
        };
    }
}

pub fn tokenize(contents: String) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut buf: String = String::new();
    let chars: Vec<char> = contents.chars().collect();

    let mut i = 0;
    while i < contents.len() {
        if i != 0 {
            i += 1;
        }
        let c = chars[i];
        if c.is_alphabetic() {
            i += 1;
            buf.push(c);
            while chars[i].is_alphanumeric() {
                buf.push(chars[i]);
                i += 1;
            }
            i -= 1;

            if buf == "exit" {
                tokens.push(Token::new(TokenType::Exit, None));
                buf.clear();

                continue;
            } else {
                eprintln!("No tokens found");
                exit(1);
            }
        } else if c.is_numeric() {
            buf.push(c);
            while chars[i].is_numeric() {
                i += 1;
                buf.push(chars[i]);
            }
            i -= 1;
            tokens.push(Token::new(TokenType::IntLit, Some(buf.clone())));
            buf.clear();
            continue;
        } else if c == ';' {
            tokens.push(Token::new(TokenType::Semi, Some(c.to_string())));
            i += 1;
        } else if c.is_whitespace() {
            continue;
        } else {
            eprintln!("No tokens found");
            exit(1);
        }
        i += 1;
    }

    tokens
}

pub fn tokens_to_asm(tokens: Vec<Token>) -> String {
    let mut output = String::from("global _start\n_start:\n");

    let mut i = 0;
    while i < tokens.len() {
        let token = &tokens[i];
        match token.token_type {
            TokenType::Exit => {
                if i + 1 < tokens.len() && tokens[i + 1].token_type == TokenType::IntLit {
                    if i + 2 < tokens.len() && tokens[i + 2].token_type == TokenType::Semi {
                        output.push_str("    mov rax, 60\n");
                        output.push_str(
                            &format!("    mov rdi, {}\n", tokens[i + 1].value.clone().unwrap())
                                .to_string(),
                        );
                        output.push_str("    syscall");
                    }
                }
                i += 1;
            }
            _ => {
                i += 1;
                continue;
            }
        }
        i += 1;
    }

    output
}
