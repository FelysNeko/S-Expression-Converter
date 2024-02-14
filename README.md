# S-Expression-Converter
CLI program that convert expression to s-expression

### Compilation
Make sure you can run `cargo` on your device. Navigate to where `Cargo.toml` locates and run `cargo build --release` in terminal. Rust production level compilation might take a few seconds, and then you should find a binary file called `parser` in `./target/release`. This is the CLI programm that you will be using. 

### Execution
Like ohter CLI program, you can run `./parser -h` for instructions. The acutal usage is `./parser '<INPUT>'`, and `-d` flag can switch on debug mode for more information. Make sure you wrap the expression with single quatation marks. The program covers most of operators, except for `->` `++` `+=` `>>` `sizeof` and a few more. Some dialectal operators like `and` `or` `**` `=>` are also excluded.

### Example
This parses and converts `x=1+foo("elysia", y, 2)` into `( = x ( + 1 ( foo "elysia" y 2 ) ) )`, and prints out all info.
```
./parser 'x=1+foo("elysia", y, 2)' -d
```

```
Debug: true
Expr: "x=1+foo(\"elysia\", y, 2)"

TokenType       Start   End     Value
Identifier      0       1       x
BinaryOper      1       2       =
Numerical       2       3       1
BinaryOper      3       4       +
FuncCall        4       7       foo
OpenParen       7       8       (
StringVar       8       16      "elysia"
ParamSplit      16      17      ,
Identifier      18      19      y
ParamSplit      19      20      ,
Numerical       21      22      2
CloseParen      22      23      )

Node {
    typing: BinaryOper,
    value: "=",
    branch: [
        Node {
            typing: Identifier,
            value: "x",
            branch: [],
        },
        Node {
            typing: BinaryOper,
            value: "+",
            branch: [
                Node {
                    typing: Numerical,
                    value: "1",
                    branch: [],
                },
                Node {
                    typing: FuncCall,
                    value: "foo",
                    branch: [
                        Node {
                            typing: StringVar,
                            value: "\"elysia\"",
                            branch: [],
                        },
                        Node {
                            typing: Identifier,
                            value: "y",
                            branch: [],
                        },
                        Node {
                            typing: Numerical,
                            value: "2",
                            branch: [],
                        },
                    ],
                },
            ],
        },
    ],
}

( = x ( + 1 ( foo "elysia" y 2 ) ) )
```

### Additional Information
This is my first Rust program. It converts normal expression into Racket style code, AKA s-expression. Now profs can use this tool to come up with some extremely complex Racket code for you evaluate LOL. This project is also the prerequisite of [Felys-Interpreter](https://github.com/FelysNeko/Felys-Interpreter) future version since it covers topics like tokenizer and parser.
