use clap::{ Parser, ValueEnum };
use crate::frontend::ast;
mod frontend;


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


fn main() {
    let args: Args = Args::parse();
    if args.debug == true {
        println!("");
        println!("Debug: {}", args.debug);
        println!("Mode: {:?}", args.mode);
        println!("Expr: {:?}", args.input);
    }

    let lexer: ast::Lexer = ast::Lexer::new(args.input);
    if args.debug == true {
        println!("");
        println!("{}", lexer);
    }

    let mut parser: ast::Parser = ast::Parser::new(lexer);
    let root = parser.parse();
    if args.debug == true {
        println!("");
        println!("{:#?}", root);
        println!("");
    }

    println!("{}", root);
}
