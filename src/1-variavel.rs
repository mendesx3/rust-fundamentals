fn main() {
    // Variáveis Imutáveis:
    let nome_da_variavel = "valor";
    println!("nome_da_variavel: {}", nome_da_variavel);

    // Variáveis Mutáveis:
    let mut contador = 0;
    contador = 2;
    println!("contador: {}", contador);

    // Tipos Numéricos:
    let inteiro: i32 = 42;
    let ponto_flutuante: f64 = 3.14;
    println!("inteiro: {}, ponto_flutuante: {}", inteiro, ponto_flutuante);

    // tipo nao pode ser negativo
    let numero_x: u16 = 1;
    println!("numero nao pode ser negativo {}", numero_x);

    // Booleanos:
    let verdadeiro = true;
    let falso: bool = false;
    println!("verdadeiro: {}, falso: {}", verdadeiro, falso);

    // Caracteres:
    let caractere = 'a';
    println!("caractere: {}", caractere);

    // string
    let texto = "Rust!";
    println!("texto: {}", texto);
}
