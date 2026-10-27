mod parser;
mod error;
mod tokenizer;
mod transpiler;
use std::fs;

fn main() {
    let src = "
    int func(int a, int b, int c, int d)
    {
        return  a + (b + c + d ); 
    }

    int main()
    {
        return func(1, 1 + 1 + 3, 1, func(1, 1, 1, 1)); 
    }
    ";

    let tokens = tokenizer::tokenize(src).unwrap();
    for token in &tokens {
        break;
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
