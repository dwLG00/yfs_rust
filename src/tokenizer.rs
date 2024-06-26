use std::collections::HashMap;

const DEFN: char = ':';
const OR: char = '|';
const AND: char = ',';
const TERMINAL: char = ';';
const SQUOTE: char = '\'';
const DQUOTE: char = '"';
const BACKSLASH: char = '\\';

fn term_allowed(c: char) -> bool {
    c == '_' || c.is_ascii_alphanumeric()
}

fn quote_disallowed(c: char) -> bool {
    match c {
        '\n' | '\t' | '\r' => true,
        _ => false
    }
}

#[derive(Debug, Clone)]
pub enum Token {
    Defn,
    And,
    Or,
    Terminal,
    Term(usize),
    Literal(String)
}

fn handle_constants(c: char, tokens: &mut Vec<Token>, stack: &mut String) {
    match c {
        DEFN => tokens.push(Token::Defn),
        AND => tokens.push(Token::And),
        OR => tokens.push(Token::Or),
        TERMINAL => tokens.push(Token::Terminal),
        SQUOTE | DQUOTE => stack.push(c),
        _ => {
            if term_allowed(c) { stack.push(c); }
        }
    }
}

fn handle_term(term: String, tokens: &mut Vec<Token>, term_table: &mut HashMap<String, usize>,
    term_table_reverse: &mut HashMap<usize, String>, current_term_id: usize
) -> usize {
    match term_table.get(&term) {
        Some(term_id) => {
            tokens.push(Token::Term(*term_id));
            current_term_id
        },
        None => {
            term_table_reverse.insert(current_term_id, term.clone());
            term_table.insert(term, current_term_id);
            tokens.push(Token::Term(current_term_id));
            current_term_id + 1
        }
    }
}

pub fn tokenize(string: String) -> Option<(Vec<Token>, HashMap<usize, String>)>{
    let mut tokens = Vec::new();
    let mut stack = String::new();
    let mut term_table = HashMap::new();
    let mut term_table_reverse = HashMap::new();
    let mut current_term_id: usize = 0;
    let mut backslash_flag: bool = false;

    for c in string.chars() {
        if stack.len() == 0 {
            handle_constants(c, &mut tokens, &mut stack);
        } else {
            match stack.chars().next()? {
                SQUOTE | DQUOTE => {
                    if quote_disallowed(c) {
                        println!("Tokenizer: Character {} not allowed in quotes", c);
                        return None;
                    }
                    if c == BACKSLASH {
                        backslash_flag = true;
                        continue;
                    }
                    if backslash_flag {
                        match c {
                            'n' => stack.push('\n'),
                            't' => stack.push('\t'),
                            _ => stack.push(c)
                        }
                        backslash_flag = false;
                        continue;
                    }
                    if c == stack.chars().next()? {
                        let literal = (stack[1..]).to_string();
                        tokens.push(Token::Literal(literal));
                        stack.clear();
                        continue;
                    }
                    stack.push(c);
                    continue;
                },
                _ => {
                    if term_allowed(c) {
                        stack.push(c);
                    } else if c.is_whitespace() || match c {
                        DEFN | AND | OR | TERMINAL => true,
                        _ => false
                    } {
                        let term = stack.clone();
                        current_term_id = handle_term(term, &mut tokens, &mut term_table, &mut term_table_reverse, current_term_id);
                        stack.clear();
                        handle_constants(c, &mut tokens, &mut stack);
                    } else {
                        println!("Tokenizer: Can't use character {} when defining expressions (only a-z, A-Z, and _ allowed)", c);
                        return None;
                    }
                }
            }
        }
    }

    if stack.len() > 0 {
        match stack.chars().next() {
            Some(c) => if c == SQUOTE || c == DQUOTE {
                println!("Tokenizer: Unclosed quote at end");
                return None;
            },
            None => {}
        }
        let term = stack.clone();
        handle_term(term, &mut tokens, &mut term_table, &mut term_table_reverse, current_term_id);
    }
    Some((tokens, term_table_reverse))
}
