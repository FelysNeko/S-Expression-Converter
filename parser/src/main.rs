use std::process::exit;
use std::fmt;
use clap::{ Parser, ValueEnum };


#[derive(Parser)]
#[command(about="S-Expression Converter")]
struct Args {
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


#[derive(Debug, PartialEq, Clone)]
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


#[derive(Debug, Clone)]
struct Token {
    typing: TokenType,
    value: String,
}


struct Lexer {
    data: Vec<Token>,
}


#[derive(Debug, Clone)]
struct Node {
    typing: TokenType,
    value: String,
    branch: Vec<Node>,
}


struct ASTParser {
    lexer: Lexer,
}


fn main() {
    let args: Args = Args::parse();
    if args.debug == true {
        println!("");
        println!("Debug: {}", args.debug);
        println!("Mode: {:?}", args.mode);
        println!("Expr: {:?}", args.input);
    }

    let lexer: Lexer = Lexer::new(args.input);
    if args.debug == true {
        println!("");
        println!("{}", lexer);
    }

    let mut parser: ASTParser = ASTParser::new(lexer);
    let root = parser.parse();
    if args.debug == true {
        println!("");
        println!("{:#?}", root);
        println!("");
    }

    println!("{}", root);
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


impl Lexer {
    fn new(line: String) -> Self {
        let mut result: Vec<Token> = tokenize(line);
        result.reverse();
        result.pop();
        Self {data: result}
    }

    fn next(&mut self) -> Option<Token> {
        self.data.pop()
    }

    fn push(&mut self, tk: Token) {
        self.data.push(tk);
    }
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


impl ASTParser {
    fn new(lxr: Lexer) -> ASTParser {
        ASTParser { lexer: lxr }
    }

    fn parse(&mut self) -> Node {
        self.assign()
    }

    fn assign(&mut self) -> Node {
        let mut left: Node = self.compare();
        while let Some(tk) = self.lexer.next() {
            if tk.value == "=" {
                if left.typing != TokenType::Identifier {
                    println!("Error at {:?}", TokenType::BinaryOper);
                    exit(1);
                }
                let mut temp = Node::new(tk);
                temp.push(left.clone());
                temp.push(self.compare());
                left = temp;
            } else {
                self.lexer.push(tk);
                break;
            }
        }
        left
    }

    fn compare(&mut self) -> Node {
        let mut left: Node = self.add();
        while let Some(tk) = self.lexer.next() {
            if 
                tk.value==">" || tk.value==">=" || tk.value=="==" || 
                tk.value=="<" || tk.value=="<=" || tk.value=="!=" 
            {
                let mut temp = Node::new(tk);
                temp.push(left.clone());
                temp.push(self.add());
                left = temp;
            } else {
                self.lexer.push(tk);
                break;
            }
        }
        left
    }

    fn add(&mut self) -> Node {
        let mut left: Node = self.multi();
        while let Some(tk) = self.lexer.next() {
            if tk.value=="+" || tk.value=="-" {
                let mut temp = Node::new(tk);
                temp.push(left.clone());
                temp.push(self.multi());
                left = temp;
            } else {
                self.lexer.push(tk);
                break;
            }
        }
        left
    }

    fn multi(&mut self) -> Node {
        let mut left: Node = self.unary();
        while let Some(tk) = self.lexer.next() {
            if tk.value=="*" || tk.value=="/" || tk.value=="%" {
                let mut temp = Node::new(tk);
                temp.push(left.clone());
                temp.push(self.unary());
                left = temp;
            } else {
                self.lexer.push(tk);
                break;
            }
        }
        left
    }

    fn unary(&mut self) -> Node {
        if let Some(tk) = self.lexer.next() {
            match tk.typing {
                TokenType::UnaryOper => {
                    let mut this: Node = Node::new(tk);
                    this.push(self.unary());
                    this
                },
                _ => {
                    self.lexer.push(tk);
                    self.primary()
                },
            }
        } else {
            println!("Error: {:?}", TokenType::Null);
            exit(1);
        }
    }

    fn primary(&mut self) -> Node {
        if let Some(tk) = self.lexer.next() {
            match tk.typing {
                TokenType::Identifier |
                TokenType::Numerical => Node::new(tk),
                TokenType::OpenParen => {
                    let node = self.parse();
                    self.lexer.next();
                    node
                },
                TokenType::FuncCall => {
                    let mut node = Node::new(tk);
                    self.lexer.next();
                    while let Some(tk) = self.lexer.next() {
                        if tk.typing == TokenType::CloseParen {
                            return node;
                        }
                        if tk.typing == TokenType::ParamSplit {
                            continue;
                        }
                        self.lexer.push(tk);
                        node.push(self.parse());
                    }
                    println!("Expect: {:?} or {:?}", TokenType::CloseParen, TokenType::ParamSplit);
                    exit(1);
                }
                _ => {
                    println!("Unexpected Error");
                    exit(1)
                },
            }
        } else {
            println!("Error: {:?}", TokenType::Null);
            exit(1);
        }
    }
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
        } else if c=='*' || c=='/' || c=='>' || c=='<' || c=='%'{
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
    result
}


impl fmt::Display for Lexer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for each in self.data.iter().rev() {
            writeln!(f, "{:?}\t{}", each.typing, each.value)?
        }
        Ok(())
    }
}


impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.typing!=TokenType::Identifier && self.typing!=TokenType::Numerical {
            write!(f, "( ")?;
        }
        
        write!(f, "{} ", self.value)?;
        for each in self.branch.iter() {
            write!(f, "{}", each)?
        }

        if self.typing!=TokenType::Identifier && self.typing!=TokenType::Numerical {
            write!(f, ") ")?;
        }
        Ok(())
    }
}