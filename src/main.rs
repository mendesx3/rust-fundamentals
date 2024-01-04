struct Pessoa {
    nome: String,
    idade: u32,
}

fn main() {
    let pessoa = Pessoa {
        nome: String::from("João"),
        idade: 15,
    };

    if pessoa.idade > 16 {
        println!("maior")
    } else if pessoa.idade >= 16 && pessoa.idade < 18 {
        println!("{}, nao obrigatorio",pessoa.nome)
    } else {
        println!("{} obrigatorio", pessoa.nome)
    }

    let numero_str = "42";
    match convert_to_int(numero_str) {
        Ok(numero) => println!("A conversão foi bem-sucedida! Resultado: {}", numero),
        Err(erro) => println!("Erro ao converter para inteiro: {:?}", erro),
    }

    let texto_invalido = "abc";
    match convert_to_int(texto_invalido) {
        Ok(numero) => println!("A conversão foi bem-sucedida! Resultado: {}", numero),
        Err(erro) => println!("Erro ao converter para inteiro: {:?}", erro),
    }
}

fn convert_to_int(s: &str) -> Result<i32, std::num::ParseIntError> {
    // Utiliza o método parse() para tentar converter a string para um número inteiro
    let resultado = s.parse::<i32>();

    // Retorna o resultado da conversão
    resultado
}