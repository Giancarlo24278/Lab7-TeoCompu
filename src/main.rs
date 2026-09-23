mod grammar;
mod parser;
mod simplify;

use std::{env, process};

fn main() {
    let path = env::args().nth(1).unwrap_or_else(|| {
        eprintln!("Uso: cargo run -- <archivo_gramatica.txt>");
        process::exit(1);
    });

    let grammar = parser::load_grammar(&path).unwrap_or_else(|error| {
        eprintln!("Error al cargar la gramática: {error}");
        process::exit(1);
    });

    println!("Gramática original:\n{}", grammar);
    let simplified = simplify::remove_epsilon_productions(&grammar);
    println!("\nGramática sin producciones ε:\n{}", simplified.grammar);
}
