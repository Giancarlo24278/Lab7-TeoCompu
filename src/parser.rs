use crate::grammar::Grammar;
use regex::Regex;
use std::{fs, path::Path};

// Acepta: S -> aA | b | ε     o     S → aA|b|ε
// Los cuerpos solo contienen letras individuales (A-Z, a-z), dígitos o ε.
pub fn load_grammar(path: impl AsRef<Path>) -> Result<Grammar, String> {
    let content = fs::read_to_string(path.as_ref())
        .map_err(|e| format!("no se pudo leer '{}': {e}", path.as_ref().display()))?;
    let line_re = Regex::new(r"^\s*([A-Z])\s*(?:->|→)\s*((?:[A-Za-z0-9]+|ε)(?:\s*\|\s*(?:[A-Za-z0-9]+|ε))*)\s*$").unwrap();
    let mut grammar: Option<Grammar> = None;

    for (index, raw) in content.lines().enumerate() {
        if raw.trim().is_empty() { continue; }
        let captures = line_re.captures(raw).ok_or_else(|| {
            format!("línea {} inválida: '{raw}'. Formato: S -> aA | b | ε", index + 1)
        })?;
        let left = captures[1].chars().next().unwrap();
        let right_text = &captures[2];
        let g = grammar.get_or_insert_with(|| Grammar::new(left));
        for alternative in right_text.split('|') {
            let alternative = alternative.trim();
            g.add(left, if alternative == "ε" { String::new() } else { alternative.to_string() });
        }
    }
    grammar.ok_or_else(|| "el archivo no contiene producciones".to_string())
}
