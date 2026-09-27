# Lab7-TeoCompu
Hecho por Giancarlo Sagastume 
24278

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
https://youtu.be/Qr8nOjNcgTc



# Problema 2 (En cuaderno)

<img width="688" height="996" alt="image" src="https://github.com/user-attachments/assets/422ed9eb-6889-4053-b58e-37498a103a15" />

<img width="745" height="987" alt="image" src="https://github.com/user-attachments/assets/92b851e1-b388-4663-bc6b-2626bf32652b" />

<img width="742" height="991" alt="image" src="https://github.com/user-attachments/assets/5d4d3dc5-4c9f-47a9-aba2-fd73107f540a" />

<img width="748" height="916" alt="image" src="https://github.com/user-attachments/assets/4b39786e-c7c0-433e-9826-be93dd186398" />
