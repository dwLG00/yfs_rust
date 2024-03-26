mod tokenizer;
mod parser;

fn main() {
    let string: String = "term1 : term2 | term3, term4 | 'literal';".to_string();
    match tokenizer::tokenize(string) {
        Some((tokens, term_table)) => {
            println!("{:?}", tokens);
            let rootnode = parser::parse(tokens);
            println!("{:?}", rootnode);
            println!("{:?}", term_table);
        },
        None => {}
    }
}
