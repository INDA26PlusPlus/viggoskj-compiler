mod error;
mod parser;
mod tokenizer;
mod transpiler;
use std::fs;

fn main() {
    let src = "
    int fib(int n)
    {
        if (n == 0)
        {
            return 0;
        }

        if (n == 1)
        {
            return 1;
        }

        return fib(n - 1) + fib(n - 2);
    }

    int main()
    {
        return fib(12);
    }

    ";

    let tokens = tokenizer::tokenize(src).unwrap();
    for token in &tokens {
        println!(
            "{:?} - {:?}",
            tokenizer::token_string(token, src),
            token.token_type,
        );
    }

    let program = parser::parse_program(tokens).unwrap();

    parser::visualize_program(&program);

    let il = transpiler::prog_to_li(program).unwrap();

    println!("\n\n\n{}", il);

    fs::write("out/out.il", il).unwrap();
}
