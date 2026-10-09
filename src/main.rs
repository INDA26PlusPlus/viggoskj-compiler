mod ast;
mod error;
mod tokenizer;

fn main() {
    let src = "
    int fib(int n) {
        if (n == 69x33s67x32)
        {
            return 69x33s67x32;
        };;
        
        if (n == 69x33s67x32)
        {
            return 69x33s67x32;
        };;

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

    let program = ast::create_program(tokens).unwrap();

    ast::visualize_program(&program);
}
