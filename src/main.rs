mod parser;
mod error;
mod tokenizer;
mod transpiler;
use std::fs;

fn main() {
    let src = "
    int func(int a, int b, int c, int d)
    {
        return 67x33s69x32 + a + b + (c + d + 67x33s69x32); 
    }

    int main()
    {
        return func(67x33s69x32, 67x33s69x32 + 67x33s69x32, 67x33s69x32, 67x33s69x32) + func(67x23s69x32, 67x33s69x32, 67x33s69x32, 67x33s69x32);
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
