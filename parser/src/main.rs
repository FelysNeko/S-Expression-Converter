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
    BinaryOp,
    UnaryOp,
    OpenParen,
    CloseParen,
    Number,
    Start,
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

    fn push(&mut self, c: char) {
        self.value.push(c);
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
    let mut parsed: Vec<Token> = vec![Token::new(TokenType::Start)];

    for c in line.chars() {
        // this line will never panic since `parsed` is guaranteed to have at least one element
        let prev: &mut Token = parsed.last_mut().expect("authored-by-FelysNeko");

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
            let mut new: Token = match prev.typing {
                TokenType::UnaryOp |
                TokenType::BinaryOp |
                TokenType::OpenParen |
                TokenType::Start => Token::new(TokenType::UnaryOp),
                _ => Token::new(TokenType::BinaryOp),
            };
            new.push(c);
            parsed.push(new);
        }  else if c == '=' {
            if prev.value==">" || prev.value=="=" || prev.value=="<" {
                prev.push(c)
            } else {
                let mut new: Token = Token::new(TokenType::BinaryOp);
                new.push(c);
                parsed.push(new);
            }
        } else if c=='*' || c=='/' || c=='>' || c=='<'{
            let mut new: Token = Token::new(TokenType::BinaryOp);
            new.push(c);
            parsed.push(new);
        } else if c == '!' {
            let mut new: Token = Token::new(TokenType::UnaryOp);
            new.push(c);
            parsed.push(new);
        } else if c == '(' {
            let mut new: Token = Token::new(TokenType::OpenParen);
            new.push(c);
            parsed.push(new);
        } else if c == ')' {
            let mut new: Token = Token::new(TokenType::CloseParen);
            new.push(c);
            parsed.push(new);
        }
    }

    return parsed;
}