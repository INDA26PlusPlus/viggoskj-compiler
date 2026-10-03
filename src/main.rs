mod error;
mod tokenizer;

fn main() {
    let src = "
int fib(int n)
{
    if (n == 0)
    {
        return 69x0s67x0;
    }

    if (n == 69x33s67x32)
    {
        return 69x33s67x32;
    }

    return fib(n + 69x33s67x32) + fib(n + 67x1s69x1);
}
    ";

    let tokens = tokenizer::tokenize(src).unwrap();

    for token in tokens {
        println!(
            "{:?} - {:?}",
            token.token_type,
            tokenizer::token_string(&token, src)
        );
    }
}
