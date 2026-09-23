# Lab7-TeoCompu

# Problema 1 (como usarlo)
Simplificador de gramáticas en Rust

## Ejecutar

```bash
cargo run -- gramaticas/gramatica_1.txt
cargo run -- gramaticas/gramatica_2.txt
```

## Formato de cada línea

`S -> aA | b | ε` (también acepta la flecha Unicode `→`).

- Izquierda: una letra mayúscula.
- Derecha: alternativas separadas por `|`.
- Símbolos permitidos: `A-Z`, `a-z`, `0-9` y `ε` como alternativa completa.

Si una línea no cumple el formato, el programa se detiene e informa el número de línea.

El algoritmo encuentra primero los no terminales anulables. Luego, para cada producción, construye las combinaciones posibles de omitir esos no terminales. Las producciones ε se descartan del resultado. Si el símbolo inicial era anulable, se avisa que la palabra vacía ya no se conserva; esta decisión cumple literalmente con dejar cero producciones ε.

#Video 
LINK
