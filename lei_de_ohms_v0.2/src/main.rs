use std::io;

const MAX_TENTATIVAS: usize = 3;

fn ler_valor(mensagem: &str) -> f64 {
    let mut tentativas = 0;

    loop {
        if tentativas >= MAX_TENTATIVAS {
            println!("Número Máximo de Tentativas Excedido! Usando valor padrão de 0.0!");
            return 0.0;
        }

        println!("{}", mensagem);
        let mut entrada = String::new();
        io::stdin().read_line(&mut entrada).expect("Falha ao ler entrada");

        match entrada.trim().parse() {
            Ok(valor) => return valor,
            Err(_) => {
                println!("Erro ao converter o valor, caso seja um número inteiro colocar .0 no final... Exemplo: 3.0");
                tentativas += 1;
            }
        }
    }
}

fn main() {
    println!("Para Descobrir o valor da Tensão digite 1,");
    println!("Para Descobrir o valor da Corrente digite 2,");
    println!("Para Descobrir o valor da Resistência digite 3");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Falha ao ler entrada");

    let n_input: u32 = input.trim().parse().expect("Entrada inválida");

    match n_input {
        1 => {
            let v = ler_valor("Informe a Voltagem:");
            let i = ler_valor("Informe o valor da corrente:");
            let resultado = v / i;
            println!("O Valor da Tensão é: {}", resultado);
        }
        2 => {
            let i = ler_valor("Informe o valor da corrente:");
            let r = ler_valor("Informe o valor da resistência:");
            let resultado = i * r;
            println!("O Valor da Corrente é: {}", resultado);
        }
        3 => {
            let v = ler_valor("Informe a Voltagem:");
            let i = ler_valor("Informe o valor da corrente:");
            let resultado = v / i;
            println!("O Valor da Resistência é: {}", resultado);
        }
        _ => println!("Opção inválida"),
    }
}


