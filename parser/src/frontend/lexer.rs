use crate::frontend::ast;
use std::fmt;

impl ast::Token {
    fn new(t: ast::TokenType) -> Self {
        Self {
            typing: t,
            value: String::new(),
        }
    }

    fn push(&mut self, c: char) {
        self.value.push(c);
    }

    fn update(&mut self, t: ast::TokenType) {
        self.typing = t;
    }

    fn null() -> Self {
        ast::Token::new(ast::TokenType::Null)
    }
}


impl ast::Lexer {
    pub fn new(line: String) -> Self {
        let mut result: Vec<ast::Token> = tokenize(line);
        result.reverse();
        result.pop();
        Self {data: result}
    }

    pub fn next(&mut self) -> Option<ast::Token> {
        self.data.pop()
    }

    pub fn push(&mut self, tk: ast::Token) {
        self.data.push(tk);
    }
}


impl fmt::Display for ast::Lexer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for each in self.data.iter().rev() {
            writeln!(f, "{:?}\t{}", each.typing, each.value)?
        }
        Ok(())
    }
}


fn tokenize(line: String) -> Vec<ast::Token> {
    let mut result: Vec<ast::Token> = vec![ast::Token::null()];
    for c in line.chars() {
        // this line will never panic since `result` is guaranteed to have at least one element
        let prev: &mut ast::Token = result.last_mut().expect("authored-by-FelysNeko");

        if c.is_ascii_alphabetic() {
            match prev.typing {
                ast::TokenType::Identifier => prev.push(c),
                _ => {
                    let mut new: ast::Token = ast::Token::new(ast::TokenType::Identifier);
                    new.push(c);
                    result.push(new);
                }
            }
        } else if c.is_ascii_digit() || c=='.' {
            match prev.typing {
                ast::TokenType::Identifier | 
                ast::TokenType::Numerical => prev.push(c),
                _ => {
                    let mut new: ast::Token = ast::Token::new(ast::TokenType::Numerical);
                    new.push(c);
                    result.push(new);
                }
            }
        } else if c == '=' {
            if prev.value==">" || prev.value=="=" || prev.value=="<" || prev.value=="!"{
                prev.update(ast::TokenType::BinaryOper);
                prev.push(c);
            } else {
                let mut new: ast::Token = ast::Token::new(ast::TokenType::BinaryOper);
                new.push(c);
                result.push(new);
            }
        } else if c=='!' || c=='|' || c=='&' || c=='^' || c=='~'{
            if prev.value == String::from(c) {
                prev.update(ast::TokenType::BinaryOper);
                prev.push(c);
            } else {
                let mut new: ast::Token = ast::Token::new(ast::TokenType::UnaryOper);
                new.push(c);
                result.push(new);
            }
        } else if c=='+' || c=='-' {
            let mut new: ast::Token = match prev.typing {
                ast::TokenType::UnaryOper |
                ast::TokenType::BinaryOper |
                ast::TokenType::OpenParen |
                ast::TokenType::Null => ast::Token::new(ast::TokenType::UnaryOper),
                _ => ast::Token::new(ast::TokenType::BinaryOper),
            };
            new.push(c);
            result.push(new);
        } else if c=='*' || c=='/' || c=='>' || c=='<' || c=='%'{
            let mut new: ast::Token = ast::Token::new(ast::TokenType::BinaryOper);
            new.push(c);
            result.push(new);
        } else if c == '(' {
            if prev.typing == ast::TokenType::Identifier {
                prev.update(ast::TokenType::FuncCall);
            }
            let mut new: ast::Token =ast::Token::new(ast::TokenType::OpenParen);
            new.push(c);
            result.push(new);
        } else if c == ')' {
            let mut new: ast::Token =ast::Token::new(ast::TokenType::CloseParen);
            new.push(c);
            result.push(new);
        } else if c == ',' {
            let mut new: ast::Token =ast::Token::new(ast::TokenType::ParamSplit);
            new.push(c);
            result.push(new);
        }
    }
    result
}