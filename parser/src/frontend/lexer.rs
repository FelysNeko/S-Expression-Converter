use crate::frontend::ast;
use std::fmt;

impl ast::Token {
    fn new(t: ast::TokenType, s:usize) -> Self {
        Self {
            typing: t,
            value: String::new(),
            loc: (s, s)
        }
    }

    fn push(&mut self, c: char) {
        self.value.push(c);
        self.loc.1 += 1;
    }

    fn update(&mut self, t: ast::TokenType) {
        self.typing = t;
    }

    pub fn null() -> Self {
        ast::Token::new(ast::TokenType::Null, 0)
    }
}


impl ast::Lexer {
    pub fn new(line: String) -> Self {
        let mut result: Vec<ast::Token> = tokenize(&line);
        result.reverse();
        result.pop();
        Self {line, data: result}
    }

    pub fn next(&mut self) -> Option<ast::Token> {
        self.data.pop()
    }

    pub fn push(&mut self, tk: ast::Token) {
        self.data.push(tk);
    }
}


impl fmt::Display for ast::Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}\t{}\t{}\t{}", self.typing, self.loc.0, self.loc.1, self.value)
    }
}


impl fmt::Display for ast::Lexer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for each in self.data.iter().rev() {
            writeln!(f, "{}", each)?
        }
        Ok(())
    }
}


fn tokenize(line: &String) -> Vec<ast::Token> {
    let mut result: Vec<ast::Token> = vec![ast::Token::null()];
    for (i, c) in line.chars().enumerate() {
        // this line will never panic since `result` is guaranteed to have at least one element
        let prev: &mut ast::Token = result.last_mut().expect("authored-by-FelysNeko");

        if 
            prev.typing == ast::TokenType::StringVar && 
            (prev.value.len()==1 || !prev.value.ends_with('\"'))
        {
            prev.push(c);
        } else if c.is_ascii_alphabetic() {
            match prev.typing {
                ast::TokenType::Identifier => prev.push(c),
                _ => {
                    let mut new: ast::Token = ast::Token::new(ast::TokenType::Identifier, i);
                    new.push(c);
                    result.push(new);
                }
            }
        } else if c.is_ascii_digit() || c=='.' {
            match prev.typing {
                ast::TokenType::Identifier | 
                ast::TokenType::Numerical => prev.push(c),
                _ => {
                    let mut new: ast::Token = ast::Token::new(ast::TokenType::Numerical, i);
                    new.push(c);
                    result.push(new);
                }
            }
        } else if c == '=' {
            if prev.value==">" || prev.value=="=" || prev.value=="<" || prev.value=="!"{
                prev.update(ast::TokenType::BinaryOper);
                prev.push(c);
            } else {
                let mut new: ast::Token = ast::Token::new(ast::TokenType::BinaryOper, i);
                new.push(c);
                result.push(new);
            }
        } else if c=='|' || c=='&'{
            if prev.value == String::from(c) {
                prev.update(ast::TokenType::BinaryOper);
                prev.push(c);
            } else {
                let mut new: ast::Token = ast::Token::new(ast::TokenType::UnaryOper, i);
                new.push(c);
                result.push(new);
            }
        } else if c=='+' || c=='-' {
            let mut new: ast::Token = match prev.typing {
                ast::TokenType::UnaryOper |
                ast::TokenType::BinaryOper |
                ast::TokenType::OpenParen |
                ast::TokenType::Null => ast::Token::new(ast::TokenType::UnaryOper, i),
                _ => ast::Token::new(ast::TokenType::BinaryOper, i),
            };
            new.push(c);
            result.push(new);
        } else if c == '\"' {
            let mut new: ast::Token = ast::Token::new(ast::TokenType::StringVar, i);
            new.push(c);
            result.push(new);           
        } else if c=='*' || c=='/' || c=='>' || c=='<' || c=='%'{
            let mut new: ast::Token = ast::Token::new(ast::TokenType::BinaryOper, i);
            new.push(c);
            result.push(new);
        } else if c == '(' {
            if prev.typing == ast::TokenType::Identifier {
                prev.update(ast::TokenType::FuncCall);
            }
            let mut new: ast::Token =ast::Token::new(ast::TokenType::OpenParen, i);
            new.push(c);
            result.push(new);
        } else if c == ')' {
            let mut new: ast::Token =ast::Token::new(ast::TokenType::CloseParen, i);
            new.push(c);
            result.push(new);
        } else if c == ',' {
            let mut new: ast::Token =ast::Token::new(ast::TokenType::ParamSplit, i);
            new.push(c);
            result.push(new);
        } else if c=='!' || c=='~' || c=='^'{
            let mut new: ast::Token = ast::Token::new(ast::TokenType::UnaryOper, i);
            new.push(c);
            result.push(new);
        }
    }
    result
}