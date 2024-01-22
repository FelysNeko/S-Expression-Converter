use clap::{Parser, ValueEnum};


#[derive(Parser)]
#[command(about="S-Expression Converter")]
struct Args {
    #[arg(short, long, help="prettier indented output")]
    pretty: bool,

    #[arg(value_enum)]
    mode: Mode,

    #[arg(help="expression or file path depends on your mode")]
    input: String,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Mode {
    Expr,
    File,
}

#[derive(Debug, PartialEq)]
enum TokenType {
    Identifier,
    BinaryOprant,
    UnaryOprant,
    OpenParenthesis,
    CloseParenthesis,
    Number,
    None,
}

#[derive(Debug)]
struct Token {
    typing: TokenType,
    value: String,
}

impl Token {
    fn new(t: TokenType) -> Self {
        Self {
            typing: t,
            value: String::new(),
        }
    }

    fn update(&mut self, t: TokenType) {
        self.typing = t;
    }

    fn push(&mut self, c: char) {
        self.value.push(c);
    }

    fn is(&self, typing: TokenType) -> bool {
        if self.typing == typing {
            return true;
        } else {
            return false;
        }
    }
}

fn main() {
    let args: Args = Args::parse();
    println!(">>> {} {:?} {:?}", args.pretty, args.mode, args.input);
    
    
    let result: Vec<Token> = tokenize(args.input);
    for each in result {
        println!("{:?}", each);
    }

}


fn tokenize(line: String) -> Vec<Token> {
    let mut parsed: Vec<Token> = Vec::new();
    let mut none: Token = Token::new(TokenType::None);
    parsed.push(Token::new(TokenType::None));

    for c in line.chars() {
        let prev: &mut Token = parsed.last_mut().unwrap_or(&mut none);

        if c.is_ascii_alphabetic() {
            match prev.typing {
                TokenType::Identifier => prev.push(c),
                _ => {
                    let mut new: Token = Token::new(TokenType::Identifier);
                    new.push(c);
                    parsed.push(new);
                }
            }
        } else if c.is_ascii_digit() || c=='.' {
            match prev.typing {
                TokenType::Identifier | 
                TokenType::Number => prev.push(c),
                _ => {
                    let mut new: Token = Token::new(TokenType::Number);
                    new.push(c);
                    parsed.push(new);
                }
            }
        } else if c=='+' || c=='-' {
            match prev.typing {
                TokenType::BinaryOprant | 
                TokenType::OpenParenthesis |
                TokenType::None => {
                    let mut new: Token = Token::new(TokenType::UnaryOprant);
                    new.push(c);
                    parsed.push(new);
                }
                _ => {
                    let mut new: Token = Token::new(TokenType::BinaryOprant);
                    new.push(c);
                    parsed.push(new);
                }
            }
        } else if c=='*' || c=='/' {
            let mut new: Token = Token::new(TokenType::BinaryOprant);
            new.push(c);
            parsed.push(new);
        }
    }

    parsed
}