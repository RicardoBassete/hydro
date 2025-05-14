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

pub struct Tokenizer {
    src: String,
    current_index: usize,
}

impl Tokenizer {
    pub fn new(src: String) -> Tokenizer {
        Tokenizer {
            current_index: 0,
            src: src,
        }
    }

    fn peak(&self, ahead: usize) -> Option<char> {
        if self.current_index + ahead >= self.src.len() {
            return None;
        } else {
            let chars: Vec<char> = self.src.chars().collect();
            return Some(chars[self.current_index]);
        }
    }

    fn consume(&mut self) -> char {
        let chars: Vec<char> = self.src.chars().collect();
        let result = chars[self.current_index];
        self.current_index += 1;
        result
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();
        let mut buf: String = String::new();

        while let Some(c) = self.peak(0) {
            if c.is_alphabetic() {
                buf.push(self.consume());
                while let Some(c) = self.peak(1) {
                    if c.is_alphanumeric() {
                        buf.push(self.consume());
                        continue;
                    } else {
                        break;
                    }
                }
                if buf == "exit" {
                    tokens.push(Token::new(TokenType::Exit, None));
                    buf.clear();
                    continue;
                } else {
                    eprintln!("No tokens found");
                    exit(1);
                }
            } else if c.is_ascii_digit() {
                buf.push(self.consume());
                while let Some(c) = self.peak(1) {
                    if c.is_ascii_digit() {
                        buf.push(self.consume());
                        continue;
                    } else {
                        break;
                    }
                }
                tokens.push(Token::new(TokenType::IntLit, Some(buf.clone())));
                buf.clear();
                continue;
            } else if c == ';' {
                tokens.push(Token {
                    token_type: TokenType::Semi,
                    value: None,
                });
                self.consume();
                continue;
            } else if c.is_whitespace() {
                self.consume();
            } else {
                println!("{}", self.current_index);
                eprintln!("No tokens found");
                exit(1);
            }
        }

        self.current_index = 0;
        tokens
    }

    pub fn to_asm(&mut self, tokens: Vec<Token>) -> String {
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
}
