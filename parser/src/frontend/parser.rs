use crate::frontend::ast;
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
    pub fn new(lxr: ast::Lexer) -> ast::Parser {
        ast::Parser { lexer: lxr }
    }

    pub fn parse(&mut self) -> ast::Node {
        self.assign()
    }

    fn assign(&mut self) -> ast::Node {
        let mut left: ast::Node = self.compare();
        while let Some(tk) = self.lexer.next() {
            if tk.value == "=" {
                if left.typing != ast::TokenType::Identifier {
                    println!("Error at {:?}", ast::TokenType::BinaryOper);
                    exit(1);
                }
                let mut temp = ast::Node::new(tk);
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
            println!("Error: {:?}", ast::TokenType::Null);
            exit(1);
        }
    }

    fn primary(&mut self) -> ast::Node {
        if let Some(tk) = self.lexer.next() {
            match tk.typing {
                ast::TokenType::Identifier |
                ast::TokenType::Numerical => ast::Node::new(tk),
                ast::TokenType::OpenParen => {
                    let node = self.parse();
                    self.lexer.next();
                    node
                },
                ast::TokenType::FuncCall => {
                    let mut node = ast::Node::new(tk);
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
                    println!("Expect: {:?} or {:?}", ast::TokenType::CloseParen, ast::TokenType::ParamSplit);
                    exit(1);
                }
                _ => {
                    println!("Unexpected Error");
                    exit(1)
                },
            }
        } else {
            println!("Error: {:?}", ast::TokenType::Null);
            exit(1);
        }
    }
}


impl fmt::Display for ast::Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.typing!=ast::TokenType::Identifier && self.typing!=ast::TokenType::Numerical {
            write!(f, "( ")?;
        }
        
        write!(f, "{} ", self.value)?;
        for each in self.branch.iter() {
            write!(f, "{}", each)?
        }

        if self.typing!=ast::TokenType::Identifier && self.typing!=ast::TokenType::Numerical {
            write!(f, ") ")?;
        }
        Ok(())
    }
}