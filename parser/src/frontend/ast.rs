#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    Identifier,
    BinaryOper,
    UnaryOper,
    OpenParen,
    CloseParen,
    FuncCall,
    ParamSplit,
    Numerical,
    StringVar,
    Null,
}


#[derive(Debug, Clone)]
pub struct Token {
    pub typing: TokenType,
    pub value: String,
    pub loc: (usize, usize),
}


pub struct Lexer {
    pub line: String,
    pub data: Vec<Token>,
}


#[derive(Debug, Clone)]
pub struct Node {
    pub typing: TokenType,
    pub value: String,
    pub branch: Vec<Node>,
}


pub struct Parser {
    pub lexer: Lexer,
}