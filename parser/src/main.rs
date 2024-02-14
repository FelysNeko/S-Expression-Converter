use clap::Parser;
use crate::frontend::ast;
mod frontend;


#[derive(Parser)]
#[command(about="S-Expression Converter")]
struct Args {
    #[arg(short, long, help="show debug information")]
    debug: bool,

    #[arg(help="expression or file path depends on your mode")]
    input: String,
}


fn main() {
    let args: Args = Args::parse();
    if args.debug == true {
        println!("");
        println!("Debug: {}", args.debug);
        println!("Expr: {:?}", args.input);
    }

    let lexer: ast::Lexer = ast::Lexer::new(args.input);
    if args.debug == true {
        println!("");
        println!("TokenType\tStart\tEnd\tValue");
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
