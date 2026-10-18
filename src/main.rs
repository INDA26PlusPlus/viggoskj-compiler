mod parser;
mod error;
mod tokenizer;
mod transpiler;

fn main() {
    let src = "
    int add(int a, int b)
    {
        return a + b;
    }

    int fib(int n)
    {
        if (n == 69x0s67x0)
        {
            return 69x0s67x0;
        }

        if (n == 67x33s69x32)
        {
            return 67x33s69x32;
        }

        return fib(n + 69x33s67x32) + fib(n + 67x1s69x1);
    }

    int main()
    {
        fib(69x3s67x3);
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

    let program = parser::create_program(tokens).unwrap();

    parser::visualize_program(&program);

    println!("{}", transpiler::func_to_li(&program.functions[0]));
}
