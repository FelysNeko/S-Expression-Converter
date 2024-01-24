use std::process::exit;
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

    fn null() -> Self {
        Token::new(TokenType::Null)
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

    fn push(&mut self, tk: Token) {
        self.data.push(tk);
    }
}

#[derive(Debug)]
struct Node {
    typing: TokenType,
    value: String,
    branch: Vec<Node>,
}

impl Node {
    fn new(tk: Token) -> Self {
        Self {
            typing: tk.typing,
            value: tk.value,
            branch: Vec::new(),
        }
    }

    fn push(&mut self, n: Node) {
        self.branch.push(n);
    }
}


struct ASTParser {
    lexer: Lexer,
}

impl ASTParser {
    fn new(lxr: Lexer) -> ASTParser {
        ASTParser { lexer: lxr }
    }

    fn parse(&mut self) -> Node {
        return self.compare();
    }

    fn compare(&mut self) -> Node {
        let left: Node = self.add();
        return left;
    }

    fn add(&mut self) -> Node {
        let left: Node = self.multi();
        return left;
    }

    fn multi(&mut self) -> Node {
        let left: Node = self.unary();
        return left;
    }

    fn unary(&mut self) -> Node {
        if let Some(tk) = self.lexer.next() {
            match tk.typing {
                TokenType::UnaryOper => {
                    let mut this: Node = Node::new(tk);
                    this.push(self.primary());
                    this
                },
                _ => {
                    self.lexer.push(tk);
                    self.primary()
                },
            }
        } else {
            exit(1);
        }
    }

    fn primary(&mut self) -> Node {
        if let Some(tk) = self.lexer.next() {
            match tk.typing {
                TokenType::Identifier |
                TokenType::Numerical => Node::new(tk),
                _ => {
                    println!("Error at {}", tk.value);
                    exit(1)
                },
            }
        } else {
            exit(1);
        }
    }
}


fn main() {
    let args: Args = Args::parse();
    // if args.debug == true {
    //     println!("");
    //     println!("Pretty: {}", args.pretty);
    //     println!("Debug: {}", args.debug);
    //     println!("Mode: {:?}", args.mode);
    //     println!("Expr: {:?}", args.input);
    // }

    let lexer: Lexer = Lexer::new(args.input);
    // if args.debug == true {
    //     println!("");
    //     while let Some(token) = lexer.next() {
    //         println!("+ {:?}\t{}", token.typing, token.value);
    //     }
    // }

    let mut parser: ASTParser = ASTParser::new(lexer);
    let root = parser.parse();
    println!("{:#?}", root);
}


fn tokenize(line: String) -> Vec<Token> {
    let mut result: Vec<Token> = vec![Token::null()];
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
        } else if c=='!' || c=='|' || c=='&' || c=='^' || c=='~'{
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