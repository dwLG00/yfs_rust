mod tokenizer;
mod parser;

fn main() {
    
    let string: String = "A : C, B;
        B : C, B | '';
        C : T, ':', D, ';';
        D : F, E;
        E : '|', F, E | '';
        F : H, G;
        G : ',', H, G | '';
        H : L | T;".to_string();
    
    //let string: String = "A : B; B : C;".to_string();
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
