use crate::grammar::Grammar;
use std::collections::BTreeSet;

pub struct SimplificationResult { pub grammar: Grammar }

pub fn remove_epsilon_productions(grammar: &Grammar) -> SimplificationResult {
    let nullable = find_nullable(grammar);
    println!("Paso 1: no terminales anulables = {{{}}}", nullable.iter().collect::<String>());
    println!("Paso 2: se generan todas las combinaciones al omitir símbolos anulables.");

    let mut result = Grammar::new(grammar.start);
    for (&left, rights) in &grammar.productions {
        for right in rights {
            if right.is_empty() {
                println!("  Se elimina {left} → ε");
                continue;
            }
            let variants = variants_without_nullable(right, &nullable);
            for variant in variants {
                if variant.is_empty() {
                    println!("  Se descarta {left} → ε generado desde {left} → {right}");
                } else {
                    println!("  {left} → {right}  genera  {left} → {variant}");
                    result.add(left, variant);
                }
            }
        }
    }
    if nullable.contains(&grammar.start) {
        println!("Nota: el símbolo inicial era anulable; al pedir cero producciones ε, la cadena vacía deja de pertenecer al lenguaje.");
    }
    SimplificationResult { grammar: result }
}

fn find_nullable(grammar: &Grammar) -> BTreeSet<char> {
    let mut nullable = BTreeSet::new();
    loop {
        let before = nullable.len();
        for (&left, rights) in &grammar.productions {
            if rights.iter().any(|right| right.is_empty() || right.chars().all(|c| c.is_ascii_uppercase() && nullable.contains(&c))) {
                nullable.insert(left);
            }
        }
        if nullable.len() == before { return nullable; }
    }
}

fn variants_without_nullable(right: &str, nullable: &BTreeSet<char>) -> BTreeSet<String> {
    let mut variants = BTreeSet::from([String::new()]);
    for symbol in right.chars() {
        let mut next = BTreeSet::new();
        for prefix in &variants {
            let mut kept = prefix.clone();
            kept.push(symbol);
            next.insert(kept);
            if symbol.is_ascii_uppercase() && nullable.contains(&symbol) {
                next.insert(prefix.clone());
            }
        }
        variants = next;
    }
    variants
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn removes_epsilon_and_creates_variants() {
        let mut g = Grammar::new('S');
        g.add('S', "ABa".into()); g.add('A', String::new()); g.add('B', String::new());
        let r = remove_epsilon_productions(&g).grammar;
        assert_eq!(r.productions[&'S'], BTreeSet::from(["ABa".into(), "Aa".into(), "Ba".into(), "a".into()]));
    }
}
