use clap::{ Parser, ValueEnum };


#[derive(Parser)]
#[command(about="S-Expression Converter")]
struct Args {
    #[arg(short, long, help="prettier indented output")]
    pretty: bool,

    #[arg(short, long, help="show debug information")]
    debug: bool,

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
    BinaryOper,
    UnaryOper,
    OpenParen,
    CloseParen,
    FuncCall,
    ParamSplit,
    Numerical,
    Null,
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

    fn update(&mut self, t: TokenType) {
        self.typing = t;
    }
}


struct Lexer {
    data: Vec<Token>,
}

impl Lexer {
    fn new(line: String) -> Self {
        let mut result: Vec<Token> = tokenize(line);
        result.reverse();
        result.pop();
        return Self {data: result};
    }

    fn next(&mut self) -> Option<Token> {
        return self.data.pop();
    }
}


fn main() {
    let args: Args = Args::parse();
    if args.debug == true {
        println!("");
        println!("Pretty: {}", args.pretty);
        println!("Debug: {}", args.debug);
        println!("Mode: {:?}", args.mode);
        println!("Expr: {:?}", args.input);
    }

    let mut lexer: Lexer = Lexer::new(args.input);
    if args.debug == true {
        println!("");
        while let Some(token) = lexer.next() {
            println!("+ {:?}\t{}", token.typing, token.value);
        }
    }

}


fn tokenize(line: String) -> Vec<Token> {
    let mut result: Vec<Token> = vec![Token::new(TokenType::Null)];
    for c in line.chars() {
        // this line will never panic since `result` is guaranteed to have at least one element
        let prev: &mut Token = result.last_mut().expect("authored-by-FelysNeko");

        if c.is_ascii_alphabetic() {
            match prev.typing {
                TokenType::Identifier => prev.push(c),
                _ => {
                    let mut new: Token = Token::new(TokenType::Identifier);
                    new.push(c);
                    result.push(new);
                }
            }
        } else if c.is_ascii_digit() || c=='.' {
            match prev.typing {
                TokenType::Identifier | 
                TokenType::Numerical => prev.push(c),
                _ => {
                    let mut new: Token = Token::new(TokenType::Numerical);
                    new.push(c);
                    result.push(new);
                }
            }
        } else if c == '=' {
            if prev.value==">" || prev.value=="=" || prev.value=="<" || prev.value=="!"{
                prev.update(TokenType::BinaryOper);
                prev.push(c);
            } else {
                let mut new: Token = Token::new(TokenType::BinaryOper);
                new.push(c);
                result.push(new);
            }
        } else if c=='!' || c=='|' || c=='&' || c=='^' || c=='^'{
            if prev.value == String::from(c) {
                prev.update(TokenType::BinaryOper);
                prev.push(c);
            } else {
                let mut new: Token = Token::new(TokenType::UnaryOper);
                new.push(c);
                result.push(new);
            }
        } else if c=='+' || c=='-' {
            let mut new: Token = match prev.typing {
                TokenType::UnaryOper |
                TokenType::BinaryOper |
                TokenType::OpenParen |
                TokenType::Null => Token::new(TokenType::UnaryOper),
                _ => Token::new(TokenType::BinaryOper),
            };
            new.push(c);
            result.push(new);
        } else if c=='*' || c=='/' || c=='>' || c=='<' {
            let mut new: Token = Token::new(TokenType::BinaryOper);
            new.push(c);
            result.push(new);
        } else if c == '(' {
            if prev.typing == TokenType::Identifier {
                prev.update(TokenType::FuncCall);
            }
            let mut new: Token =Token::new(TokenType::OpenParen);
            new.push(c);
            result.push(new);
        } else if c == ')' {
            let mut new: Token =Token::new(TokenType::CloseParen);
            new.push(c);
            result.push(new);
        } else if c == ',' {
            let mut new: Token =Token::new(TokenType::ParamSplit);
            new.push(c);
            result.push(new);
        }
    }

    return result;
}