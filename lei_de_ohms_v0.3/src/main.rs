





const MAX_TENTATIVAS: i32 = 3;


fn chamada (mensagem: &str) -> f64 {
    let mut tentativas = 0;

    loop{
        if tentativas >= MAX_TENTATIVAS {
            println!("Limite de Tentativas Excedido!
                Aplicando Valor Padrão de 0.0");
            return 0.0;
        }
        println!("{}", mensagem); // Aqui está a mágica... exatamente nesse println! antes de pegar os dados.
        let mut entrada = String::new();
        std::io::stdin()
            .read_line(&mut entrada)
            .expect("Falha ao ler Entrada");

        match entrada.trim().parse() {
            Ok(entrada) => return entrada,
            Err(_) => {
                println!("Erro ao Converter o valor, caso seja um número inteiro colocar o .0 no final... Exemplo: 3.0");
                tentativas +=1;
            }
        }
    }
}

fn main(){
    println!("Digite 1 para o descobrir valor de Tensão(Volts)");
    println!("Digite 2 para descobrir o valor da Corrente(A)");
    println!("Digite 3 para descobrir o valor da Resistência(Ohm)");

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Falha ao ler entrada");
    let n_input: u32 = input.trim().parse().expect("Entrada Inválida");

    match n_input {
        1 => {
            let i = chamada("Informe o valor da Corrente");
            
            println!("O Valor da Corrente está em MiliAmperes?
                    Se sim digite 'S' senão, pressione qualquer tecla.");
            let  mut conversao = String::new();
            std::io::stdin().read_line(&mut conversao)
            .expect("Falha");            
            let conversao_i = conversao
            .trim()
            .to_uppercase();
            
            let mut i = i;
            if conversao_i == "S" {
                i /= 1000.0;
            }
            
            let r = chamada("Informe o valor da Resistência(em Ohm's).");
            let resultado = i*r;
            println!("O valor da tensão é de: {} Volts", resultado);
        }

        2 => {
            let v = chamada("Informe o valor da Tensão(em Volts).");
            let r = chamada("Informe o valor da Resistência(em Ohm's).");
            let resultado = v / r;
            let resultado2 = resultado * 1000.0;
            println!("O Valor da Corrente é de {} Amperes, ou {} miliAmperes.", resultado, resultado2);
        }

        3 => {
            let v = chamada("Informe o valor da Tensão(em Volts).");
            let i = chamada("Informe o valor da Corrente.");

            println!("O Valor da Corrente está em MiliAmperes?
            Se sim digite 'S' senão, pressione qualquer tecla.");
            
            let mut conversao = String::new();
            std::io::stdin().read_line(&mut conversao).expect("Falha ao ler Entrada de Dados");
            let conversao_i = conversao
            .trim()
            .to_uppercase();

            let mut i = i;
            if conversao_i == "S" {
                i /= 1000.0;
            }

            let resultado = v / i;
            println!("O valor da Resistência é de {} Ohm's.", resultado);
        }
        
        _ => println!("Opção Inválida"),
    }

}










