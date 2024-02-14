use crate::frontend::ast;
use colored::Colorize;
use std::process::exit;
use std::fmt;

impl ast::Node {
    fn new(tk: ast::Token) -> Self {
        Self {
            typing: tk.typing,
            value: tk.value,
            branch: Vec::new(),
        }
    }

    fn push(&mut self, n: ast::Node) {
        self.branch.push(n);
    }
}


impl ast::Parser {
    pub fn new(lxr: ast::Lexer) -> Self {
        Self { lexer: lxr }
    }

    pub fn parse(&mut self) -> ast::Node {
        self.assign()
    }

    fn warn(&self, tk: ast::Token) -> ! {
        let start: usize;
        let end: usize;

        if tk.typing == ast::TokenType::Null {
            start = self.lexer.line.len();
            end = start + 1;
        } else {
            start = tk.loc.0;
            end = tk.loc.1;
        }
        
        println!("");
        println!("{}", self.lexer.line);
        for _ in 0..start {
            print!(" ");
        }
        for _ in start..end {
            print!("{}", "^".red().bold());
        }
        println!(" {}", "ERROR".red().bold());
        println!("");

        exit(1);
    }

    fn assign(&mut self) -> ast::Node {
        let left: ast::Node = self.compare();
        if let Some(tk) = self.lexer.next() {
            if tk.value == "=" {
                if left.typing != ast::TokenType::Identifier {
                    self.warn(tk);
                }
                let mut temp = ast::Node::new(tk);
                temp.push(left);
                temp.push(self.assign());
                return temp;
            }
            self.lexer.push(tk);
        }
        left
    }

    fn compare(&mut self) -> ast::Node {
        let mut left: ast::Node = self.add();
        while let Some(tk) = self.lexer.next() {
            if 
                tk.value==">" || tk.value==">=" || tk.value=="==" || 
                tk.value=="<" || tk.value=="<=" || tk.value=="!=" 
            {
                let mut temp = ast::Node::new(tk);
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

    fn add(&mut self) -> ast::Node {
        let mut left: ast::Node = self.multi();
        while let Some(tk) = self.lexer.next() {
            if tk.value=="+" || tk.value=="-" {
                let mut temp = ast::Node::new(tk);
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

    fn multi(&mut self) -> ast::Node {
        let mut left: ast::Node = self.unary();
        while let Some(tk) = self.lexer.next() {
            if tk.value=="*" || tk.value=="/" || tk.value=="%" {
                let mut temp = ast::Node::new(tk);
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

    fn unary(&mut self) -> ast::Node {
        if let Some(tk) = self.lexer.next() {
            match tk.typing {
                ast::TokenType::UnaryOper => {
                    let mut this: ast::Node = ast::Node::new(tk);
                    this.push(self.unary());
                    this
                },
                _ => {
                    self.lexer.push(tk);
                    self.primary()
                },
            }
        } else {
            self.warn(ast::Token::null());
        }
    }

    fn primary(&mut self) -> ast::Node {
        if let Some(tk) = self.lexer.next() {
            match tk.typing {
                ast::TokenType::Identifier |
                ast::TokenType::Numerical | 
                ast::TokenType::StringVar => ast::Node::new(tk),
                ast::TokenType::OpenParen => {
                    let node = self.parse();
                    self.lexer.next();
                    node
                },
                ast::TokenType::FuncCall => {
                    let mut node = ast::Node::new(tk.clone());
                    self.lexer.next();
                    while let Some(tk) = self.lexer.next() {
                        if tk.typing == ast::TokenType::CloseParen {
                            return node;
                        }
                        if tk.typing == ast::TokenType::ParamSplit {
                            continue;
                        }
                        self.lexer.push(tk);
                        node.push(self.parse());
                    }
                    self.warn(tk);
                }
                _ => {
                    self.warn(tk);
                },
            }
        } else {
            self.warn(ast::Token::null());
        }
    }
}


impl fmt::Display for ast::Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if 
            self.typing != ast::TokenType::Identifier && 
            self.typing != ast::TokenType::Numerical &&
            self.typing != ast::TokenType::StringVar
        {
            write!(f, "( ")?;
        }
        
        write!(f, "{} ", self.value)?;
        for each in self.branch.iter() {
            write!(f, "{}", each)?
        }

        if 
            self.typing != ast::TokenType::Identifier && 
            self.typing != ast::TokenType::Numerical &&
            self.typing != ast::TokenType::StringVar
        {
            write!(f, ") ")?;
        }
        Ok(())
    }
}