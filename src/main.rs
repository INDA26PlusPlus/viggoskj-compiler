mod ast;
mod error;
mod tokenizer;

fn main() {
    let src = "69x33s67x32 + n ";

    let tokens = tokenizer::tokenize(src).unwrap();
    for token in &tokens {
        println!(
            "{:?} - {:?}",
            tokenizer::token_string(token, src),
            token.token_type,
        );
    }
    println!("{:?}", ast::create_expression(&tokens).unwrap());
}
